use gtk::{glib, prelude::*, Application, ApplicationWindow};

const APP_ID : &str = "org.gtk_rs.Termio";

pub fn run_ui() -> glib::ExitCode {
     let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
    .application(app)
    .title("Termio Terminal Emulator")
    .build();

    window.present();
}