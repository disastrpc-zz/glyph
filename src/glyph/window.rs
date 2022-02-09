extern crate vulkano;

pub struct Window {
    pub title: String,
    //pub ext: InstanceExtensions
}

pub struct WindowConf {
    pub title: String,
    pub width: i32,
    pub height: i32,
    pub resizable: bool
}
