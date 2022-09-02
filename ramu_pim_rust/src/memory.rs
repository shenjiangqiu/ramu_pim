use std::collections::VecDeque;

use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{
    config::Config,
    controller::Controller,
    dram::{Dram, DramSpec},
    request::Request,
};

pub trait MemoryTrait {
    fn clk_ns(&self) -> f64;
    fn tick(&mut self);
    fn try_send(&mut self, req: Request) -> Result<(), Request>;
    fn try_recv(&mut self) -> Option<Request>;
    fn pending_requests(&self) -> usize;
    fn finish(&mut self);
}

pub struct SimpleMemory<'a, T> {
    clk: u64,
    addr_bits: Vec<usize>,
    mapping_type: MappingType,
    controllers: Vec<Controller<'a, T>>,
    ret_queue: VecDeque<Request>,
}
impl<'a, T> SimpleMemory<'a, T>
where
    T: DramSpec,
{
    pub fn new(config: &Config, controllers: Vec<Controller<'a, T>>, spec: &'a T) -> Self {
        SimpleMemory {
            clk: 0,
            addr_bits: spec.get_addr_bits().to_vec(),
            mapping_type: config.mapping_type,
            controllers,
            ret_queue: Default::default(),
        }
    }
    pub fn with_config(config: &Config, spec: &'a T) -> Self {
        let mut controllers = vec![];
        let child_size = spec.get_child_size();
        for _i in 0..child_size[0] {
            controllers.push(Controller::new(
                config,
                Dram::new(spec, Level::Channel, child_size),
            ));
        }
        SimpleMemory::new(config, controllers, spec)
    }
}
#[derive(Debug, Clone, Copy)]
pub enum MappingType {
    ChRaBaRoCo,
    RoBaRaCoCh,
    CoRoBaRaCh,
    RoCoBaRaCh,
}
impl MappingType {
    fn get_slice_sequence(&self) -> [usize; 5] {
        match self {
            MappingType::ChRaBaRoCo => [4, 3, 2, 1, 0],
            MappingType::RoBaRaCoCh => [0, 4, 1, 2, 3],
            MappingType::CoRoBaRaCh => [0, 1, 2, 3, 4],
            MappingType::RoCoBaRaCh => [0, 1, 2, 4, 3],
        }
    }
}
#[derive(Debug, Clone, Copy, TryFromPrimitive, IntoPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum Level {
    Channel = 0,
    Rank,
    BankGroup,
    Bank,
    Row,
    Column,
    Max,
}

impl Level {
    pub fn next_level(&self) -> Option<Level> {
        match self {
            Level::Channel => Some(Level::Rank),
            Level::Rank => Some(Level::BankGroup),
            Level::BankGroup => Some(Level::Bank),
            Level::Bank => Some(Level::Row),
            Level::Row => Some(Level::Column),
            Level::Column => Some(Level::Max),
            Level::Max => None,
        }
    }
}

fn slicing_lower_bits(addr: &mut u64, bits: usize) -> u64 {
    let mask = (1 << bits) - 1;
    let lower_bits = *addr & mask;
    *addr >>= bits;
    lower_bits
}
fn clear_lower_bits(addr: &mut u64, bits: usize) {
    *addr >>= bits;
}
fn setup_addr_vec(mut addr: u64, addr_bits: &[usize], addr_vec: &mut [u64], sequence: &[usize]) {
    for &level in sequence.iter() {
        addr_vec[level] = slicing_lower_bits(&mut addr, addr_bits[level]);
    }
}

impl<'a, T> MemoryTrait for SimpleMemory<'a, T>
where
    T: DramSpec,
{
    fn clk_ns(&self) -> f64 {
        todo!()
    }

    fn tick(&mut self) {
        self.clk += 1;
        for controller in self.controllers.iter_mut() {
            controller.tick(self.clk);
            if let Some(req) = controller.finished_queue.pop_front() {
                self.ret_queue.push_back(req);
            }
        }
    }

    fn try_send(&mut self, mut req: Request) -> Result<(), Request> {
        if !req.done_setup {
            req.addr_vec.resize(self.addr_bits.len(), 0);
            let mut addr = req.addr;
            clear_lower_bits(&mut addr, 6);

            setup_addr_vec(
                addr,
                &self.addr_bits,
                &mut req.addr_vec,
                &self.mapping_type.get_slice_sequence(),
            );
            req.done_setup = true;
        }
        self.controllers[req.addr_vec[0] as usize].try_enqueue(req)
    }

    fn pending_requests(&self) -> usize {
        self.controllers
            .iter()
            .map(|c| {
                c.read_queue.size()
                    + c.write_queue.size()
                    + c.act_queue.size()
                    + c.other_queue.size()
            })
            .sum()
    }

    fn finish(&mut self) {
        todo!()
    }

    fn try_recv(&mut self) -> Option<Request> {
        self.ret_queue.pop_front()
    }
}
