// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{self, File};
use std::io::{copy, Read, Write};
use std::path::{PathBuf, Path};
use std::process::Command;
use std::env::consts::OS;
use serde::{Deserialize, Serialize};
use serde_json;
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_autostart::ManagerExt;
use reqwest::Client;
use futures_util::StreamExt;

fn get_installation_dir() -> Result<std::path::PathBuf, String> {
    let local_app_data = dirs::data_local_dir()
        .ok_or("Could not find local app data directory")?;
    let install_dir = local_app_data
        .join("EmulatorX")
        .join("emulators");
    fs::create_dir_all(&install_dir).map_err(|e| e.to_string())?;
    Ok(install_dir)
}

#[derive(serde::Serialize)]
struct EmulatorStatus {
    name: String,
    installed: bool,
}

#[tauri::command]
async fn check_installations() -> Result<Vec<EmulatorStatus>, String> {
    let install_dir = get_installation_dir()?;
    
    let emulators = vec![
        ("Dolphin", "dolphin"),
        ("Xenia", "xenia"),
        ("PCSX2", "pcsx2"),
        ("RPCS3", "rpcs3"),
        ("DuckStation", "duckstation"),
        ("mGBA", "mgba"),
        ("xemu", "xemu"),
        ("PPSSPP", "ppsspp"),
        ("Flycast", "flycast"),
        ("ZSNES", "zsnes"),
        ("Mesen", "mesen"),
    ];
    
    let statuses = emulators.into_iter()
        .map(|(name, dir)| EmulatorStatus {
            name: name.to_string(),
            installed: install_dir.join(dir).exists(),
        })
        .collect();
    
    Ok(statuses)
}

#[tauri::command]
async fn download_emulator(emulator: String) -> Result<(), String> {
    match emulator.as_str() {
        "Dolphin" => download_and_extract(
            "https://dl.dolphin-emu.org/releases/2412/dolphin-2412-x64.7z",
            "dolphin.7z",
            true,
        ).await,
        "Xenia" => download_and_extract(
            "https://github.com/xenia-project/release-builds-windows/releases/latest/download/xenia_master.zip",
            "xenia.zip",
            false,
        ).await,
        "PCSX2" => download_and_extract(
            "https://github.com/PCSX2/pcsx2/releases/download/v2.0.0/pcsx2-v2.0.0-windows-x64-Qt.7z",
            "pcsx2.7z",
            true,
        ).await,
        "RPCS3" => download_and_extract(
            "https://github.com/RPCS3/rpcs3-binaries-win/releases/download/build-394fc8eb79845caaf9528d7c1ac6fd78d653863c/rpcs3-v0.0.34-17416-394fc8eb_win64.7z",
            "rpcs3.7z",
            true,
        ).await,
        "DuckStation" => download_and_extract(
            "https://github.com/stenzek/duckstation/releases/download/latest/duckstation-windows-x64-release.zip",
            "duckstation.zip",
            false,
        ).await,
        "mGBA" => download_and_extract(
            "https://github.com/mgba-emu/mgba/releases/download/0.10.4/mGBA-0.10.4-win32.7z",
            "mgba.7z",
            true,
        ).await,
        "xemu" => download_and_extract(
            "https://github.com/xemu-project/xemu/releases/latest/download/xemu-win-x86_64-release.zip",
            "xemu.zip",
            false,
        ).await,
        "PPSSPP" => download_and_extract(
            "https://www.ppsspp.org/files/1_18_1/ppsspp_win.zip",
            "ppsspp.zip",
            false,
        ).await,
        "Flycast" => download_and_extract(
            "https://github.com/flyinghead/flycast/releases/download/v2.4/flycast-win64-2.4.zip",
            "flycast.zip",
            false,
        ).await,
        "ZSNES" => download_and_extract(
            "https://www.fosshub.com/ZSNES.html?dwl=zsnes151.zip",
            "zsnes.zip",
            false,
        ).await,
        "Mesen" => download_and_extract(
            "https://nightly.link/SourMesen/Mesen2/workflows/build/master/Mesen%20%28Windows%20-%20net8.0%20-%20AoT%29.zip",
            "mesen.zip",
            false,
        ).await,
        _ => Err("Unknown emulator".into()),
    }
}

