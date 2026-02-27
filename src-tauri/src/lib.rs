// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use windows::Win32::UI::WindowsAndMessaging::{
    SPI_GETMOUSESPEED,
    SPI_SETMOUSESPEED,
    SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
    SystemParametersInfoW
};

use tauri::Manager;

use std::fs;

/** SPIF_SENDCHANGE,
SPI_SETMOUSE, */

/** use windows::Win32::System::Registry::{
    HKEY, HKEY_CURRENT_USER, KEY_READ, KEY_SET_VALUE, RRF_RT_REG_SZ, REG_SZ, RegGetValueW, RegOpenKeyExW, RegSetValueExW
};
use windows::core::PCWSTR; */

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MouseInfo {
    pub speed: u32,
    // pub accel_enabled: bool
}

impl MouseInfo {
    pub fn load() -> Self {
        Self {
            speed: Self::get_speed(),
            // accel_enabled: Self::get_accel(),
        }
    }

    pub fn get_speed() -> u32 {
        unsafe {
            let mut speed: u32 = 0;
            let _ = SystemParametersInfoW(
                SPI_GETMOUSESPEED,
                0,
                Some(&mut speed as *mut _ as _),
                SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
            );
            speed /*return*/
        }
    }

    pub fn set_speed(speed: u32) -> bool {
        if speed < 1 || speed > 20 {
            return false;
        }

        unsafe {
            SystemParametersInfoW(
                SPI_SETMOUSESPEED,
                0,
                Some(speed as usize as _),
                SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
            ).is_ok()
        }
    }

    // fn get_accel() -> bool {
    //     unsafe {
    //         let mut hkey = HKEY::default();
    //         let subkey: Vec<u16> = "Control Panel\\Mouse"
    //             .encode_utf16()
    //             .chain(Some(0))
    //             .collect();

    //         if RegOpenKeyExW(HKEY_CURRENT_USER, PCWSTR(subkey.as_ptr()), 0, KEY_READ, &mut hkey).is_err() {
    //             return false;
    //         }

    //         let value_name: Vec<u16> = "MouseSpeed"
    //             .encode_utf16()
    //             .chain(Some(0))
    //             .collect();

    //         let mut buffer = [0u16; 8];
    //         let mut buffer_size = (buffer.len() * 2) as u32;

    //         if RegGetValueW(
    //             hkey,
    //             None,
    //             PCWSTR(value_name.as_ptr()),
    //             RRF_RT_REG_SZ,
    //             None,
    //             Some(buffer.as_mut_ptr() as _),
    //             Some(&mut buffer_size)
    //         ).is_err() {
    //             return false;
    //         }

    //         let value = String::from_utf16_lossy(&buffer);
    //         value.trim_matches(char::from(0)) == "1" /*return*/
    //     }
    // }

    // fn set_mouse_accel(enabled: bool) -> bool {
    //     unsafe {
    //         let mut hkey = HKEY::default();

    //         // 경로 UTF-16 변환
    //         let subkey: Vec<u16> = "Control Panel\\Mouse"
    //             .encode_utf16()
    //             .chain(Some(0))
    //             .collect();

    //         if RegOpenKeyExW(
    //             HKEY_CURRENT_USER,
    //             PCWSTR(subkey.as_ptr()),
    //             0,
    //             KEY_SET_VALUE,
    //             &mut hkey,
    //         )
    //         .is_err()
    //         {
    //             return false;
    //         }

    //         // 값 이름
    //         let value_name: Vec<u16> = "MouseSpeed"
    //             .encode_utf16()
    //             .chain(Some(0))
    //             .collect();

    //         // "1" 또는 "0"
    //         let value_str = if enabled { "1" } else { "0" };

    //         let data: Vec<u16> = value_str
    //             .encode_utf16()
    //             .chain(Some(0))
    //             .collect();

    //         if RegSetValueExW(
    //             hkey,
    //             PCWSTR(value_name.as_ptr()),
    //             0,
    //             REG_SZ,
    //             Some(data.as_ptr() as *const u8),
    //             (data.len() * 2) as u32,
    //         ).is_err() {
    //             return false;
    //         }

    //         // Windows에 변경 알림
    //         SystemParametersInfoW(
    //             SPI_SETMOUSE,
    //             0,
    //             None,
    //             SPIF_SENDCHANGE,
    //         )
    //         .is_ok()
    //     }
    // }
}

#[tauri::command]
fn get_mouse_info() -> MouseInfo {
    MouseInfo::load()
}

#[tauri::command]
fn set_mouse_speed(speed: u32) -> bool {
    MouseInfo::set_speed(speed)
}

#[tauri::command]
fn save_mouse_state(app: tauri::AppHandle) -> bool {
    println!("SAVE CALLED");

    let info = MouseInfo::load();

    let app_dir = app.path().app_data_dir().unwrap();
    std::fs::create_dir_all(&app_dir).ok();

    let file_path = app_dir.join("mouse_state.json");

    let json = match serde_json::to_string_pretty(&info) {
        Ok(j) => j,
        Err(_) => return false,
    };

    match fs::write("mouse_state.json", json) {
        Ok(_) => {
            println!("SAVE SUCCESS");
            true
        }
        Err(e) => {
            println!("SAVE ERROR: {:?}", e);
            false
    }
}
}

#[tauri::command]
fn apply_mouse_state() -> bool {
    println!("APPLY CALLED");

    let content = match std::fs::read_to_string("mouse_state.json") {
        Ok(c) => {
            println!("READ SUCCESS: {}", c);
            c
        }
        Err(e) => {
            println!("READ ERROR: {:?}", e);
            return false;
        }
    };

    let saved: MouseInfo = match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(_) => return false,
    };

    MouseInfo::set_speed(saved.speed);

    true
}
 
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_mouse_info,
            set_mouse_speed,
            save_mouse_state,
            apply_mouse_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
