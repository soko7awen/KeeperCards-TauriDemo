use tauri::command;
use tokio::io::AsyncWriteExt;
use tokio_serial::SerialPortBuilderExt;

#[command]
async fn test_command(color: String) -> Result<(), String> {
    println!("{color}");
    println!("Sending data over serial...");

    // Set up the serial port.
    let mut port = match tokio_serial::new("/dev/ttyACM0", 115200)
        .open_native_async()
    {
        Ok(port) => port,
        Err(e) => {
            println!("Failed to open serial port: {}", e);
            return Err(e.to_string());
        }
    };

    // Send the string '#FF0000' as bytes over serial.
    let data = color.as_bytes(); // Send as bytes.
    match port.write_all(data).await {
        Ok(_) => {
            println!("Data sent: {:?}", data);
            Ok(())
        }
        Err(e) => {
            println!("Failed to send data: {}", e);
            Err(e.to_string())
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Make sure Tokio runtime is initialized correctly
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    rt.block_on(async {
        tauri::Builder::default()
            .plugin(tauri_plugin_opener::init())
            .invoke_handler(tauri::generate_handler![test_command])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    });
}
