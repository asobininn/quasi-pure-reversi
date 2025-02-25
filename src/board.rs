use std::f32::consts::PI;
use std::ops::ControlFlow::*;
use std::{cell::RefCell, fmt::Display, rc::Rc};

pub const BOARD_SIZE: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Black,
    White,
}

impl Cell {
    pub fn flip(self) -> Self {
        match self {
            Cell::Empty => Cell::Empty,
            Cell::Black => Cell::White,
            Cell::White => Cell::Black,
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Black => write!(f, "●"),
            Cell::White => write!(f, "○"),
        }
    }
}

pub type Board = Rc<RefCell<[[Cell; BOARD_SIZE]; BOARD_SIZE]>>;
pub type Coord = (usize, usize); // x, y

pub trait BoardOps {
    #[must_use]
    fn put(self, coord: Coord, cell: Cell) -> Self;
    fn coords_to_cells(&self, coords: &[Coord]) -> Vec<Cell>;
    fn count_flippable(&self, cell: Cell, cells: Vec<Cell>) -> usize;
    fn get_flippable_coords(&self, cell: Cell, pos: Coord) -> Vec<Coord>;
    fn get_putable_coords(&self, cell: Cell) -> Vec<(Coord, Vec<Coord>)>;
}

impl BoardOps for Board {
    fn put(self, coord: Coord, cell: Cell) -> Self {
        self.borrow_mut()[coord.1][coord.0] = cell;
        self
    }

    fn coords_to_cells(&self, coords: &[Coord]) -> Vec<Cell> {
        coords.iter().map(|(x, y)| self.borrow()[*y][*x]).collect()
    }

    fn count_flippable(&self, cell: Cell, cells: Vec<Cell>) -> usize {
        let target = cell.flip();
        let len = cells.iter().take_while(|&next| &target == next).count();
        if let Some(&end) = cells.get(len) {
            if end == cell { len } else { 0 }
        } else {
            0
        }
    }

    fn get_flippable_coords(&self, cell: Cell, pos: Coord) -> Vec<Coord> {
        let dirs: Vec<(isize, isize)> = (0..8)
            .map(|n| {
                (
                    (n as f32 * PI / 4.).cos().round() as isize,
                    (n as f32 * PI / 4.).sin().round() as isize,
                )
            })
            .collect();
        dirs.into_iter()
            .flat_map(|dir| {
                let coords = get_coords_to_edge((pos.0 as isize, pos.1 as isize), dir);
                let len = self.count_flippable(cell, self.coords_to_cells(&coords));
                coords.into_iter().take(len).collect::<Vec<_>>()
            })
            .collect()
    }

    fn get_putable_coords(&self, cell: Cell) -> Vec<(Coord, Vec<Coord>)> {
        self.borrow()
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, cell)| {
                    if cell == &Cell::Empty {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .filter_map(|pos| {
                let vec = self.get_flippable_coords(cell, pos);
                if !vec.is_empty() {
                    Some((pos, vec))
                } else {
                    None
                }
            })
            .collect()
    }
}

pub fn get_coords_to_edge(coord: (isize, isize), dir: (isize, isize)) -> Vec<Coord> {
    std::iter::repeat(())
        .try_fold((Vec::new(), coord), |(mut acc, coord), ()| {
            if 0 <= coord.0
                && coord.0 < BOARD_SIZE as isize
                && 0 <= coord.1
                && coord.1 < BOARD_SIZE as isize
            {
                acc.push(coord);
                Continue((acc, (coord.0 + dir.0, coord.1 + dir.1)))
            } else {
                Break(acc)
            }
        })
        .break_value()
        .unwrap()
        .into_iter()
        .skip(1)
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}
