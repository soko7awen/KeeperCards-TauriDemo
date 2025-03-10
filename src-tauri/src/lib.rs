use serde_json::Value;
use tauri::{Emitter};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::time::{sleep, Duration};
use tokio_serial::{SerialPortBuilderExt};

#[tauri::command]
async fn set_led_color(color: String) -> Result<(), String> {
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

// serial_poll function remains unchanged
#[tauri::command]
async fn serial_poll(app_handle: tauri::AppHandle) -> Result<(), String> {
    println!("Starting serial monitor...");

    // Open the serial port
    let port = match tokio_serial::new("/dev/ttyACM0", 115200)
        .open_native_async()
    {
        Ok(port) => port,
        Err(e) => {
            println!("Failed to open serial port: {}", e);
            return Err(e.to_string());
        }
    };

    let reader = tokio::io::BufReader::new(port);
    let mut lines = reader.lines();

    // Continuous loop to read from serial
    while let Ok(Some(line)) = lines.next_line().await {
        println!("Received: {}", line);

        // Attempt to parse JSON from the Arduino
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
            if let (Some(state), Some(pin)) = (json["state"].as_str(), json["pin"].as_i64()) {
                if state == "connected" {
                    println!("Pin {} connected!", pin);
                    // Set the LED to green (#010)
                    set_led_color("#010".to_string()).await.unwrap_or_else(|e| {
                        println!("Failed to set LED color: {}", e);
                    });
                } else if state == "disconnected" {
                    println!("Pin {} disconnected!", pin);
                    // Fade out LED
                    fade_out_led().await.unwrap_or_else(|e| {
                        println!("Failed to fade out LED: {}", e);
                    });
                }
            }
        }
    }

    Ok(())
}

// This function simulates the fading out of the LED
async fn fade_out_led() -> Result<(), String> {
    let mut brightness = 255;
    while brightness > 0 {
        let color = format!("#00{:02X}00", brightness); // Green to off
        set_led_color(color).await?;

        brightness -= 5; // Slow fade-out step
        sleep(Duration::from_millis(50)).await; // Wait a bit between steps
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Make sure Tokio runtime is initialized correctly
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    rt.block_on(async {
        tauri::Builder::default()
            .plugin(tauri_plugin_opener::init())
            .invoke_handler(tauri::generate_handler![set_led_color, serial_poll])
            .setup(|app| {
                // Access the AppHandle from the app
                let app_handle = app.handle().clone();
                // Spawn serial_poll in the background
                tokio::spawn(async move {
                    if let Err(e) = serial_poll(app_handle).await {
                        println!("Error in serial_poll: {}", e);
                    }
                });
                Ok(())
              })
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    });
}