async fn download_and_extract(url: &str, filename: &str, is_7z: bool) -> Result<(), String> {
    // Use temp dir for downloads
    let temp_dir = std::env::temp_dir().join("emulatorx_downloads");
    fs::create_dir_all(&temp_dir).map_err(|e| format!("Failed to create temp dir: {}", e))?;
    
    let temp_filepath = temp_dir.join(filename);
    println!("Downloading to: {:?}", temp_filepath);
    
    // Download file to temp directory
    let response = reqwest::get(url)
        .await
        .map_err(|e| format!("Failed to download: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Download failed with status: {}", response.status()));
    }

    // Get content length if available
    let content_length = response.content_length();
    println!("Expected download size: {} bytes", content_length.unwrap_or(0));
    
    let content = response.bytes()
        .await
        .map_err(|e| format!("Failed to get response bytes: {}", e))?;
    
    println!("Actual downloaded size: {} bytes", content.len());
    
    if content.len() < 1000000 { // Less than 1MB
        return Err(format!("Download seems too small ({} bytes). Possible error.", content.len()));
    }

    let mut file = File::create(&temp_filepath)
        .map_err(|e| format!("Failed to create temp file: {}", e))?;
    
    copy(&mut content.as_ref(), &mut file)
        .map_err(|e| format!("Failed to write download to file: {}", e))?;

    // Get permanent installation directory
    let install_dir = get_installation_dir()?;
    let emulator_name = Path::new(filename).file_stem().unwrap().to_str().unwrap();
    let emulator_dir = install_dir.join(emulator_name);
    
    println!("Extracting to: {:?}", emulator_dir);
    
    // Create fresh directory
    if emulator_dir.exists() {
        fs::remove_dir_all(&emulator_dir)
            .map_err(|e| format!("Failed to remove existing directory: {}", e))?;
    }
    fs::create_dir_all(&emulator_dir)
        .map_err(|e| format!("Failed to create emulator directory: {}", e))?;

    // Extract based on file type
    if is_7z {
        match sevenz_rust::decompress_file(&temp_filepath, &emulator_dir) {
            Ok(_) => println!("7z extraction successful"),
            Err(e) => {
                fs::remove_dir_all(&emulator_dir).ok(); // Clean up on failure
                return Err(format!("7z extraction failed: {}", e));
            }
        }
    } else {
        let file = File::open(&temp_filepath)
            .map_err(|e| format!("Failed to open zip file: {}", e))?;
        let mut archive = zip::ZipArchive::new(file)
            .map_err(|e| format!("Failed to read zip archive: {}", e))?;
        
        match archive.extract(&emulator_dir) {
            Ok(_) => println!("Zip extraction successful"),
            Err(e) => {
                fs::remove_dir_all(&emulator_dir).ok(); // Clean up on failure
                return Err(format!("Zip extraction failed: {}", e));
            }
        }
    }

    // Clean up downloaded archive
    fs::remove_file(&temp_filepath)
        .map_err(|e| format!("Failed to clean up temp file: {}", e))?;

    println!("Installation completed successfully");
    Ok(())
}

#[tauri::command]
async fn uninstall_emulator(emulator: String) -> Result<(), String> {
    let install_dir = get_installation_dir()?;
    let emulator_dir = install_dir.join(match emulator.as_str() {
        "Dolphin" => "dolphin",
        "Xenia" => "xenia",
        "PCSX2" => "pcsx2",
        "RPCS3" => "rpcs3",
        "DuckStation" => "duckstation",
        "mGBA" => "mgba",
        "xemu" => "xemu",
        _ => return Err("Unknown emulator".into()),
    });

    if emulator_dir.exists() {
        fs::remove_dir_all(&emulator_dir).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err(format!("Emulator {} is not installed", emulator))
    }
}

