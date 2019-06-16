use super::{Color, Figure, FigureType, Point};

#[derive(Debug, Clone, PartialEq)]
pub struct ActiveFigure {
    figure: Figure,
    position: Point,
    rotation_step: usize,
}

impl ActiveFigure {
    pub fn new(figure_type: FigureType, position: Point) -> ActiveFigure {
        return ActiveFigure {
            figure: Figure::new(figure_type),
            position,
            rotation_step: 0,
        };
    }

    pub fn to_cartesian(&self) -> Vec<Point> {
        let figure_points = self.figure.to_cartesian();
        let (dx, dy) = (self.position.x, self.position.y);

        return figure_points
            .iter()
            .map(|point| Point {
                x: point.x + dx,
                y: point.y + dy,
            })
            .collect();
    }

    pub fn color(&self) -> Color {
        return self.figure.color();
    }

    pub fn position(&self) -> Point {
        return self.position;
    }

    pub fn get_type(&self) -> FigureType {
        return self.figure.get_type();
    }

    pub fn left_edge(&self) -> i32 {
        let points = self.to_cartesian();
        return points.iter().fold(i32::max_value(), |edge, point| {
            if point.x < edge {
                return point.x;
            }
            return edge;
        });
    }

    pub fn right_edge(&self) -> i32 {
        let points = self.to_cartesian();
        return points.iter().fold(i32::min_value(), |edge, point| {
            if point.x > edge {
                return point.x;
            }
            return edge;
        });
    }

    pub fn bottom_edge(&self) -> i32 {
        let points = self.to_cartesian();
        return points.iter().fold(i32::min_value(), |edge, point| {
            if point.y > edge {
                return point.y;
            }
            return edge;
        });
    }

    pub fn rotated(&self) -> ActiveFigure {
        let figure = self.figure.rotated();
        return ActiveFigure {
            figure,
            position: self.position,
            rotation_step: self.next_rotation_step(),
        };
    }

    pub fn moved_down(&self) -> ActiveFigure {
        return self.updating_position_by_xy(0, 1);
    }

    pub fn moved_left(&self) -> ActiveFigure {
        return self.updating_position_by_xy(-1, 0);
    }

    pub fn moved_right(&self) -> ActiveFigure {
        return self.updating_position_by_xy(1, 0);
    }

    pub fn wall_kicked_rotation_tests(&self) -> Vec<ActiveFigure> {
        return self
            .wall_kick_tests()
            .iter()
            .map(|point| self.updating_position_by_xy(point.x, point.y).rotated())
            .collect();
    }

    fn wall_kick_tests(&self) -> Vec<Point> {
        let kick_wall_tests_matrix = self.figure.wall_kick_tests();
        return kick_wall_tests_matrix[self.rotation_step].clone();
    }

    fn next_rotation_step(&self) -> usize {
        match self.get_type() {
            FigureType::O => 0,
            _ => {
                let next_step = self.rotation_step + 1;
                if next_step > 3 {
                    return 0;
                }
                return next_step;
            }
        }
    }

    fn updating_position_by_xy(&self, x: i32, y: i32) -> ActiveFigure {
        return ActiveFigure {
            figure: self.figure.clone(),
            position: Point {
                x: self.position().x + x,
                y: self.position().y + y,
            },
            ..*self
        };
    }
}

#[cfg(test)]
mod active_figure_tests {
    use super::*;
    #[test]
    fn test_to_cartesian_shifted() {
        let figure = ActiveFigure::new(FigureType::O, Point { x: 5, y: 5 });
        let coordinates = figure.to_cartesian();
        let expectation = vec![
            Point { x: 5, y: 5 },
            Point { x: 6, y: 5 },
            Point { x: 5, y: 6 },
            Point { x: 6, y: 6 },
        ];
        assert_eq!(coordinates, expectation);
    }
    #[test]
    fn test_color() {
        let figure_type = FigureType::T;
        let figure = ActiveFigure::new(FigureType::T, Point { x: 0, y: 0 });
        assert_eq!(figure.color(), figure_type.color());
    }
    #[test]
    fn test_update_position() {
        let figure = ActiveFigure::new(FigureType::L, Point { x: 0, y: 0 });
        let moved = figure.updating_position_by_xy(5, 5);
        assert_eq!(moved.position(), Point { x: 5, y: 5 });
    }
    #[test]
    fn test_left_edge() {
        let figure = ActiveFigure::new(FigureType::L, Point { x: 2, y: 2 });
        let edge = figure.left_edge();
        assert_eq!(edge, 2);
        let rotated_edge = figure.rotated().left_edge();
        assert_eq!(rotated_edge, 3);
    }
    #[test]
    fn test_right_edge() {
        let figure = ActiveFigure::new(FigureType::I, Point { x: 2, y: 2 });
        let edge = figure.right_edge();
        assert_eq!(edge, 5);
        let rotated_edge = figure.rotated().right_edge();
        assert_eq!(rotated_edge, 4);
    }
    #[test]
    fn test_right_edge_l_horizontal() {
        let figure = ActiveFigure::new(FigureType::I, Point { x: 17, y: 2 });
        let edge = figure.right_edge();
        assert_eq!(edge, 20);
    }
    #[test]
    fn test_bottom_edge() {
        let figure = ActiveFigure::new(FigureType::I, Point { x: 2, y: 2 });
        let edge = figure.bottom_edge();
        assert_eq!(edge, 3);
        let rotated_edge = figure.rotated().bottom_edge();
        assert_eq!(rotated_edge, 5);
    }
    #[test]
    fn test_rotation_steps_o_figure() {
        let figure = ActiveFigure::new(FigureType::O, Point { x: 0, y: 0 });
        let rotated_01 = figure.rotated();
        let rotated_02 = rotated_01.rotated();
        assert_eq!(rotated_01.rotation_step, 0);
        assert_eq!(rotated_02.rotation_step, 0);
    }
    #[test]
    fn test_totation_steps_non_o_figures() {
        let figure = ActiveFigure::new(FigureType::I, Point { x: 0, y: 0 });
        let rotation_01 = figure.rotated();
        let rotation_02 = rotation_01.rotated();
        let rotation_03 = rotation_02.rotated();
        let rotation_04 = rotation_03.rotated();

        assert_eq!(figure.rotation_step, 0);
        assert_eq!(rotation_01.rotation_step, 1);
        assert_eq!(rotation_02.rotation_step, 2);
        assert_eq!(rotation_03.rotation_step, 3);
        assert_eq!(rotation_04.rotation_step, 0);
    }
    #[test]
    fn test_moved_left() {
        let figure = ActiveFigure::new(FigureType::I, Point { x: 1, y: 0 });
        assert_eq!(figure.moved_left().position(), Point { x: 0, y: 0 });
    }
    #[test]
    fn test_moved_right() {
        let figure = ActiveFigure::new(FigureType::I, Point { x: 1, y: 0 });
        assert_eq!(figure.moved_right().position(), Point { x: 2, y: 0 });
    }
    #[test]
    fn test_moved_down() {
        let figure = ActiveFigure::new(FigureType::I, Point { x: 1, y: 0 });
        assert_eq!(figure.moved_down().position(), Point { x: 1, y: 1 });
    }
}
