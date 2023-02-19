use cpal::traits::DeviceTrait;
use gtk::{prelude::ComboBoxExtManual, ComboBoxText};

pub(crate) fn build_dropdown(
    devices_list: Result<
        std::iter::Filter<cpal::Devices, fn(&cpal::Device) -> bool>,
        cpal::DevicesError,
    >,
) -> ComboBoxText {
    let combo_box = ComboBoxText::new();
    for device in devices_list.unwrap() {
        combo_box.append_text(&device.name().unwrap_or("Unknown Device".to_string()));
    }
    combo_box.set_active(Some(0));
    return combo_box;
}