fn get_emulator_executable(install_dir: &PathBuf, emulator: &str) -> Result<PathBuf, String> {
    let base_path = install_dir.join(match emulator {
        "Dolphin" => "dolphin",
        "Xenia" => "xenia",
        "PCSX2" => "pcsx2",
        "RPCS3" => "rpcs3",
        "DuckStation" => "duckstation",
        "mGBA" => "mgba",
        "xemu" => "xemu",
        _ => return Err("Unknown emulator".into()),
    });

    println!("Looking for executable in: {:?}", base_path);

    let exe_path = match (emulator, OS) {
        ("Dolphin", "windows") => base_path.join("Dolphin-x64").join("Dolphin.exe"),
        ("Xenia", "windows") => base_path.join("xenia.exe"),
        ("PCSX2", "windows") => base_path.join("pcsx2-qt.exe"),
        ("RPCS3", "windows") => base_path.join("rpcs3.exe"),
        ("DuckStation", "windows") => base_path.join("duckstation-qt-x64-ReleaseLTCG.exe"),
        ("mGBA", "windows") => base_path.join("mGBA.exe"),
        ("xemu", "windows") => base_path.join("xemu.exe"),
        _ => return Err(format!("Unsupported OS {} for emulator {}", OS, emulator)),
    };

    println!("Attempting to launch: {:?}", exe_path);

    // List directory contents to help debug
    if let Ok(entries) = fs::read_dir(&base_path) {
        println!("Directory contents of {:?}:", base_path);
        for entry in entries {
            if let Ok(entry) = entry {
                println!("Found: {:?}", entry.path());
            }
        }
    }

    if !exe_path.exists() {
        return Err(format!("Executable not found at {:?}", exe_path));
    }

    Ok(exe_path)
}

#[tauri::command]
async fn run_emulator(emulator: String) -> Result<(), String> {
    let install_dir = get_installation_dir()?;
    
    let exe_path = match emulator.as_str() {
        "Dolphin" => get_emulator_executable(&install_dir, &emulator)?,
        "Xenia" => get_emulator_executable(&install_dir, &emulator)?,
        "PCSX2" => get_emulator_executable(&install_dir, &emulator)?,
        "RPCS3" => get_emulator_executable(&install_dir, &emulator)?,
        "DuckStation" => get_emulator_executable(&install_dir, &emulator)?,
        "mGBA" => get_emulator_executable(&install_dir, &emulator)?,
        "xemu" => get_emulator_executable(&install_dir, &emulator)?,
        "PPSSPP" => install_dir.join("ppsspp").join("PPSSPPWindows64.exe"),
        "Flycast" => install_dir.join("flycast").join("flycast.exe"),
        "ZSNES" => install_dir.join("zsnes").join("zsnesw.exe"),
        "Mesen" => install_dir.join("mesen").join("Mesen.exe"),
        _ => return Err(format!("Unknown emulator: {}", emulator)),
    };

    if !exe_path.exists() {
        return Err(format!("Emulator executable not found at {:?}", exe_path));
    }

    match Command::new(&exe_path)
        .current_dir(exe_path.parent().unwrap_or(&install_dir))
        .spawn()
    {
        Ok(_) => {
            println!("Successfully launched {}", emulator);
            Ok(())
        }
        Err(e) => {
            Err(format!("Failed to launch {}: {} (path: {:?})", emulator, e, exe_path))
        }
    }
}

#[derive(Serialize, Deserialize)]
struct AppSettings {
    auto_start: bool,
    check_updates: bool,
    close_to_tray: bool,
    rom_server_url: String,
}

#[tauri::command]
fn get_install_path() -> Result<String, String> {
    let path = get_installation_dir()?;
    Ok(path.to_string_lossy().into_owned())
}

fn get_settings_file() -> Result<PathBuf, String> {
    let app_dir = dirs::data_local_dir()
        .ok_or("Could not find local app data directory")?
        .join("EmulatorX");
    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    Ok(app_dir.join("settings.json"))
}

