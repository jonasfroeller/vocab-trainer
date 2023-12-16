// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Stops the client from outputting a huge number of warnings during compilation.
#[allow(warnings, unused)]
mod db;

use db::*;
use serde::Deserialize;
use specta::Type;
use std::sync::Arc;
use tauri::State;
use crate::db::text::Data;
// use tauri_specta::ts;

type DbState<'a> = State<'a, Arc<PrismaClient>>;

#[tokio::main]
async fn main() {
    let db = PrismaClient::_builder().build().await.unwrap();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_all_vocabs, post_vocab])
        .manage(Arc::new(db)) // shares client globally
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
#[specta::specta]
async fn get_all_vocabs(db: DbState<'_>) -> Result<Vec<Data>, ()> {
    db.text()
        .find_many(vec![])
        .exec()
        .await
        .map_err(|_| ())
}

#[derive(Deserialize, Type)]
struct CreateTextData {
    title: String,
    content: String,
}

#[tauri::command]
#[specta::specta]
async fn post_vocab(db: DbState<'_>, data: CreateTextData) -> Result<Data, ()> {
    db.text()
        .create(data.title, data.content, vec![])
        .exec()
        .await
        .map_err(|_| ())
}
