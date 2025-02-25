use std::cmp::Ordering;
use std::ops::ControlFlow::*;
use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::board::*;

pub type Putable = Vec<((usize, usize), Vec<(usize, usize)>)>;
type MoveHistorys = Rc<RefCell<Vec<MoveHistory>>>;

#[derive(Debug)]
pub enum MoveHistory {
    Put(Coord, Vec<Coord>),
    Pass,
}

impl MoveHistory {
    pub fn is_pass(&self) -> bool {
        matches!(self, MoveHistory::Pass)
    }

    pub fn put_value(&self) -> Option<(Coord, Vec<Coord>)> {
        if let MoveHistory::Put(put, flipped) = self {
            Some((*put, flipped.to_vec()))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub enum Action {
    Put(Coord),
    Undo,
}

#[derive(Debug, Clone)]
pub struct GameManager {
    board: Board,
    move_history: MoveHistorys,
}

impl GameManager {
    #[must_use]
    pub fn new() -> Self {
        let mid = BOARD_SIZE / 2;
        let board = Rc::new(RefCell::new([[Cell::Empty; BOARD_SIZE]; BOARD_SIZE]))
            .put((mid - 1, mid - 1), Cell::White)
            .put((mid, mid), Cell::White)
            .put((mid - 1, mid), Cell::Black)
            .put((mid, mid - 1), Cell::Black);
        Self {
            board,
            move_history: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn now_hand(&self) -> Cell {
        if self.move_history.borrow().len() % 2 == 0 {
            Cell::Black
        } else {
            Cell::White
        }
    }

    #[must_use]
    fn undo(self) -> Self {
        std::iter::repeat(())
            .try_fold(self, move |this, ()| {
                let mut history_mut = this.move_history.borrow_mut();
                if let Some(history) = history_mut.pop() {
                    drop(history_mut);
                    if history.is_pass() {
                        Continue(this)
                    } else {
                        let cell = this.now_hand();
                        let (put_place, flipped) = history.put_value().unwrap();
                        let _ = flipped
                            .into_iter()
                            .fold(this.board.clone(), |board, pos| board.put(pos, cell.flip()))
                            .put(put_place, Cell::Empty);
                        Break(this)
                    }
                } else {
                    drop(history_mut);
                    Break(this)
                }
            })
            .break_value()
            .unwrap()
    }

    fn display(&self, cell: Cell, putables: &Putable) {
        println!("{self}");
        println!("{}'s turn.", cell);
        putables.iter().for_each(|i| {
            println!("{:?}", i.0);
        });
    }

    #[must_use]
    pub fn game_loop<F: Fn(&Putable) -> Action>(self, black_hand: F, white_hand: F) -> Self {
        std::iter::repeat(())
            .try_fold(self, move |this, ()| {
                let cell = this.now_hand();
                let putables = this.board.get_putable_coords(cell);
                if putables.is_empty() {
                    let history_ref = this.move_history.borrow();
                    let prev = history_ref.last();
                    if prev.is_some() && prev.unwrap().is_pass() {
                        drop(history_ref);
                        this.move_history.borrow_mut().push(MoveHistory::Pass);
                        Break(this)
                    } else {
                        drop(history_ref);
                        this.move_history.borrow_mut().push(MoveHistory::Pass);
                        Continue(this)
                    }
                } else {
                    this.display(cell, &putables);
                    let hand = if cell == Cell::Black {
                        &black_hand
                    } else {
                        &white_hand
                    };
                    match hand(&putables) {
                        Action::Put(coord) => {
                            if let Some((pos, flippable)) =
                                putables.iter().find(|(putable, _)| putable == &coord)
                            {
                                let _ = flippable
                                    .iter()
                                    .fold(this.board.clone(), |board, pos| board.put(*pos, cell))
                                    .put(*pos, cell);
                                this.move_history
                                    .borrow_mut()
                                    .push(MoveHistory::Put(*pos, flippable.to_vec()));
                                Continue(this)
                            } else {
                                println!("Cant' put there.");
                                Continue(this)
                            }
                        }
                        Action::Undo => Continue(this.undo()),
                    }
                }
            })
            .break_value()
            .unwrap()
    }

    fn count_cells(&self) -> (usize, usize) {
        self.board
            .borrow()
            .iter()
            .flat_map(|line| line.iter())
            .fold((0, 0), |(black, white), cell| match cell {
                Cell::Empty => (black, white),
                Cell::Black => (black + 1, white),
                Cell::White => (black, white + 1),
            })
    }

    pub fn show_result(self) {
        println!("{self}");
        let (black, white) = self.count_cells();
        match black.cmp(&white) {
            Ordering::Equal => println!("Draw!"),
            Ordering::Less => println!("White Win!"),
            Ordering::Greater => println!("Black Win!"),
        }
        println!("black: {black}");
        println!("white: {white}");
    }
}

impl Display for GameManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let labels: String = format!(
            "  {}",
            ('a'..='h')
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
        let board_ref = self.board.borrow();
        let board_rows: String = board_ref
            .iter()
            .enumerate()
            .map(|(i, line)| {
                format!(
                    "{} {}",
                    i + 1,
                    line.map(|cell| cell.to_string())
                        .into_iter()
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{labels}\n{board_rows}")
    }
}
