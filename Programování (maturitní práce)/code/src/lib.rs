// import lokálních souborů
mod config;
use config::Config;

// import z std knihovny
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
use directories::UserDirs;

// import komunitních beden
use eframe::{
    egui::{self, vec2, Color32, Id, Pos2, Rect, Stroke, Ui},
    epi::App,
};
use thiserror::Error;

#[cfg(not(target_arch = "wasm32"))]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(target_arch = "wasm32")]
use wasm_timer::{SystemTime, UNIX_EPOCH};


// ad hoc error typ
#[derive(Error, Debug)]
pub enum MyError {
	#[error("")]
	Dummy,
}

// objekt buňky
#[derive(Default, Clone)]
struct Cell {
    pub is_alive: bool,
}

// herní objekt
#[derive(Clone)]
pub struct Game {
    pub config: Config,
    cells: Vec<Vec<Cell>>,
    is_running: bool,
    is_recording: bool,
    recording_buffer: Vec<Vec<u8>>,
}

impl Game {
    // --------------------------------------
    //      Tvorba
    // --------------------------------------


    // pomocná funkce pro generování reprezentace pole s buňkami
    fn generate_board(size: usize) -> Vec<Vec<Cell>> {
        (0..size)
            .map(|_x| (0..size).map(|_x| Cell::default()).collect())
            .collect()
    }

    // vytvoří instanci `Game`
    pub fn new() -> Game {
        let config = Config::handle_config();

        Game {
            cells: Game::generate_board(config.size),
            config,
            is_running: false,
            is_recording: false,
            recording_buffer: Vec::new(),
        }
    }


    // --------------------------------------
    //      Manipulace
    // --------------------------------------


    // umožní změnit velikost hrací desky do spec. velikosti
    pub fn resize_to(&mut self, size: usize) {
        use std::cmp::Ordering;

        match size.cmp(&self.cells[0].len()) {
            Ordering::Greater => {
                for _ in 0..(size - self.cells[0].len()) {
                    for i in 0..self.cells[0].len() {
                        self.cells[i].push(Cell::default());
                    }

                    self.cells
                        .push((0..size).map(|_x| Cell::default()).collect());
                }
            }
            Ordering::Less => {
                for _ in 0..(self.cells[0].len() - size) {
                    for i in 0..self.cells[0].len() {
                        self.cells[i].pop();
                    }

                    self.cells.pop();
                }
            }
            Ordering::Equal => (),
        }
    }

