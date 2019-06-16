use super::{FigureType, Matrix, Point, Size};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Board {
    matrix: Matrix<Option<FigureType>>,
}

impl Board {
    pub fn new(size: &Size) -> Board {
        let mut cells = vec![];
        for _y in 0..size.height {
            let line = Board::get_empty_line(size.width);
            cells.push(line);
        }
        let matrix = Matrix::new(cells);
        return Board { matrix };
    }

    fn get_empty_line(width: usize) -> Vec<Option<FigureType>> {
        let mut line: Vec<Option<FigureType>> = vec![];
        for _x in 0..width {
            line.push(None);
        }
        return line;
    }

    pub fn height(&self) -> usize {
        return self.matrix.height();
    }

    pub fn width(&self) -> usize {
        return self.matrix.width();
    }

    pub fn figure_at_xy(&self, x: usize, y: usize) -> &Option<FigureType> {
        if let Some(element) = self.matrix.at_xy(x, y) {
            return element;
        } else {
            return &None;
        }
    }

    pub fn replacing_figure_at_xy(
        &self,
        x: usize,
        y: usize,
        figure_type: Option<FigureType>,
    ) -> Board {
        let matrix = self.matrix.replacing_at_xy(x, y, figure_type);
        return Board { matrix };
    }

    pub fn contains(&self, point: Point) -> bool {
        if point.x < 0 || point.y < 0 {
            return false;
        }
        return self
            .figure_at_xy(point.x as usize, point.y as usize)
            .is_some();
    }

    pub fn get_line(&self, line: usize) -> Option<&Vec<Option<FigureType>>> {
        return self.matrix.row_at(line);
    }

    pub fn removing_lines(&self, lines: &Vec<usize>) -> Board {
        let mut new_board_data: VecDeque<Vec<Option<FigureType>>> = VecDeque::default();
        for line_number in 0..self.height() {
            if lines.contains(&line_number) {
                new_board_data.push_front(Board::get_empty_line(self.width()));
            } else {
                if let Some(line) = self.get_line(line_number) {
                    new_board_data.push_back(line.clone());
                }
            }
        }
        return Board {
            matrix: Matrix::new(Vec::from(new_board_data)),
        };
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;

    #[test]
    fn test_height() {
        let height = 10;
        let board = Board::new(&Size { height, width: 10 });
        assert_eq!(board.height(), height as usize);
    }
    #[test]
    fn test_width() {
        let width = 10;
        let board = Board::new(&Size { height: 10, width });
        assert_eq!(board.width(), width);
    }
    #[test]
    fn test_replacing_figure() {
        let board = Board::new(&Size {
            height: 2,
            width: 1,
        });
        let replaced_board = board.replacing_figure_at_xy(0, 0, Some(FigureType::I));
        assert!(replaced_board.matrix.at_xy(0, 0).is_some());
    }
    #[test]
    fn test_does_not_contains() {
        let board = Board::new(&Size {
            height: 4,
            width: 4,
        });
        assert!(!board.contains(Point { x: 0, y: 0 }));
        assert!(!board.contains(Point { x: 100, y: 100 }));
        let board_with_figure = board.replacing_figure_at_xy(0, 0, Some(FigureType::I));
        assert!(board_with_figure.contains(Point { x: 0, y: 0 }));
    }
    #[test]
    fn test_contains() {
        let board = Board::new(&Size {
            height: 4,
            width: 4,
        });
        let board_with_figure = board.replacing_figure_at_xy(0, 0, Some(FigureType::I));
        assert!(board_with_figure.contains(Point { x: 0, y: 0 }));
    }
    #[test]
    fn test_removing_lines() {
        let board = Board::new(&Size {
            height: 4,
            width: 1,
        });
        let board_02 = board.replacing_figure_at_xy(0, 0, Some(FigureType::I));
        let board_03 = board_02.replacing_figure_at_xy(0, 3, Some(FigureType::I));
        let final_board = board_03.removing_lines(&vec![3]);

        let expectation = Matrix::new(vec![
            vec![None],
            vec![Some(FigureType::I)],
            vec![None],
            vec![None],
        ]);

        assert_eq!(final_board.matrix, expectation);

        let final_board_02 = board_03.removing_lines(&vec![0, 3]);
        let expectation_02 = Matrix::new(vec![vec![None], vec![None], vec![None], vec![None]]);

        assert_eq!(final_board_02.matrix, expectation_02);
    }
}
