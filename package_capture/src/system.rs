pub(in crate::system) mod devices;

pub fn msg(){
    devices::get_devices();
}