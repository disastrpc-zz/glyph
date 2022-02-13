extern crate vulkano;
extern crate vulkano_win;

use std::sync::Arc;

use vulkano::app_info_from_cargo_toml;
use vulkano::Version;

use vulkano::instance::Instance;

use vulkano::device::physical::{
    PhysicalDevice, 
    QueueFamily
};
use vulkano::device::{
    Device, 
    DeviceExtensions, 
    QueuesIter,
    Features
};

#[allow(dead_code, unused)]
pub struct Glyph {
    device: Arc<Device>,
    queues: QueuesIter,
    phyid: usize
}

#[allow(dead_code, unused)]
impl <'a> Glyph {
    pub fn new(d_index: Option<usize>) -> Self {

        let i: usize;
        let families: Vec<QueueFamily>;

        let app_info = app_info_from_cargo_toml!();
        let ext = vulkano_win::required_extensions();

        let instance = match Instance::new(
            Some(&app_info), 
            Version::V1_5, 
            &ext, None) {

            Ok(arc) => arc,
            Err(err) => panic!("error initializing glyph: {:?}", err),
        }; 

        // Default to first device if no index is specified
        if let None = d_index {
            i = 0;
        } else {
            i = d_index.unwrap();
        }

        let physical = match PhysicalDevice::from_index(&instance, i) {
            Some(device) => device,
            None => panic!("unable to find Vulkan compatible device"),
        };

        let families = Self::enum_queue_fams(&physical);
        let ext = Self::enum_vk_ext(&physical);
        let feat = Self::enum_vk_feat(&physical);
        
        let (device, queues) = {
            Device::new(physical, 
                &feat, 
                &ext, 
                families
                    .iter()
                    .cloned()
            ).unwrap()
        };

        Self {
            device: device,
            queues: queues,
            phyid: physical.index()
        }
    }

    fn enum_queue_fams(physical: &'a PhysicalDevice) -> Vec<(QueueFamily<'a>, f32)> {

        let mut families = Vec::new();

        for family in physical.queue_families() {
            if family.supports_graphics() {
                families.push((family, 0.5))
            }
        }

        families
    }

    fn enum_vk_feat(physical: &PhysicalDevice) -> Box<Features> {

        let min = Features {
            geometry_shader: true,
            .. Features::none()
        };
        
        let opt = Features {
            geometry_shader: true,
            tessellation_shader: true,
            .. Features::none()
        };

        assert!(opt.is_superset_of(&min));
        
        if !physical.supported_features().is_superset_of(&min) {
            panic!("The physical device is not good enough for this application.");
        }

        Box::new(opt.intersection(physical.supported_features()))
    }

    fn enum_vk_ext(physical: &PhysicalDevice) -> Box<DeviceExtensions> {
        let min = DeviceExtensions {
            khr_swapchain: true,
            .. DeviceExtensions::none()
        };

        let opt = DeviceExtensions {
            khr_swapchain: true,
            .. DeviceExtensions::none()
        };

        assert!(opt.is_superset_of(&min));
        if !physical.supported_extensions().is_superset_of(&min) {
            panic!("The physical device is not good enough for this application.");
        }

        Box::new(opt.intersection(physical.supported_extensions()))
    }

}

