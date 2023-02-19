use cpal::{self, traits::HostTrait, Devices, DevicesError};

pub(crate) fn get_inputlist(
) -> Result<std::iter::Filter<Devices, for<'r> fn(&'r cpal::Device) -> bool>, DevicesError> {
    let host = cpal::default_host();
    return host.input_devices();
}

pub(crate) fn get_outputlist(
) -> Result<std::iter::Filter<Devices, for<'r> fn(&'r cpal::Device) -> bool>, DevicesError> {
    let host = cpal::default_host();
    return host.output_devices();
}

// fn print_hosts() {
//     for host_id in cpal::available_hosts() {
//         let host = cpal::host_from_id(host_id).unwrap();
//         for (device_index, device) in host.input_devices().unwrap().enumerate() {
//             println!("{} -> {}", device_index, device.name().unwrap());
//         }

//         for (device_index, device) in host.output_devices().unwrap().enumerate() {
//             println!("{} -> {}", device_index, device.name().unwrap());
//         }
//     }
// }
