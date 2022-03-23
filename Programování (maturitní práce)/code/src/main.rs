// neukazovat konzoli při spouštění na Windows 
#![windows_subsystem = "windows"]

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use eframe::{run_native, NativeOptions};
    use gol::Game;
    
    tracing_subscriber::fmt::init();

    let game = Game::new();
    
    let win_option = NativeOptions::default();
    run_native(Box::new(game), win_option);
}

#[cfg(target_arch = "wasm32")]
fn main() {}
