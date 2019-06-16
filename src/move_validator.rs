use super::{ActiveFigure, Board};

pub fn has_valid_position(active_figure: &ActiveFigure, board: &Board) -> bool {
    return !will_colide_with_block(active_figure, board)
        && !will_collide_with_edge(active_figure, board);
}

pub fn can_move_down(figure: &ActiveFigure, board: &Board) -> bool {
    let moved_down = figure.moved_down();
    return !is_at_the_bottom(figure, board) && !will_colide_with_block(&moved_down, board);
}

fn will_colide_with_block(figure: &ActiveFigure, board: &Board) -> bool {
    let points = figure.to_cartesian();
    for point in points {
        if board.contains(point) {
            return true;
        }
    }
    return false;
}

fn will_collide_with_edge(active_figure: &ActiveFigure, board: &Board) -> bool {
    let collided_with_left = active_figure.left_edge() < 0;
    let collided_with_right = active_figure.right_edge() >= board.width() as i32;
    let collided_with_bottom = active_figure.bottom_edge() >= board.height() as i32;
    return collided_with_left || collided_with_right || collided_with_bottom;
}

fn is_at_the_bottom(figure: &ActiveFigure, board: &Board) -> bool {
    return figure.bottom_edge() == (board.height() as i32 - 1);
}

#[cfg(test)]
mod move_validator_tests {
    use super::super::FigureType;
    use super::*;
    use super::super::geometry::{Point, Size};
    use ActiveFigure;

    #[test]
    fn test_is_at_the_bottom() {
        let board = Board::new(&Size {
            height: 10,
            width: 10,
        });
        let figure = ActiveFigure::new(FigureType::L, Point { x: 3, y: 8 });
        assert!(is_at_the_bottom(&figure, &board));
    }

    #[test]
    fn test_is_at_the_bottom_return_false() {
        let board = Board::new(&Size {
            height: 10,
            width: 10,
        });
        let figure = ActiveFigure::new(FigureType::L, Point { x: 3, y: 7 });
        assert!(!is_at_the_bottom(&figure, &board));
    }
    #[test]
    fn test_will_colide_with_block() {
        let mut board = Board::new(&Size {
            height: 10,
            width: 10,
        });

        board = board.replacing_figure_at_xy(0, 3, Some(FigureType::T));
        board = board.replacing_figure_at_xy(1, 3, Some(FigureType::T));
        board = board.replacing_figure_at_xy(2, 3, Some(FigureType::T));
        board = board.replacing_figure_at_xy(3, 3, Some(FigureType::T));

        let colider = ActiveFigure::new(FigureType::I, Point { x: 0, y: 0 });
        let rotated = colider.rotated();

        assert!(!will_colide_with_block(&colider, &board));
        assert!(will_colide_with_block(&rotated, &board));
    }
}
