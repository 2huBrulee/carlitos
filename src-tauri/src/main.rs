#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rusqlite::Connection;
use std::{env, panic, path::PathBuf};
use tauri::{generate_handler, State};
use tokio::{
    runtime,
    sync::{mpsc, oneshot},
};

struct DbPath(PathBuf);

struct DbActorHandle(MyActorHandle);

#[tokio::main]
async fn main() {
    let cwd_path_buf = env::current_dir().unwrap();

    let cwd_path = cwd_path_buf.join("db.sqlite");

    println!("{}", cwd_path.to_str().unwrap());

    let db_actor_handle = MyActorHandle::new();

    tauri::Builder::default()
        .manage(DbPath(cwd_path))
        .manage(db_actor_handle)
        .invoke_handler(generate_handler![save_carry_run_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn get_db_connection(cwd_path: PathBuf) {
    Connection::open(cwd_path).unwrap();
}

// external calls

#[tauri::command]
async fn save_carry_run_info(data: String, db: State<'_, MyActorHandle>) -> Result<String, String> {
    db.save_carry_run_info(data).await
}

/// ACTOR
struct MyActor {
    receiver: mpsc::Receiver<ActorMessage>,
    next_id: u32,
}
enum ActorMessage {
    GetUniqueId {
        respond_to: oneshot::Sender<u32>,
    },
    SaveCarryRunInfo {
        data: String,
        respond_to: oneshot::Sender<String>,
    },
}

impl MyActor {
    fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
        MyActor {
            receiver,
            next_id: 0,
        }
    }

    fn handle_message(&mut self, msg: ActorMessage) {
        match msg {
            ActorMessage::GetUniqueId { respond_to } => {
                self.next_id += 1;

                // The `let _ =` ignores any errors when sending.
                //
                // This can happen if the `select!` macro is used
                // to cancel waiting for the response.
                let _ = respond_to.send(self.next_id);
            }

            ActorMessage::SaveCarryRunInfo { data, respond_to } => {
                println!("{}", data);
                let _ = respond_to.send("Saved".to_string());
            }
        }
    }

    async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            self.handle_message(msg);
        }
    }
}

#[derive(Clone)]
pub struct MyActorHandle {
    sender: mpsc::Sender<ActorMessage>,
}

impl MyActorHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let mut actor = MyActor::new(receiver);
        tokio::spawn(async move { actor.run().await });

        Self { sender }
    }

    pub async fn get_unique_id(&self) -> u32 {
        let (send, recv) = oneshot::channel();
        let msg = ActorMessage::GetUniqueId { respond_to: send };

        // Ignore send errors. If this send fails, so does the
        // recv.await below. There's no reason to check for the
        // same failure twice.
        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor task has been killed")
    }

    pub async fn save_carry_run_info(&self, data: String) -> Result<String, String> {
        let (send, recv) = oneshot::channel();

        let msg = ActorMessage::SaveCarryRunInfo {
            data,
            respond_to: send,
        };

        let _ = self.sender.send(msg).await;

        let res = recv.await;

        return res.map_err(|_| "Could not save carry run info".to_string());
    }
}