    // nastaví náhodný stav buněk (v závislosti na daném faktoru 0-100)
    #[cfg(not(target_arch = "wasm32"))]
    pub fn randomize_cells(&mut self, factor: u8) {
        use rand::Rng;

        for i in 0..self.config.size {
            for j in 0..self.config.size {
                self.cells[i as usize][j as usize].is_alive =
                    rand::thread_rng().gen_range(0..101) < factor;
            }
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn randomize_cells(&mut self, factor: u8) {
        // use getryandom::getrandom;

        for i in 0..self.config.size {
            for j in 0..self.config.size {
                let mut x = [0u8; 1];
                // getrandom(&mut x).unwrap();

                self.cells[i as usize][j as usize].is_alive =
                    (false);
            }
        }
    }

    // změní stav specifikované buňky
    pub fn change_cell_state(&mut self, cell: (usize, usize)) {
        self.cells[cell.0 as usize][cell.1 as usize].is_alive =
            !self.cells[cell.0 as usize][cell.1 as usize].is_alive;
    }

    // vrátí pozici (ne)kliknuté buňky
    fn get_clicked_cell(&self, pos: Pos2, rect: Rect) -> Option<(usize, usize)> {
        let cell: (usize, usize) = (
            (((pos.x - rect.left()) / self.config.appearance.cell_size).floor() as usize),
            (((pos.y - rect.top()) / self.config.appearance.cell_size).floor() as usize),
        );

        if cell.0 < self.config.size && cell.1 < self.config.size {
            return Some(cell);
        }

        None
    }


    // --------------------------------------
    //      Herní logika
    // --------------------------------------


    // získá počet živých sousedů buňky
    fn get_neighbor_count(&self, pos: (i32, i32)) -> u8 {
        let mut count: u8 = 0;

        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {
                    continue;
                }

                let x = pos.0 + i;
                let y = pos.1 + j;

                if x >= 0 && x < self.config.size as i32 && y >= 0 && y < self.config.size as i32 {
                    if self.cells[x as usize][y as usize].is_alive {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    // vytvoří novou desku na základě pravidel
    pub fn iterate(&mut self) {
        let mut future_board = Game::generate_board(self.config.size);
        let parsed_s: Vec<u8> = self.config.s.chars().map(|x| x as u8 - '0' as u8).collect();
        let parsed_b: Vec<u8> = self.config.b.chars().map(|x| x as u8 - '0' as u8).collect();

        for i in 0..self.config.size {
            for j in 0..self.config.size {
                let count = Game::get_neighbor_count(&self, (i as i32, j as i32));

                if self.cells[i as usize][j as usize].is_alive {
                    future_board[i as usize][j as usize].is_alive = parsed_s.contains(&count);
                } else {
                    future_board[i as usize][j as usize].is_alive = parsed_b.contains(&count);
                }
            }
        }

        self.cells = future_board;
    }


    // --------------------------------------
    //      Kontroléry, komunikace s GUI
    // --------------------------------------


    // rozhodování o možnosti další iterace v závislosti na konf.
    pub fn handle_iterate(&mut self, ui: &mut Ui) {
        if self.is_running {
            let timestamp = get_timestamp();

            if timestamp - self.config.timer > 1000 / (self.config.speed as u128) {
                self.iterate();
                if self.is_recording {
                    self.add_frame_to_buffer();
                }
                
                self.config.timer = timestamp;
            }
        }

        ui.ctx().request_repaint();
    }

    // přidělí místo pro pole s buňkami
    fn allocate_space_for_cells(&self, ui: &mut Ui) -> (Id, Rect) {
        ui.allocate_space(vec2(
            ((self.config.size) as f32 * self.config.appearance.cell_size) as f32,
            ((self.config.size) as f32 * self.config.appearance.cell_size) as f32,
        ))
    }

    // vykresluje pole s buňkami
    fn show_cells(&mut self, ui: &mut Ui, rect: Rect) {
        let painter = ui.painter();

        for i in 0..self.config.size as u32 {
            for j in 0..self.config.size as u32 {
                let color: Color32 = match self.cells[i as usize][j as usize].is_alive {
                    true => self.config.colors.cell_alive,
                    false => self.config.colors.cell_dead,
                };

                painter.rect(
                    Rect::from_two_pos(
                        Pos2 {
                            x: rect.left() + i as f32 * self.config.appearance.cell_size,
                            y: rect.top() + j as f32 * self.config.appearance.cell_size,
                        },
                        Pos2 {
                            x: rect.left() + (i as f32 + 1.) * self.config.appearance.cell_size,
                            y: rect.top() + (j as f32 + 1.) * self.config.appearance.cell_size,
                        },
                    ),
                    0.,
                    color,
                    Stroke::new(0., Color32::BLACK),
                );
            }
        }
    }

    // vrátí popis pro nahrávací tlačítko
    fn get_recording_label(&mut self) -> String {
        String::from(match self.is_recording {
            true => "Zastavit nahrávání",
            false => "Začít nahrávat",
        })
    }

    // vypne/zapne nahrávání
    fn change_recording_state(&mut self) {
        self.is_recording = !self.is_recording;
    }

    // pořídí snímek a uloží ho do bufferu
    fn add_frame_to_buffer(&mut self) {
        let mut normalized_cells: Vec<u8> = Vec::new();

        for i in 0..self.config.size {
            for j in 0..self.config.size {
                normalized_cells.push(if self.cells[i][j].is_alive { 0 } else { 1 });
            }
        }

        self.recording_buffer.push(normalized_cells);
    }


    // --------------------------------------
    //      Externí soubory
    // --------------------------------------

    #[cfg(not(target_arch = "wasm32"))]
    pub fn handle_gif_save(&mut self) -> anyhow::Result<()> {
        if cfg!(target_arch = "wasm32") {
            return Ok(())
        }

        let buff = self.recording_buffer.clone();
        let cfg = self.config.clone();
        std::thread::spawn(move || {
            save_gif(buff, cfg);
        });

        self.recording_buffer.clear();

        Ok(())
    }
}

// GUI
impl App for Game {
    // vykreslování
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(15.);

            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    // komponent s konfiguracemi hry
                    ui.vertical(|ui| {
                        // generování
                        ui.group(|ui| {
                            ui.set_width(150.);

                            if ui.button("Další generace").clicked() {
                                self.iterate();
                            }

                            ui.checkbox(&mut self.is_running, "Automatická generace");
                            ui.add(egui::Slider::new(&mut self.config.speed, 5..=100));
                        });

                        ui.add_space(15.);

                        // znáhodnění stavů buněk
                        ui.group(|ui| {
                            ui.set_width(150.);

                            if ui.button("Náhodně změnit buňky dle faktoru").clicked() {
                                #[cfg(not(target_arch = "wasm32"))]
                                self.randomize_cells(self.config.randomize_factor);
                            }
                            ui.add(egui::Slider::new(&mut self.config.randomize_factor, 0..=100));
                        });

                        ui.add_space(15.);

                        // znáhodnění s/b pravidel hry
                        ui.group(|ui| {
                            ui.set_width(150.);

                            ui.label("Vlastní pravidla");

                            ui.add_space(5.);

                            ui.horizontal(|ui| {
                                ui.set_max_width(100.0);
                                ui.label("S: ");
                                ui.text_edit_singleline(&mut self.config.s);
                            });

                            ui.horizontal(|ui| {
                                ui.set_max_width(100.0);
                                ui.label("B: ");
                                ui.text_edit_singleline(&mut self.config.b);
                            });
                        });

                        ui.add_space(15.);

                        // nastavení velikosti pole
                        ui.group(|ui| {
                            ui.set_max_width(150.);

                            ui.horizontal(|ui| {
                                ui.label("Velikost: ");
                                ui.add(egui::DragValue::new(&mut self.config.size));
                            });
                        });

                        ui.add_space(15.);

                        // nahrávání
                        #[cfg(not(target_arch = "wasm32"))]
                        ui.group(|ui| {
                            ui.set_width(150.);

                            if ui.button(self.get_recording_label()).clicked() {
                                self.change_recording_state();
                            }

                            ui.add_space(5.);

                            
                            ui.horizontal(|ui| {
                                ui.label("Velikost buňky: ");
                                ui.add(egui::DragValue::new(&mut self.config.gif_cell_upscale));
                                ui.label("px");
                            });

                            if ui.button("Uložit jako GIF").clicked() {
                                self.is_recording = false;
                                self.handle_gif_save();
                            }
                        })
                    });

                    ui.add_space(15.);

                    // komponent s vykreslovanými buňkami
                    ui.group(|ui| {
                        let (id, rect) = self.allocate_space_for_cells(ui);

                        let board_interact = ui.interact(rect, id, egui::Sense::click());
                        if board_interact.clicked() {
                            if let Some(pos) = board_interact.hover_pos() {
                                let cell = self.get_clicked_cell(pos, rect);

                                if let Some(cell) = cell {
                                    self.change_cell_state(cell);
                                }
                            }
                        }

                        if self.config.size != self.cells[0].len() {
                            #[cfg(not(target_arch = "wasm32"))]
                            if self.is_recording {
                                self.is_recording = false;
                                self.handle_gif_save();
                            }
                            self.resize_to(self.config.size);
                        }

                        self.show_cells(ui, rect);

                        self.handle_iterate(ui);
                    });
                });
            });
        });

        #[cfg(not(target_arch = "wasm32"))]
        // uložit konfig každých přibl. 25 sekund
        let cfg = self.config.clone();
        #[cfg(not(target_arch = "wasm32"))]
        std::thread::spawn(move || {
            match SystemTime::now().duration_since(UNIX_EPOCH) {
                Ok(n) => {
                    if n.as_nanos() % 500 == 0 {
                        cfg.try_save().unwrap();
                    }
                },
                _ => (),
            }
        });
    }

    // název okna
    fn name(&self) -> &str {
        "Hra života v Rustu, egui"
    }
}

