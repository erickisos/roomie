use cpal::traits::DeviceTrait;
use gtk::ComboBoxText;

pub(crate) fn build_dropdown(
    devices_list: std::iter::Filter<cpal::Devices, fn(&cpal::Device) -> bool>,
) -> ComboBoxText {
    let combo_box = ComboBoxText::new();
    for device in devices_list {
        combo_box.append_text(&device.name().unwrap_or("Unknown Device".to_string()));
    }
    return combo_box;
}
