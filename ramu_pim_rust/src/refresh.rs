#[derive(Default)]
pub struct Refresh {}
impl Refresh {
    pub fn tick(&mut self, clk: u64) {
        tracing::error!(clk, "Refresh");
        // TODO: implement refresh
    }
}
