use crate::get_timestamp;

#[cfg(not(target_arch = "wasm32"))]
use std::fs::{self, File, OpenOptions};
#[cfg(not(target_arch = "wasm32"))]
use std::io::{Read, Write};
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
use directories::BaseDirs;
use eframe::egui::Color32;
#[cfg(not(target_arch = "wasm32"))]
use serde::{Serialize, Deserialize};

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Clone)]
pub struct Colors {
    pub cell_alive: Color32,
    pub cell_dead: Color32
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
pub struct Colors {
    pub cell_alive: Color32,
    pub cell_dead: Color32
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Appearance {
    pub cell_size: f32,
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
pub struct Appearance {
    pub cell_size: f32,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub appearance: Appearance,
    pub b: String,
    pub colors: Colors,
    pub gif_cell_upscale: usize,
    pub s: String,
    pub size: usize,
    pub speed: u8,
    pub randomize_factor: u8,
    pub timer: u128,
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
pub struct Config {
    pub appearance: Appearance,
    pub b: String,
    pub colors: Colors,
    pub gif_cell_upscale: usize,
    pub s: String,
    pub size: usize,
    pub speed: u8,
    pub randomize_factor: u8,
    pub timer: u128,
}

impl Config {
    fn default_config() -> Self {
        Self {
            appearance: Appearance {
                cell_size: 20.,
            },
            b: "3".to_string(),
            colors: Colors {
                cell_alive: Color32::from_rgb(207, 25, 56),
                cell_dead:  Color32::from_rgb( 63, 59, 59),
            },
            gif_cell_upscale: 15,
            s: "23".to_string(),
            size: 40,
            speed: 25,
            randomize_factor: 35,
            timer: get_timestamp(),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn try_get_config() -> anyhow::Result<Self> {
        if let Some(base_dirs) = BaseDirs::new() {
            let mut config_path = PathBuf::from(&base_dirs.config_dir());
            config_path.push("gol");

            fs::create_dir_all(&config_path)?;

            config_path.push("config.json");


            match fs::metadata(&config_path) {
                Ok(_) => {
                    let mut config_file = File::open(config_path)?;
                    let mut config_content = String::new();
                    config_file.read_to_string(&mut config_content)?;

                    let cfg: Config = serde_json::from_str(&config_content)?;

                    return Ok(cfg);
                },
                Err(_) => {
                    let ser_config = serde_json::to_string(&Self::default_config())?;
                    let mut file = File::create(config_path)?;
                    file.write_all(ser_config.as_bytes())?;

                    return Ok(Self::default_config());
                }
            }
        } else {
            return Ok(Self::default_config());
        }
    }

    pub fn handle_config() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        match Self::try_get_config() {
            Ok(cfg) => return cfg,
            Err(_)  => return Self::default_config(),
        }

        return Self::default_config();
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn try_save(&self) -> anyhow::Result<()> {
        if cfg!(target_arch = "wasm32") {
            return Ok(());
        }

        if let Some(base_dirs) = BaseDirs::new() {
            let mut config_path = PathBuf::from(&base_dirs.config_dir());
            config_path.push("gol");
            config_path.push("config.json");
            
            match fs::metadata(&config_path) {
                Ok(r) => {
                    if r.is_file() {
                        let ser_config = serde_json::to_string(&self)?;
                        let mut config_file = OpenOptions::new()
                            .write(true)
                            .open(config_path)
                            .unwrap();
                        config_file.write_all(ser_config.as_bytes())?;
                        config_file.flush()?;
                    }
                },
                Err(_) => ()
            }
        }

        println!("[GoL] Config saved");
        Ok(())
    }
}
