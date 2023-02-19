use cpal::{traits::DeviceTrait, Device, Devices, DevicesError};
use gtk::{prelude::ComboBoxExtManual, ComboBoxText};
use std::iter::Filter;

pub(crate) fn build_dropdown(
    devices_list: Result<Filter<Devices, fn(&Device) -> bool>, DevicesError>,
) -> ComboBoxText {
    let combo_box = ComboBoxText::new();
    for device in devices_list.unwrap() {
        combo_box.append_text(&device.name().unwrap_or("Unknown Device".to_string()));
    }
    combo_box.set_active(Some(0));
    return combo_box;
}
