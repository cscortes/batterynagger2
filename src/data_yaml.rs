extern crate serde;
extern crate serde_yaml;

use std::fs;
use std::io::prelude::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Conf {
    pub fatal_wait:     u64,
    pub fatal_limit:    u64,
    pub critical_wait:  u64,
    pub critical_limit: u64,
    pub warning_wait:   u64,
    pub warning_limit:  u64,
    pub normal_wait:    u64,
    pub charging_wait:  u64,
}

impl Conf {
    pub fn new() -> Conf {
        let mut  buf = String::new();
        let mut yfile = std::fs::File::open("assets/conf.yaml")
                .expect("Yaml config missing in assets/conf.yaml.");
        yfile.read_to_string(&mut buf).expect("Can't read contents of Yaml file.");
        let prog_conf : Conf = serde_yaml::from_str(buf.as_str()).unwrap();

        prog_conf
    }

    pub fn save(& self) {
        // untested!
        let serialized = serde_yaml::to_string(self).unwrap();
        
        fs::write("assets/conf.yaml", serialized).expect("Can't write to Yaml config file.");
    }
}

