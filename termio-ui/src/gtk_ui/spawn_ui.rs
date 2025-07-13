use gtk::{glib, prelude::*, Application, ApplicationWindow, Orientation, ScrolledWindow, TextView};

const APP_ID : &str = "org.termio.Termio";

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
    .default_width(320)
    .default_height(200)
    .build();

    // TextView inside a ScrolledWindow
    let text_view = TextView::builder()
        .wrap_mode(gtk::WrapMode::None)
        .monospace(true)
        .editable(true)
        .cursor_visible(true)
        .build();
    
    let scroll = ScrolledWindow::builder()
        .child(&text_view)
        .hscrollbar_policy(gtk::PolicyType::Automatic)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .propagate_natural_height(true)
        .propagate_natural_width(true)
        .build();

    scroll.set_vexpand(true);
    scroll.set_hexpand(true);

    let layout = gtk::Box::new(Orientation::Vertical, 0);
    layout.append(&scroll);

    layout.set_vexpand(true);
    layout.set_hexpand(true);

    // Add to window
    window.set_child(Some(&layout));
    window.present();
}