#[tauri::command]
fn load_settings() -> Result<AppSettings, String> {
    let install_dir = get_installation_dir()?;
    let settings_file = install_dir.join("settings.json");
    
    if settings_file.exists() {
        let contents = fs::read_to_string(&settings_file)
            .map_err(|e| format!("Failed to read settings file: {}", e))?;
        
        serde_json::from_str(&contents)
            .map_err(|e| format!("Failed to parse settings: {}", e))
    } else {
        // Default settings
        Ok(AppSettings {
            auto_start: false,
            check_updates: true,
            close_to_tray: false,
            rom_server_url: "http://localhost:1248".to_string(),
        })
    }
}

#[tauri::command]
fn save_settings(settings: AppSettings) -> Result<(), String> {
    let install_dir = get_installation_dir()?;
    let settings_file = install_dir.join("settings.json");
    
    let json = serde_json::to_string_pretty(&settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;
    
    fs::write(&settings_file, json)
        .map_err(|e| format!("Failed to write settings file: {}", e))?;
    
    Ok(())
}

#[tauri::command]
async fn toggle_auto_launch(app_handle: tauri::AppHandle, enable: bool) -> Result<(), String> {
    let auto_launch = app_handle.autolaunch();
    
    if enable {
        auto_launch.enable().map_err(|e| e.to_string())?;
    } else {
        auto_launch.disable().map_err(|e| e.to_string())?;
    }
    
    Ok(())
}

#[derive(Debug, Deserialize)]
struct RomMetadata {
    name: String,
    size: u64,
    last_modified: u64,
    path: String,
}

#[tauri::command]
async fn download_rom(url: String, filename: String) -> Result<String, String> {
    let install_dir = get_installation_dir()?;
    let roms_dir = install_dir.join("roms");
    fs::create_dir_all(&roms_dir).map_err(|e| format!("Failed to create roms directory: {}", e))?;
    
    let filepath = roms_dir.join(&filename);
    println!("Starting ROM download to: {:?}", filepath);
    
    // Create a new client for the download
    let client = Client::new();
    
    // Create the download request
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("Failed to start download: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Download failed with status: {}", response.status()));
    }

    let total_size = response.content_length().unwrap_or(0);
    println!("Expected file size: {} bytes", total_size);

    // Create the output file
    let mut file = File::create(&filepath)
        .map_err(|e| format!("Failed to create file: {}", e))?;

    // Stream the download
    let mut downloaded: u64 = 0;
    let mut last_percentage = 0;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("Error during download: {}", e))?;
        file.write_all(&chunk)
            .map_err(|e| format!("Failed to write to file: {}", e))?;
        
        downloaded += chunk.len() as u64;
        
        // Only log every 10% progress
        if total_size > 0 {
            let percentage = (downloaded * 100 / total_size) as u8;
            if percentage >= last_percentage + 10 {
                println!("Download progress: {}%", percentage);
                last_percentage = percentage;
            }
        }
    }

    // Verify the download
    let final_size = fs::metadata(&filepath)
        .map_err(|e| format!("Failed to get file metadata: {}", e))?
        .len();

    if total_size > 0 && final_size != total_size {
        fs::remove_file(&filepath).ok();
        return Err(format!(
            "Download incomplete. Expected {} bytes but got {} bytes",
            total_size, final_size
        ));
    }

    println!("Download completed successfully. Final size: {} bytes", final_size);
    Ok(filepath.to_string_lossy().into_owned())
}

#[tauri::command]
fn is_rom_downloaded(filename: String) -> Result<bool, String> {
    let install_dir = get_installation_dir()?;
    let rom_path = install_dir.join("roms").join(filename);
    Ok(rom_path.exists())
}

#[tauri::command]
fn delete_rom(filename: String) -> Result<(), String> {
    let install_dir = get_installation_dir()?;
    let rom_path = install_dir.join("roms").join(filename);
    
    if rom_path.exists() {
        fs::remove_file(rom_path)
            .map_err(|e| format!("Failed to delete ROM: {}", e))?;
    }
    
    Ok(())
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--flag1", "--flag2"])))
        .invoke_handler(tauri::generate_handler![
            download_emulator,
            uninstall_emulator,
            check_installations,
            run_emulator,
            get_install_path,
            save_settings,
            load_settings,
            toggle_auto_launch,
            download_rom,
            is_rom_downloaded,
            delete_rom
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
