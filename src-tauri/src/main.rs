mod core;

use bifrost::AppBuilder;

// #![cfg_attr(
//   all(not(debug_assertions), target_os = "windows"),
//   windows_subsystem = "windows"
// )]

pub fn main() {
    AppBuilder::new().run();
}
