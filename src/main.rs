pub mod domain;
pub mod ports;
use domain::audio;
use gtk::{
    prelude::{ApplicationExt, ApplicationExtManual},
    traits::{BoxExt, GtkWindowExt},
    Application, ApplicationWindow, Orientation,
};
use ports::widgets;

const APP_ID: &str = "com.tliwaka.roomie";

fn main() {
    let application = Application::builder().application_id(APP_ID).build();
    application.connect_activate(start_app);
    application.run();
}

fn start_app(app: &Application) {
    // let list_store_inputs = ListStore::from(audio::get_inputlist());
    let inputs = audio::get_inputlist();
    let outputs = audio::get_outputlist();
    let dropdown_inputs = widgets::build_dropdown(inputs);
    let dropdown_outputs = widgets::build_dropdown(outputs);
    // Add buttons to `gtk_box`
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&dropdown_inputs);
    gtk_box.append(&dropdown_outputs);
    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Roomie")
        .child(&gtk_box)
        .build();
    window.present();
}
