#![allow(unused)]

use std::fmt::Formatter;

pub enum GameError {
    CellDoesntExist,
}
#[derive(Clone, Copy)]
pub enum CellStatus {
    Alive,
    Dead,
}
impl From<CellStatus> for char {
    fn from(value: CellStatus) -> Self {
        match value {
            CellStatus::Alive => '☑',
            CellStatus::Dead => '☒',
        }
    }
}
impl std::fmt::Display for CellStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <CellStatus as Into<char>>::into(*self))
    }
}

pub struct GameBoard {
    board: Vec<Vec<CellStatus>>,
    x_max: usize,
    y_max: usize,
}
impl GameBoard {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            board: vec![vec![CellStatus::Dead; x]; y],
            x_max: x,
            y_max: y,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> CellStatus {
        self.board[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, status: CellStatus) {
        self.board[y][x] = status;
    }
}
impl Default for GameBoard {
    fn default() -> Self {
        GameBoard::new(10, 10)
    }
}
