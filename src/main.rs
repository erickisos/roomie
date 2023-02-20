pub mod controllers;
pub mod domain;
pub mod ports;
use domain::audio;
use gtk::{
    prelude::{ApplicationExt, ApplicationExtManual},
    traits::GtkWindowExt,
    Application, ApplicationWindow,
};
use ports::widgets;

const APP_ID: &str = "com.tliwaka.roomie";

fn main() {
    let application = Application::builder().application_id(APP_ID).build();
    application.connect_activate(start_app);
    application.run();
}

fn start_app(app: &Application) {
    let inputs = audio::get_inputlist();
    let outputs = audio::get_outputlist();
    let main_box = widgets::build_layout(inputs, outputs);
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Roomie")
        .child(&main_box)
        .build();
    window.present();
}
