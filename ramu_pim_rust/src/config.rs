use crate::{
    ddr4::{DDR4Org, Speed},
    memory::MappingType,
};

pub struct Config {
    pub channels: usize,
    pub ranks: usize,
    pub ddr4_org: DDR4Org,
    pub ddr4_speed: Speed,
    pub mapping_type: MappingType,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            channels: 1,
            ranks: 1,
            mapping_type: MappingType::RoBaRaCoCh,
            ddr4_org: DDR4Org::DDR4_2Gb_x4,
            ddr4_speed: Speed::DDR4_3200,
        }
    }
}
