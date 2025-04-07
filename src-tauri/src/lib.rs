use std::env;
use std::sync::Arc;
use std::str::FromStr;
use palette::{Srgb, LinSrgb, Mix};
use palette::rgb::Rgb;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, ReadHalf, WriteHalf};
use tokio::sync::{Mutex, Semaphore};
use tokio_serial::{SerialPortBuilderExt, SerialStream};
use tauri::{AppHandle,Emitter,Manager};

#[derive(Clone)]
struct SerialPortState {
    writer: Arc<Mutex<Option<WriteHalf<SerialStream>>>>,
    fade_semaphore: Arc<Semaphore>,
}

#[cfg(unix)]
const DEFAULT_TTY: &str = "/dev/ttyACM0";
#[cfg(windows)]
const DEFAULT_TTY: &str = "COM1";

#[tauri::command]
async fn set_led_color(app_handle: AppHandle, color_code: String) -> Result<(), String> {
    let state = app_handle.state::<SerialPortState>();
    let mut guard = state.writer.lock().await;
    
    if let Some(port) = &mut *guard {
        let data = format!("{}\n", color_code);
        port.write_all(data.as_bytes())
            .await
            .map_err(|e| e.to_string())?;
        port.flush()
            .await
            .map_err(|e| e.to_string())?; 
    }
    app_handle.emit("color-changed",&color_code).unwrap();
    Ok(())
}

#[tauri::command]
async fn fade_led(app_handle: AppHandle, color_code: String) -> Result<(), String> {
    let state = app_handle.state::<SerialPortState>();
    let _permit = state.fade_semaphore.acquire().await.map_err(|e| e.to_string())?;
    
    // Calculate colors first, then send them sequentially
    let given_color: Srgb<f32> = Srgb::from_str(&color_code)
        .map_err(|e| e.to_string())?
        .into_format::<f32>();
    
    let given_linear: LinSrgb<f32> = given_color.into_linear();
    let end_linear = LinSrgb::new(0.0, 0.0, 0.0);
    let begin_linear: LinSrgb<f32> = given_linear.mix(end_linear, 0.8);

    let steps = 3;
    for i in 0..=steps {
        let factor = i as f32 / steps as f32;
        let interpolated = begin_linear.mix(end_linear, factor);
        
        let rgb_color: Rgb<palette::encoding::Srgb, u8> = 
            Srgb::<f32>::from_linear(interpolated).into_format::<u8>();
        
        let hex_color = format!(
            "#{:02X}{:02X}{:02X}",
            rgb_color.red,
            rgb_color.green,
            rgb_color.blue
        );

        set_led_color(app_handle.clone(), hex_color).await?;
    }

    Ok(())
}

async fn serial_poll(reader: ReadHalf<SerialStream>, app_handle: AppHandle) -> Result<(), String> {
    let reader = BufReader::new(reader);
    let mut lines = reader.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        println!("Received: {}", line);

        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
            if let (Some(state), Some(pin)) = (json["state"].as_str(), json["pin"].as_i64()) {
                let handle = app_handle.clone();
                
                // Determine color based on pin number
                let color_code = match pin {
                    35 => "#100",
                    36 => "#010",
                    37 => "#001",
                    _ => "#000",
                };

                if state == "connected" {
                    println!("Pin {} connected! Setting color: {}", pin, color_code);
                    tokio::spawn(async move {
                        let _ = set_led_color(handle, color_code.to_string()).await;
                    });
                } else if state == "disconnected" {
                    println!("Pin {} disconnected! Fading from: {}", pin, color_code);
                    tokio::spawn(async move {
                        let _ = fade_led(handle, color_code.to_string()).await;
                    });
                }
            }
        }
    }
    Ok(())
}

#[tauri::command]
async fn serial_setup(app_handle: AppHandle) -> Result<(), String> {
    let tty_path = env::args().nth(1).unwrap_or_else(|| DEFAULT_TTY.into());
    
    let mut port = tokio_serial::new(tty_path, 115200)
        .open_native_async()
        .map_err(|e| format!("Failed to open port: {}", e))?;
    
    port.set_exclusive(false)
        .map_err(|e| format!("Failed to set exclusive: {}", e))?;

    let (reader, writer) = tokio::io::split(port);
    
    let state = app_handle.state::<SerialPortState>();
    let mut guard = state.writer.lock().await;
    *guard = Some(writer);
    drop(guard);

    serial_poll(reader, app_handle).await
}

#[tauri::command]
async fn send_serial_data(
    app_handle: AppHandle,
    data: String,
) -> Result<(), String> {
    set_led_color(app_handle, data).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    rt.block_on(async {
        tauri::Builder::default()
            .plugin(tauri_plugin_opener::init())
            .manage(SerialPortState {
                writer: Arc::new(Mutex::new(None)),
                fade_semaphore: Arc::new(Semaphore::new(1)),
            })
            .invoke_handler(tauri::generate_handler![
                serial_setup, 
                send_serial_data, 
                set_led_color, 
                fade_led
            ])
            .setup(|app| {
                let app_handle = app.handle().clone();
                tokio::spawn(async move {
                    if let Err(e) = serial_setup(app_handle).await {
                        println!("Serial setup error: {}", e);
                    }
                });
                Ok(())
            })
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    });
}