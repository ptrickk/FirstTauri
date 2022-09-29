#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};
use tauri::{Manager, State};
use rand::Rng;

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

#[tauri::command]
fn flip_coin(guess: u8) -> String {
	let mut rng = rand::thread_rng();
	let mut result:u8 = rng.gen();
	result %= 2;

	let coin:String;
	let output:String;

	if(result == 0){
		coin = String::from("Kopf");
	}
	else {
		coin = String::from("Zahl");
	}

	if(guess == result){
		output = format!("Richtig! - {coin}");
	}
	else {
		output = format!("Leider Falsch! - {coin}");
	}


	output
}

fn main() {
	tauri::Builder::default()
		.setup(|app| {
			/*
			let app_handle = app.app_handle();
			tauri::async_runtime::spawn(async move {
				loop {
					//sleep(Duration::from_millis(2000)).await;
					//println!("sending backend-ping");
					//app_handle.emit_all("backend-ping", "ping").unwrap();
				}
			});*/

			Ok(())
		})
		.manage(Counter::default())
		.invoke_handler(tauri::generate_handler![add_count, greet, flip_coin]) // alle funktionen im array
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
