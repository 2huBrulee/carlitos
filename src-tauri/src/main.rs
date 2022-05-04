#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rusqlite::Connection;
use std::{env, path::PathBuf};

struct DbPath(PathBuf);

fn main() {
    let cwd_path_buf = env::current_dir().unwrap();

    let cwd_path = cwd_path_buf.join("db.sqlite");

    println!("{}", cwd_path.to_str().unwrap());

    tauri::Builder::default()
        .manage(DbPath(cwd_path))
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn get_db_connection(cwd_path: PathBuf) {
    Connection::open(cwd_path).unwrap();
}
