use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use engine::{
    board::{Board, WIDTH},
    engine::Engine,
    opening_book::OpeningBook,
};
use tauri::State;

struct Game {
    engine: Engine,
    encoded_board: String,
}

impl Game {
    fn new() -> Self {
        Self {
            engine: Engine::new(),
            encoded_board: String::new(),
        }
    }
}

type AppState<'a> = State<'a, Arc<Mutex<Game>>>;

#[tauri::command]
fn get_encoded_board(state: AppState) -> String {
    let game = state.lock().unwrap();
    game.encoded_board.clone()
}

#[tauri::command]
fn open_book(book_path: String, state: AppState) -> Result<(), String> {
    let mut game = state.lock().unwrap();
    let book = OpeningBook::open(&book_path).map_err(|err| err.to_string())?;
    game.engine = Engine::with_book(book);
    Ok(())
}

#[tauri::command]
fn play_colm(colm: usize, state: AppState) -> Result<(), String> {
    let mut game = state.lock().unwrap();

    let colm = char::from_digit(colm as u32, 10).ok_or("Invalid column digit")?;
    game.encoded_board.push(colm);

    match Board::from_str(&game.encoded_board) {
        Ok(_) => Ok(()),
        Err(err) => {
            game.encoded_board.pop();
            Err(err.to_string())
        }
    }
}

#[tauri::command]
fn reset_game(state: AppState) {
    let mut game = state.lock().unwrap();
    game.engine.reset();
    game.encoded_board.clear();
}

#[tauri::command]
fn columns_score(state: AppState) -> [Option<i32>; WIDTH] {
    let mut game = state.lock().unwrap();
    let board = game.encoded_board.parse().unwrap();
    game.engine.solve(board)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(Arc::new(Mutex::new(Game::new())))
        .invoke_handler(tauri::generate_handler![
            get_encoded_board,
            open_book,
            play_colm,
            reset_game,
            columns_score
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
