#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};
use tauri::{Manager, State};

#[derive(Default)]
struct Counter(Arc<Mutex<i32>>);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn add_count(num: i32, counter: State<'_, Counter>) -> String {
	let mut val = counter.0.lock().unwrap();
	*val += num;

	format!("{val}")
}

fn main() {
	tauri::Builder::default()
		.setup(|app| {
			let app_handle = app.app_handle();
			tauri::async_runtime::spawn(async move {
				loop {
					//sleep(Duration::from_millis(2000)).await;
					//println!("sending backend-ping");
					//app_handle.emit_all("backend-ping", "ping").unwrap();
				}
			});

			Ok(())
		})
		.manage(Counter::default())
		.invoke_handler(tauri::generate_handler![add_count, greet]) // alle funktionen im array
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
