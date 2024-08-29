#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use potool::app;

fn main() {
    app::PotoolApp::new().run();
}
