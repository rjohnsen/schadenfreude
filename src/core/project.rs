use crate::core::config;

pub fn load_project() {
    let mut configuration = config::Config::default();
    configuration.load(); 

    println!("{:?}", configuration);
}