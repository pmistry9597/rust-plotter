use std::time::Duration;

use serde::Serialize;
use tauri::{State, AppHandle, Manager};

use crate::notify_block::notify_block;

#[tauri::command]
pub fn get_blck_fake(i: i32, fakeblck_repo: State<FakeBlckRepo>) -> Result<FakeBlck, String> {
    let blck = fakeblck_repo.get(i);
    match blck {
        Err(_) => Err(format!("Failure to retrieve blck at index {}", i)),
        Ok(blck) => Ok(blck)
    }
}
#[derive(Serialize, Clone, Copy)]
pub struct FakeBlck {
    fake_blck_thing: &'static str,
}
pub struct FakeBlckRepo {
    blcks: Vec<FakeBlck>,
}
impl FakeBlckRepo {
    pub fn get(&self, i: i32) -> Result<FakeBlck, ()> {
        let i: Result<usize, _> = i.try_into();
        if let Err(_) = i {
            return Err(())
        }
        let i = i.expect("ur not supposed to b here eh");
        match self.blcks.get(i) {
            Some(blck_ref) => Ok(blck_ref.clone()),
            None => Err(()),
        }
    }
}

pub fn init_state() -> FakeBlckRepo {
    let blcks = vec![ 
        FakeBlck{ fake_blck_thing: "wow blck is fake?" },
        FakeBlck{ fake_blck_thing: "wow blck is NOT fake?" }
    ];
    FakeBlckRepo { blcks, }
}

pub async fn notify_worker<'a>(handle: AppHandle) {
    let mut i = 0;
    loop {
        let window = handle.get_window("main").unwrap();
        notify_block(i, "fake_blck", &window).expect("smthn broke from notify_block alright");
        i = (i + 1) % 2;
        tokio::time::sleep(Duration::from_millis(700)).await;
    }
}