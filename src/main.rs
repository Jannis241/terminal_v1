
pub mod window;
pub mod command;

fn main() {
    let terminal_settings = window::Settings::init("$ ", "Terminal V1", 700, 450);

    window::create(terminal_settings);
}
