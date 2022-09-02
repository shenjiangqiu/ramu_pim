use std::collections::{BTreeMap, VecDeque};

use crate::{command::Command, memory::Level, request::ReqType};
#[derive(PartialEq, Eq)]
pub enum State {
    Opened(u64),
    Closed,
    PowerUp,
    ActPowerDown,
    PrePowerDown,
    SelfRefresh,
    NoUse,
}
pub struct Dram<'a, T: ?Sized> {
    pub spec: &'a T,
    pub level: Level,
    pub children: Vec<Dram<'a, T>>,
    pub state: State,
    pub next_clk: Vec<u64>,
    pub prev: Vec<VecDeque<u64>>,
}
impl<'a, T> Dram<'a, T>
where
    T: DramSpec,
{
    pub fn new(spec: &'a T, level: Level, child_size: &[usize]) -> Self {
        let state = T::get_start_state(&level);
        let child_level = level.next_level().unwrap();
        let mut children = vec![];
        if !matches!(child_level, Level::Row) {
            for _i in 0..child_size[u8::from(child_level) as usize] {
                children.push(Dram::new(
                    spec,
                    Level::try_from(child_level).unwrap(),
                    child_size,
                ));
            }
        }
        let mut prev = vec![];
        for i in 0..Command::Max as usize {
            let mut tmp = VecDeque::new();

            let mut dist = 0;
            let cmd = Command::try_from(i as u8).unwrap();
            for time_entry in spec.get_timming(&level, &cmd) {
                dist = dist.max(time_entry.dist);
            }
            tmp.resize(dist, u64::MAX);
            prev.push(tmp);
        }
        let mut next_clk = vec![0; Command::Max as usize];
        Self {
            spec,
            level,
            state,
            children,
            next_clk,
            prev,
        }
    }
    pub fn decode(&self, cmd: &Command, addr_vec: &[u64]) -> Command {
        let child_index = addr_vec[self.level as usize + 1];
        if let Some(Command) = T::get_pre_cmd(self, cmd, child_index) {
            return Command;
        } else {
            match self.level {
                Level::Bank => {
                    return *cmd;
                }
                _ => {
                    return self.children[child_index as usize].decode(cmd, addr_vec);
                }
            }
        }
    }

    pub fn get_first_cmd(&self, req_type: &ReqType) -> Command {
        T::get_first_cmd(req_type)
    }
    pub fn update(&mut self, cmd: &Command, addr_vec: &[u64], clk: u64) {
        self.update_state(cmd, addr_vec);
        self.update_timming(cmd, addr_vec, clk);
    }
    fn update_state(&mut self, cmd: &Command, addr_vec: &[u64]) {
        tracing::debug!("update_state: {:?} {:?}", self.level, cmd);
        let child_level = self.level.next_level();
        self.spec
            .update_state(self, cmd, child_level.unwrap() as u64);
        if self.level == self.spec.get_scope(cmd) || self.children.is_empty() {
            return;
        }
        tracing::debug!("update_state: {:?}", child_level);
        let child_index = addr_vec[self.level as usize + 1] as usize;
        self.children[child_index].update_state(cmd, addr_vec);
    }
    fn update_timming(&mut self, cmd: &Command, addr_vec: &[u64], clk: u64) {
        if !self.prev[*cmd as usize].is_empty() {
            self.prev[*cmd as usize].pop_back();
            self.prev[*cmd as usize].push_front(clk);
        }
        for timing in self.spec.get_timming(&self.level, cmd) {
            if timing.sibling {
                continue;
            }
            let past = self.prev[*cmd as usize][timing.dist - 1 as usize];
            if past == u64::MAX {
                continue;
            }
            let future = past + timing.val;
            self.next_clk[timing.cmd as usize] = future.max(self.next_clk[timing.cmd as usize]);
        }
        self.children.iter_mut().for_each(|child| {
            child.update_timming(cmd, addr_vec, clk);
        });
    }
    /// return if the command is ok to issue
    pub fn check(&self, cmd: &Command, addr_vec: &[u64], clk: u64) -> bool {
        if clk < self.get_next_avaliable_clk(cmd) {
            return false;
        }
        if let Some(child_level) = self.level.next_level() {
            if self.spec.get_scope(cmd) != self.level && !self.children.is_empty() {
                let child_index = addr_vec[child_level as usize + 1];
                return self.children[child_index as usize].check(cmd, addr_vec, clk);
            }
        }
        true
    }
    pub fn get_next_avaliable_clk(&self, cmd: &Command) -> u64 {
        self.next_clk[*cmd as usize]
    }
}
#[derive(Clone)]
pub struct TimeEntry {
    pub cmd: Command,
    pub dist: usize,
    pub val: u64,
    pub sibling: bool,
}
pub trait DramSpec {
    fn get_first_cmd(req_type: &ReqType) -> Command;
    fn get_pre_cmd<'a>(dram: &Dram<'a, Self>, cmd: &Command, child_id: u64) -> Option<Command>;
    fn get_start_state(level: &Level) -> State;
    fn get_addr_bits(&self) -> &[usize];
    fn get_child_size(&self) -> &[usize];
    fn get_scope(&self, cmd: &Command) -> Level;
    fn update_state(&self, dram: &mut Dram<Self>, cmd: &Command, child_id: u64);
    fn get_timming(&self, level: &Level, cmd: &Command) -> &[TimeEntry];
    fn get_read_latency(&self) -> u64;
}
