#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[allow(unused_imports)]
#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;

use dotenv::dotenv;
use log::{debug, error, warn};
use serde::Serialize;
use std::io::Write;
use std::sync::{
    mpsc::{channel, Receiver, Sender},
    Mutex, Once,
};
use tauri::{
    AppHandle, CustomMenuItem, GlobalShortcutManager, Manager, Menu, RunEvent, Submenu, SystemTray,
    SystemTrayEvent, SystemTrayMenu, WindowMenuEvent
};

static mut SAFE_START_RECEIVER: Option<Mutex<Sender<()>>> = None;
static SAFE_START_INIT: Once = Once::new();

#[derive(Clone, Serialize)]
struct Reply {
  data: String,
}

fn set_safe_start_sender(sender: Sender<()>) {
    SAFE_START_INIT.call_once(|| unsafe {
        SAFE_START_RECEIVER = Some(Mutex::new(sender));
    });
}

fn get_safe_start_sender() -> Option<Sender<()>> {
    unsafe {
        SAFE_START_RECEIVER
            .as_ref()
            .map(|r| r.lock().unwrap().clone())
    }
}

fn setup_logger() {
    let mut env_logger_builder: env_logger::Builder = env_logger::Builder::from_default_env();
    env_logger_builder.format(|f, record| {
        let level_string: &str;
        match record.level() {
            log::Level::Error => level_string = "E",
            log::Level::Warn => level_string = "W",
            log::Level::Info => level_string = "I",
            log::Level::Debug => level_string = "D",
            log::Level::Trace => level_string = "T",
        }

        writeln!(
            f,
            "{} [{}] {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            level_string,
            record.args()
        )
    });
    env_logger_builder.init();
}

#[tauri::command]
fn safe_close_splashscreen() {
    debug!("Safe Splash Close Triggered");

    match get_safe_start_sender() {
        Some(sender) => {
            sender.send(()).unwrap();
        }
        None => {
            error!("No Safe Splash Sender Found Cannot Close Splash Screen");
            std::thread::sleep(std::time::Duration::from_secs(2));
            std::process::exit(0);
        }
    }
}

fn main() {
    dotenv().ok();
    setup_logger();

    let (sender, receiver): (Sender<()>, Receiver<()>) = channel();
    set_safe_start_sender(sender);

    let close: CustomMenuItem = CustomMenuItem::new("close".to_string(), "Close");
    let hide: CustomMenuItem = CustomMenuItem::new("hide", "Hide");
    let submenu: Submenu = Submenu::new("File", Menu::new().add_item(close).add_item(hide));
    let menu: Menu = Menu::new().add_submenu(submenu);

    let app = tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools();

            let splashscreen_window = app.get_window("splashscreen").unwrap();
            let main_window = app.get_window("main").unwrap();

            tauri::async_runtime::spawn(async move {
                debug!("Initializing App...");
                // Place To Run Initialization Code
                std::thread::sleep(std::time::Duration::from_secs(2));
                // Wait for the client to be ready
                receiver.recv().unwrap();
                splashscreen_window.hide().expect("Failed to hide splash");
                main_window.show().expect("Failed to show main window");
            });

            Ok(())
        })
        .on_page_load(|window, _| {
            let window_ = window.clone();

            window.listen("js-event", move |event| {
                println!("got js-event with message '{:?}'", event.payload());
                let reply = Reply {
                    data: "something else".to_string(),
                };

                window_
                    .emit("rust-event", Some(reply))
                    .expect("failed to emit");
            });
        })
        .system_tray(
            SystemTray::new().with_menu(
                SystemTrayMenu::new()
                    .add_item(CustomMenuItem::new("exit", "Exit"))
                    .add_item(CustomMenuItem::new("cards", "Cards")),
            ),
        )
        .on_system_tray_event(|app: &AppHandle, event: SystemTrayEvent| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            }
           SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "exit" => {
                        let window = app.get_window("main").unwrap();
                        window.close().unwrap();
                        std::process::exit(0);
                    }
                    "cards" => {
                        // TODO: Implement redirect to cards page
                        println!("Cards Clicked");
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .menu(menu)
        .on_menu_event(|event: WindowMenuEvent| match event.menu_item_id() {
            "close" => {
                warn!("Closing...");
                std::thread::sleep(std::time::Duration::from_secs(1));
                std::process::exit(0);
            }
            "hide" => {
                event.window().hide().unwrap();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![safe_close_splashscreen])
        .build(tauri::generate_context!())
        .expect("Error while building app");

    app.run(|app_handle, e| match e {
        RunEvent::Ready => {
            let app_handle = app_handle.clone();
            app_handle.global_shortcut_manager().register("Ctrl+Q", move || {
                let window = app_handle.get_window("main").unwrap();
                window.close().unwrap();
                std::process::exit(0);
            }).expect("Could not create binding for closing to tray!");
        }

        RunEvent::ExitRequested {api: _, ..} => {
            debug!("App Close");
            std::thread::sleep(std::time::Duration::from_secs(2));
            std::process::exit(0);
        }

        _ => {}
    });
}