// uloží GIF z bufferu
#[cfg(not(target_arch = "wasm32"))]
fn save_gif(buf: Vec<Vec<u8>>, cfg: Config) -> anyhow::Result<()> {
    if cfg!(target_arch = "wasm32") {
        return Ok(());
    } else {

    use gif::{Encoder, Frame, Repeat};
    use std::borrow::Cow;
    use std::fs::File;

    let color_map = &[0xFF, 0xFF, 0xFF, 0, 0, 0];
    let size: u16 = (cfg.size * cfg.gif_cell_upscale) as u16;

    let mut file_path = get_desktop_path()?;
    file_path.push(&format!("GoL_{}.gif", get_timestamp()));

    let mut image = File::create(file_path).unwrap();
    let mut encoder = Encoder::new(&mut image, size, size, color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();

    for state in buf {
        let state = upscale_vector(&state, cfg.gif_cell_upscale, cfg.size);

        let mut frame = Frame::default();
        frame.width = size;
        frame.height = size;
        frame.buffer = Cow::Borrowed(&*state.as_slice());

        encoder.write_frame(&frame).unwrap();
    }

    Ok(())
}}

#[cfg(not(target_arch = "wasm32"))]
fn get_desktop_path<'a>() -> anyhow::Result<PathBuf> {
    Ok(UserDirs::new().ok_or(MyError::Dummy)?.desktop_dir().ok_or(MyError::Dummy)?.to_path_buf())
}

// vrátí časové razítko v milisekundách
fn get_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

// zvětší 1-rozměrný vektor
fn upscale_vector<T>(vec: &Vec<T>, factor: usize, row_size: usize) -> Vec<T>
where
    T: Copy,
{
    let mut out_vec: Vec<T> = Vec::new();

    for chunk in vec.chunks(row_size) {
        for _ in 0..factor {
            for val in chunk {
                for _ in 0..factor {
                    out_vec.push(*val);
                }
            }
        }
    }

    out_vec
}

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};


#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn main_web(canvas_id: &str) {
    extern crate console_error_panic_hook;
    use std::panic;
    tracing_wasm::set_as_global_default();
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let game = Game::new();

    eframe::start_web(canvas_id, Box::new(game));
}
