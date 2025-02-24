use std::time::Duration;
use serialport::prelude::*;
use std::io::Write;

#[tauri::command]
fn test_command(port_name: &str, color: &str) -> () {
    let baud_rate = BaudRate::BaudOther(115200);
    let settings = SerialPortSettings {
        baud_rate,
        timeout: Duration::from_secs(1),
        ..Default::default()
    };

    let port = serialport::new("/dev/ttyUSB0", 115_200);
    serialport::open_with_settings(port, &settings);

    let output = "This is a test. This is only a test.".as_bytes();
    port.write(output).expect("Write failed!");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![test_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
