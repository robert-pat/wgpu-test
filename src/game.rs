#![allow(unused)]

use std::fmt::Formatter;

pub enum GameError{
    CellDoesntExist
}
#[derive(Clone, Copy)]
pub enum CellStatus{
    Alive,
    Dead
}
impl std::fmt::Display for CellStatus{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {CellStatus::Alive => '☑', CellStatus::Dead => '☒'})
    }
}

pub struct GameBoard {
    pub board: Vec<Vec<CellStatus>>,
    pub x_max: usize,
    pub y_max: usize
}
impl GameBoard {
    pub fn new(x: usize, y: usize) -> Self{
        Self{
            board: vec![vec![CellStatus::Dead; x]; y],
            x_max: x,
            y_max: y
        }
    }

    pub fn get(&self, x: usize, y:usize) -> CellStatus{
        self.board[y][x]
    }

    pub fn set(&mut self, x: usize, y: usize, status: CellStatus){
        self.board[y][x] = status;
    }
}
impl Default for GameBoard {
    fn default() -> Self {
        GameBoard::new(10, 10)
    }
}