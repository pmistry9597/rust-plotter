#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod chart;

use tauri::async_runtime;
use chart::types::{ Vec2 };

async fn shit_data(raw_in: async_runtime::Sender<Vec2>) {
  if let Err(_err) = raw_in.send([0.2, 3.4]).await {
    println!("this fucker took a shit!");
  }
}

async fn print_crap(mut raw_out: async_runtime::Receiver<Vec2>) {
  'whoreloop: loop {
    match raw_out.recv().await {
      Some(raw) => {
        println!("here da dat: {:?}", raw);
      },
      None => {
        break 'whoreloop;
      }
    }
  }
}

fn main() {
  let buf_size: usize = 7;
  let (raw_in, raw_out) = async_runtime::channel::<Vec2>(buf_size); // should move into closure if not used outside

  tauri::Builder::default()
    .setup(|_app| {
      async_runtime::spawn(shit_data(raw_in));
      async_runtime::spawn(print_crap(raw_out));
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}