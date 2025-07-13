use termio_ui::run_ui;
use gtk::glib;

fn main() -> glib::ExitCode{
    let result : glib::ExitCode = run_ui();
    result
}
