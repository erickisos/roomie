use cpal::{traits::DeviceTrait, Device, Devices, DevicesError};
use gtk::DropDown;
use std::iter::Filter;

pub(crate) fn build_dropdown(
    devices_list: Result<Filter<Devices, fn(&Device) -> bool>, DevicesError>,
) -> DropDown {
    let device_names: Vec<String> = devices_list
        .unwrap()
        .map(|device| device.name().unwrap())
        .collect();

    let dropdown = DropDown::from_strings(
        device_names
            .iter()
            .map(|name| name.as_str())
            .collect::<Vec<&str>>()
            .as_slice(),
    );

    dropdown.enables_search();
    return dropdown;
}
