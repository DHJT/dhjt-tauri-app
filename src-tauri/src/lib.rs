mod tray;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn greet(name: &str) -> Result<String, InvokeError> {
    Ok(format!("Hello, {}! You've been greeted from Rust--!", name))
}
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust--!", name)
// }
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, invoke])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use tauri::{Manager};
use serde::{Deserialize, Serialize};
use tauri::ipc::InvokeError;
use tauri::menu::{Menu, MenuItem};

// 定义一个结构体来表示可以调用的命令
#[derive(Serialize, Deserialize)]
enum CommandRequest {
    Command1 { /* 参数 */ },
    Command2 { /* 参数 */ },
    // ...
    Command100 { /* 参数 */ },
}

// 定义一个结构体来表示命令的响应
#[derive(Serialize, Deserialize)]
enum CommandResponse {
    Response1(String),
    Response2(i32),
    // ...
    Response100(Vec<u8>),
}

// 中央处理函数
async fn handle_command(request: CommandRequest) -> Result<CommandResponse, InvokeError> {
    match request {
        CommandRequest::Command1 { /* 参数 */ } => {
            // 处理逻辑
            Ok(CommandResponse::Response1("result".to_string()))
        }
        CommandRequest::Command2 { /* 参数 */ } => {
            // 处理逻辑
            Ok(CommandResponse::Response2(42))
        }
        // ...
        CommandRequest::Command100 { /* 参数 */ } => {
            // 处理逻辑
            Ok(CommandResponse::Response100(vec![1, 2, 3]))
        }
    }
}

// Tauri 的 invoke_handler
#[tauri::command]
async fn invoke(request: CommandRequest) -> Result<CommandResponse, InvokeError> {
    handle_command(request).await
}


fn config_menu() {
    // let  menu = Menu::new()
    //     .add_item(MenuItem::About("My App".to_string()))
    //     .add_submenu("File", Menu::new()
    //         .add_item(CustomMenuItem::new("Open", "CmdOrCtrl+O"))
    //         .add_item(CustomMenuItem::new("Save", "CmdOrCtrl+S"))
    //         .add_item(MenuItem::Separator)
    //         .add_item(CustomMenuItem::new("Quit", "CmdOrCtrl+Q")));
}
