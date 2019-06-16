mod active_figure;
mod board;
mod figure;
mod game;
use active_figure::ActiveFigure;
use board::Board;
use figure::{Figure, FigureType, Matrix, graphics, geometry, block};
pub use game::{Game, Randomizer};
use block::Block;
use geometry::*;
use graphics::Color;

mod move_validator;
