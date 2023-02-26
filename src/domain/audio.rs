use cpal::{
    self,
    traits::{DeviceTrait, HostTrait},
    Device,
};

pub(crate) fn get_outputlist() -> Vec<String> {
    return cpal::default_host()
        .output_devices()
        .unwrap()
        .map(|device| device.name().unwrap())
        .collect();
}

pub(crate) fn get_inputlist() -> Vec<String> {
    return cpal::default_host()
        .input_devices()
        .unwrap()
        .map(|device| device.name().unwrap())
        .collect();
}

pub(crate) fn sweep(output_device: String) {
    let device = find_device(&output_device);
    play(&device)
}

fn find_device(device_name: &str) -> Device {
    return cpal::default_host()
        .devices()
        .unwrap()
        .into_iter()
        .find(|device| device.name().unwrap() == device_name)
        .unwrap();
}

fn play(device: &Device) {}
