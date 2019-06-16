use super::figure_type::FigureType;
use super::matrix::Matrix;
use super::geometry::Point;
use super::graphics::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct Figure {
    figure_type: FigureType,
    matrix: Matrix<u8>,
}

impl Figure {
    pub fn new(figure_type: FigureType) -> Figure {
        let matrix = figure_type.initial_matrix();
        return Figure {
            figure_type,
            matrix,
        };
    }

    pub fn get_type(&self) -> FigureType {
        return self.figure_type.clone();
    }

    pub fn color(&self) -> Color {
        return self.figure_type.color();
    }

    pub fn wall_kick_tests(&self) -> Vec<Vec<Point>> {
        return self.figure_type.wall_kick();
    }

    pub fn rotated(&self) -> Self {
        return Figure {
            matrix: self.matrix.rotated(),
            figure_type: self.figure_type.clone(),
        };
    }

    pub fn to_cartesian(&self) -> Vec<Point> {
        let mut points = vec![];
        for y in 0..self.matrix.height() {
            for x in 0..self.matrix.width() {
                if let Some(element) = self.matrix.at_xy(x, y) {
                    if *element == 1 {
                        points.push(Point {
                            x: x as i32,
                            y: y as i32,
                        });
                    }
                }
            }
        }
        return points;
    }
}

