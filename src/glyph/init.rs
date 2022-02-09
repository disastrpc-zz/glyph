extern crate vulkano;
extern crate vulkano_win;

use std::sync::Arc;

use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::Version;

pub fn init_vkcontext() -> Arc<Instance> {

    let ext = vulkano_win::required_extensions();
    let instance = match Instance::new(None, Version::V1_5, &ext, None) {
        Ok(arc) => arc,
        Err(err) => panic!("Error initializing VKContext: {:?}", err),
    };

    instance
}