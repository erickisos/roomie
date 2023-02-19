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