#[cfg(test)]
mod figure_tests {
    use super::*;
    #[test]
    fn test_t_figure_rotation() {
        let figure = Figure::new(FigureType::T);
        let first_rotation = figure.rotated();
        let second_rotation = first_rotation.rotated();
        let third_rotation = second_rotation.rotated();
        let full_loop_rotation = third_rotation.rotated();

        let first_rotation_matrix = Matrix::new(vec![vec![0, 1, 0], vec![0, 1, 1], vec![0, 1, 0]]);
        let second_rotation_matrix = Matrix::new(vec![vec![0, 0, 0], vec![1, 1, 1], vec![0, 1, 0]]);
        let third_rotation_matrix = Matrix::new(vec![vec![0, 1, 0], vec![1, 1, 0], vec![0, 1, 0]]);
        let full_loop_rotation_matrix =
            Matrix::new(vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 0, 0]]);

        assert_eq!(first_rotation.matrix, first_rotation_matrix);
        assert_eq!(second_rotation.matrix, second_rotation_matrix);
        assert_eq!(third_rotation.matrix, third_rotation_matrix);
        assert_eq!(full_loop_rotation.matrix, full_loop_rotation_matrix);
    }
    #[test]
    fn test_i_figure_rotation() {
        let figure = Figure::new(FigureType::I);
        let first_rotation = figure.rotated();
        let second_rotation = first_rotation.rotated();
        let third_rotation = second_rotation.rotated();
        let full_loop_rotation = third_rotation.rotated();
        let first_rotation_matrix = Matrix::new(vec![
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
        ]);
        let second_rotation_matrix = Matrix::new(vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![1, 1, 1, 1],
            vec![0, 0, 0, 0],
        ]);
        let third_rotation_matrix = Matrix::new(vec![
            vec![0, 1, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 1, 0, 0],
            vec![0, 1, 0, 0],
        ]);
        let full_loop_rotation_matrix = Matrix::new(vec![
            vec![0, 0, 0, 0],
            vec![1, 1, 1, 1],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ]);
        assert_eq!(first_rotation.matrix, first_rotation_matrix);
        assert_eq!(second_rotation.matrix, second_rotation_matrix);
        assert_eq!(third_rotation.matrix, third_rotation_matrix);
        assert_eq!(full_loop_rotation.matrix, full_loop_rotation_matrix);
    }
    #[test]
    fn test_l_figure_rotation() {
        let figure = Figure::new(FigureType::L);
        let first_rotation = figure.rotated();
        let second_rotation = first_rotation.rotated();
        let third_rotation = second_rotation.rotated();
        let full_loop_rotation = third_rotation.rotated();

        let first_rotation_matrix = Matrix::new(vec![vec![0, 1, 0], vec![0, 1, 0], vec![0, 1, 1]]);
        let second_rotation_matrix = Matrix::new(vec![vec![0, 0, 0], vec![1, 1, 1], vec![1, 0, 0]]);
        let third_rotation_matrix = Matrix::new(vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]]);
        let full_loop_rotation_matrix =
            Matrix::new(vec![vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]]);

        assert_eq!(first_rotation.matrix, first_rotation_matrix);
        assert_eq!(second_rotation.matrix, second_rotation_matrix);
        assert_eq!(third_rotation.matrix, third_rotation_matrix);
        assert_eq!(full_loop_rotation.matrix, full_loop_rotation_matrix);
    }
    #[test]
    fn test_j_figure_rotation() {
        let figure = Figure::new(FigureType::J);
        let first_rotation = figure.rotated();
        let second_rotation = first_rotation.rotated();
        let third_rotation = second_rotation.rotated();
        let full_loop_rotation = third_rotation.rotated();

        let first_rotation_matrix = Matrix::new(vec![vec![0, 1, 1], vec![0, 1, 0], vec![0, 1, 0]]);
        let second_rotation_matrix = Matrix::new(vec![vec![0, 0, 0], vec![1, 1, 1], vec![0, 0, 1]]);
        let third_rotation_matrix = Matrix::new(vec![vec![0, 1, 0], vec![0, 1, 0], vec![1, 1, 0]]);
        let full_loop_rotation_matrix =
            Matrix::new(vec![vec![1, 0, 0], vec![1, 1, 1], vec![0, 0, 0]]);

        assert_eq!(first_rotation.matrix, first_rotation_matrix);
        assert_eq!(second_rotation.matrix, second_rotation_matrix);
        assert_eq!(third_rotation.matrix, third_rotation_matrix);
        assert_eq!(full_loop_rotation.matrix, full_loop_rotation_matrix);
    }
    #[test]
    fn test_s_figure_rotation() {
        let figure = Figure::new(FigureType::S);
        let first_rotation = figure.rotated();
        let second_rotation = first_rotation.rotated();
        let third_rotation = second_rotation.rotated();
        let full_loop_rotation = third_rotation.rotated();

        let first_rotation_matrix = Matrix::new(vec![vec![0, 1, 0], vec![0, 1, 1], vec![0, 0, 1]]);
        let second_rotation_matrix = Matrix::new(vec![vec![0, 0, 0], vec![0, 1, 1], vec![1, 1, 0]]);
        let third_rotation_matrix = Matrix::new(vec![vec![1, 0, 0], vec![1, 1, 0], vec![0, 1, 0]]);
        let full_loop_rotation_matrix =
            Matrix::new(vec![vec![0, 1, 1], vec![1, 1, 0], vec![0, 0, 0]]);

        assert_eq!(first_rotation.matrix, first_rotation_matrix);
        assert_eq!(second_rotation.matrix, second_rotation_matrix);
        assert_eq!(third_rotation.matrix, third_rotation_matrix);
        assert_eq!(full_loop_rotation.matrix, full_loop_rotation_matrix);
    }
    #[test]
    fn test_z_figure_rotation() {
        let figure = Figure::new(FigureType::Z);
        let first_rotation = figure.rotated();
        let second_rotation = first_rotation.rotated();
        let third_rotation = second_rotation.rotated();
        let full_loop_rotation = third_rotation.rotated();

        let first_rotation_matrix = Matrix::new(vec![vec![0, 0, 1], vec![0, 1, 1], vec![0, 1, 0]]);
        let second_rotation_matrix = Matrix::new(vec![vec![0, 0, 0], vec![1, 1, 0], vec![0, 1, 1]]);
        let third_rotation_matrix = Matrix::new(vec![vec![0, 1, 0], vec![1, 1, 0], vec![1, 0, 0]]);
        let full_loop_rotation_matrix =
            Matrix::new(vec![vec![1, 1, 0], vec![0, 1, 1], vec![0, 0, 0]]);

        assert_eq!(first_rotation.matrix, first_rotation_matrix);
        assert_eq!(second_rotation.matrix, second_rotation_matrix);
        assert_eq!(third_rotation.matrix, third_rotation_matrix);
        assert_eq!(full_loop_rotation.matrix, full_loop_rotation_matrix);
    }
    #[test]
    fn test_o_figure_rotation() {
        let figure = Figure::new(FigureType::O);
        let first_rotation = figure.rotated();
        let second_rotation = first_rotation.rotated();
        let third_rotation = second_rotation.rotated();
        let full_loop_rotation = third_rotation.rotated();

        let how_it_should_always_look_like = Matrix::new(vec![vec![1, 1], vec![1, 1]]);

        assert_eq!(first_rotation.matrix, how_it_should_always_look_like);
        assert_eq!(second_rotation.matrix, how_it_should_always_look_like);
        assert_eq!(third_rotation.matrix, how_it_should_always_look_like);
        assert_eq!(full_loop_rotation.matrix, how_it_should_always_look_like);
    }
    #[test]
    fn test_draw() {
        let figure = Figure::new(FigureType::T);
        let drawed = figure.to_cartesian();
        assert_eq!(drawed.len(), 4);
        assert_eq!(drawed[0], Point { x: 1, y: 0 });
        assert_eq!(drawed[1], Point { x: 0, y: 1 });
        assert_eq!(drawed[2], Point { x: 1, y: 1 });
        assert_eq!(drawed[3], Point { x: 2, y: 1 });
    }
    #[test]
    fn test_to_cartesian() {
        let figure = Figure::new(FigureType::O);
        let expectation = vec![
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
        ];
        let cartesian = figure.to_cartesian();
        assert_eq!(cartesian, expectation);
    }
}
