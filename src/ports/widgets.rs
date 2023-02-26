use gtk::{traits::BoxExt, Align, Box, DropDown, Label, Orientation};

fn build_dropdown(devices_list: Vec<String>) -> DropDown {
    let dropdown = DropDown::from_strings(
        devices_list
            .iter()
            .map(|name| name.as_str())
            .collect::<Vec<&str>>()
            .as_slice(),
    );

    dropdown.enables_search();
    return dropdown;
}

pub(crate) fn build_labeled_dropdown(label: &str, devices_list: Vec<String>) -> Box {
    let container_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .build();

    let widget_label = Label::new(Some(label));
    let dropdown = build_dropdown(devices_list);
    container_box.append(&widget_label);
    container_box.append(&dropdown);
    return container_box;
}
fn build_audio_options(inputs: Vec<String>, outputs: Vec<String>) -> Box {
    let horizontal_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .halign(Align::Center)
        .build();
    let dropdown_inputs = build_labeled_dropdown("Entrada", inputs);
    let dropdown_outputs = build_labeled_dropdown("Salida", outputs);

    horizontal_box.append(&dropdown_inputs);
    horizontal_box.append(&dropdown_outputs);

    return horizontal_box;
}

pub(crate) fn build_layout(inputs: Vec<String>, outputs: Vec<String>) -> Box {
    let main_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(10)
        .margin_start(10)
        .margin_end(10)
        .margin_top(10)
        .margin_bottom(10)
        .build();
    let audio_options = build_audio_options(inputs, outputs);
    main_box.append(&audio_options);
    return main_box;
}
