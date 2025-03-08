mod tray;

use std::fs::File;
use tauri::App;
use serde::{Deserialize, Serialize};
use tauri::ipc::{InvokeError, IpcResponse};
use tauri::menu::{AboutMetadata, CheckMenuItem, CheckMenuItemBuilder, IconMenuItem, IconMenuItemBuilder, Menu, MenuBuilder, MenuEvent, MenuItem, PredefinedMenuItem, Submenu, SubmenuBuilder};
use tauri::{AppHandle, Emitter, Manager, Runtime};
use tauri::image::Image;
use tauri::path::BaseDirectory;

pub type SetupHook = Box<dyn FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send>;

#[derive(Default)]
pub struct AppBuilder {
    setup: Option<SetupHook>,
}

impl AppBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn setup<F>(mut self, setup: F) -> Self
    where
        F: FnOnce(&mut App) -> Result<(), Box<dyn std::error::Error>> + Send + 'static,
    {
        self.setup.replace(Box::new(setup));
        self
    }
    pub fn run(self) {
        #[cfg(desktop)]
        {
            setup_desktop();
        }
        #[cfg(mobile)]
        {
            setup_mobile();
        }
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn greet(name: &str) -> Result<String, InvokeError> {
    Ok(format!("Hello, {}! You've been greeted from Rust--!", name))
}

#[tauri::command]
fn greet2(app_handle: AppHandle, name: &str) {
    // 使用AppHandle发送消息给前端
    app_handle.emit("message-from-backend", format!("Hello--, {}!", name)).unwrap();
    app_handle.emit("test_event","hello").unwrap();
}

// async fn greet(name: &str) -> Result<String, InvokeError> {
//     Ok(format!("Hello, {}! You've been greeted from Rust--!", name))
// }
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust--!", name)
// }
#[cfg(mobile)]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn setup_mobile() {
    run();
}

#[cfg(desktop)]
fn setup_desktop() {
    run();
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // 初始化逻辑
            let _ = config_menu(app);
            Ok(())
        })
        .on_menu_event(move |_app_handle: &tauri::AppHandle, event| {
            match event.id().0.as_str() {
                "en" | "zh" => {
                    println!("menu event: {}", event.id().0);
                }
                _ => {
                    println!("unexpected menu event");
                }
            }
        })
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, greet2, invoke])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}



// 定义一个结构体来表示可以调用的命令
#[derive(Serialize, Deserialize)]
enum CommandRequest {
    Command1 {/* 参数 */},
    Command2 {/* 参数 */},
    // ...
    Command100 {/* 参数 */},
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

fn config_menu(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // let  menu = Menu::new(&())
        // .add_item(MenuItem::About("My App".to_string()))
        // .add_submenu("File", Menu::new()
        //     .add_item(CustomMenuItem::new("Open", "CmdOrCtrl+O"))
    //         .add_item(CustomMenuItem::new("Save", "CmdOrCtrl+S"))
    //         .add_item(MenuItem::Separator)
    //         .add_item(CustomMenuItem::new("Quit", "CmdOrCtrl+Q")));
    let handle = app.handle();
    let file_menu = SubmenuBuilder::new(app, "File")
        .separator()
        .text("open", "Open")
        .quit()
        .text("quit", "Quit").about(None)
        .build()?;

    let lang_str = "en";
    let check_sub_item_1 = CheckMenuItemBuilder::new("English")
        .id("en")
        .checked(lang_str == "en")
        .accelerator("CmdOrCtrl+E")
        .build(app)?;

    let check_sub_item_zh = CheckMenuItemBuilder::with_id("zh", "Chinese")
        .checked(false)
        .build(app)?;
    let check_sub_item_th = CheckMenuItemBuilder::new("Thai")
        .id("th")
        .checked(lang_str == "th")
        .enabled(false)
        .build(app)?;
    let check_sub_item_tw = CheckMenuItemBuilder::new("Taiwan")
        .id("tw")
        .checked(lang_str == "tw")
        .enabled(false)
        .build(app)?;

    // Load icon from path
    let icon_image = Image::from_bytes(include_bytes!("../icons/icon.png")).unwrap();

    let icon_item = IconMenuItemBuilder::new("icon")
        .icon(icon_image.clone())
        .build(app)?;

    let other_item = SubmenuBuilder::new(app, "language")
        .item(&check_sub_item_1)
        .item(&check_sub_item_zh)
        .item(&check_sub_item_th)
        .item(&check_sub_item_tw)
        .build()?;

    let oprations_menu = SubmenuBuilder::new(app, "Opt")
        .undo()
        .redo()
        .separator()
        .cut()
        .copy()
        .paste()
        .select_all()
        .item(&PredefinedMenuItem::copy(app, Some("custom text"))?)
        .item(&MenuItem::new(handle, "MenuItem 1", true, None::<&str>)?)
        .items(&[
            &CheckMenuItem::new(handle, "CheckMenuItem 1", true, true, None::<&str>)?,
            &IconMenuItem::new(handle, "IconMenuItem 1", true, Some(icon_image.clone()), None::<&str>)?,
        ])
        .text("item2", "MenuItem 2")
        .check("checkitem2", "CheckMenuItem 2")
        .icon("iconitem2", "IconMenuItem 2", app.default_window_icon().cloned().unwrap())
        .separator()
        .about(Some(AboutMetadata::default()))
        .quit()
        .build()?;

    let text_menu = MenuItem::with_id(
        app,
        "change_text",
        &"Change menu".to_string(),
        true,
        Some("Ctrl+Z"),
    ).unwrap();

    let menu = MenuBuilder::new(app)
        .items(&[&file_menu, &other_item, &icon_item, &oprations_menu, &text_menu])
        .build()?;
    app.set_menu(menu)?;

    app.on_menu_event(move |_app_handle: &tauri::AppHandle, event| {
        match event.id().0.as_str() {
            "refresh" => {
                // _app_handle.window().emit("reload", {}).unwrap();
                _app_handle.emit("reload", format!("Hello--, {}!", event.id().0)).unwrap();
            },
            "change_text" => {
                text_menu
                    .set_text("changed menu text")
                    .expect("Change text error");

                text_menu
                    .set_text("changed menu text")
                    .expect("Change text error");
            },
            "en" | "zh" => {
                check_sub_item_1
                    .set_checked(event.id().0.as_str() == "en")
                    .expect("Change check error");
                check_sub_item_zh
                    .set_checked(event.id().0.as_str() == "zh")
                    .expect("Change check error");
                check_sub_item_zh.set_accelerator(Some("Ctrl+L"))
                    .expect("Change accelerator error");
            }
            &_ => {}
        }
    });

    // `tauri.conf.json > bundle > resources`
    let resource_path = app.path().resolve("resources/lang/de.json", BaseDirectory::Resource)?;

    let file = std::fs::File::open(&resource_path).unwrap();

    let lang_de: serde_json::Value = serde_json::from_reader(file).unwrap();

    // This will print 'Guten Tag!' to the terminal
    println!("{}", lang_de.get("hello").unwrap());

    Ok(())
}
