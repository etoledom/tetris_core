use super::move_validator::{can_move_down, has_valid_position};
use super::{ActiveFigure, Block, Board, FigureType, Point, Size};

const MOVING_PERIOD: f64 = 0.2; //secs

pub enum Action {
    MoveDown,
    MoveLeft,
    MoveRight,
    Rotate,
}

pub trait Randomizer {
    fn random_between(&self, first: i32, last: i32) -> i32;
}

#[derive(PartialEq)]
pub enum GameState {
    Playing,
    GameOver,
}

pub struct Game {
    board: Board,
    score: u64,
    active: ActiveFigure,
    next: ActiveFigure,
    waiting_time: f64,
    randomizer: Box<dyn Randomizer + 'static>,
    state: GameState,
}

impl Game {
    pub fn new(size: &Size, randomizer: Box<dyn Randomizer + 'static>) -> Game {
        let start_point = Game::figure_start_point(size.width);
        let active = Game::random_figure(start_point, &randomizer);
        let next = Game::random_figure(start_point, &randomizer);

        let board = Board::new(size);
        return Game {
            board,
            score: 0,
            active,
            next,
            waiting_time: 0.0,
            randomizer,
            state: GameState::Playing,
        };
    }

    fn figure_start_point(width: usize) -> Point {
        let mid_point = (width as i32).wrapping_div(2) - 2;
        return Point { x: mid_point, y: 0 };
    }

    fn random_figure(position: Point, randomizer: &Box<dyn Randomizer + 'static>) -> ActiveFigure {
        let figure = match randomizer.random_between(0, 6) {
            0 => FigureType::I,
            1 => FigureType::J,
            2 => FigureType::L,
            3 => FigureType::O,
            4 => FigureType::S,
            5 => FigureType::T,
            _ => FigureType::Z,
        };
        return ActiveFigure::new(figure, position);
    }

    pub fn is_game_over(&self) -> bool {
        return self.state == GameState::GameOver;
    }

    // DRAWING FUNCTIONS

    pub fn draw(&self) -> Vec<Block> {
        let board = self.draw_board();
        let figure = self.draw_active_figure();
        return board.iter().chain(&figure).cloned().collect();
    }

    fn draw_active_figure(&self) -> Vec<Block> {
        let figure = self.active.to_cartesian();
        return figure
            .iter()
            .map(|point| Block::new(point.x, point.y, 1, 1, self.active.color()))
            .collect();
    }

    fn draw_board(&self) -> Vec<Block> {
        let mut blocks = vec![];
        for y in 0..self.board.height() {
            for x in 0..self.board.width() {
                if let Some(square) = self.board.figure_at_xy(x, y) {
                    let block = Block::new(x as i32, y as i32, 1, 1, square.color());
                    blocks.push(block);
                }
            }
        }
        return blocks;
    }

    // GAME UPDATE

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        if self.waiting_time > MOVING_PERIOD {
            self.update_game();
            self.waiting_time = 0.0;
        }
    }

    fn update_game(&mut self) {
        if self.state == GameState::GameOver {
            return;
        }
        if can_move_down(&self.active, &self.board) {
            self.move_down();
        } else {
            self.update_next_figure();
        }
    }

    fn update_next_figure(&mut self) {
        self.add_active_figure_to_board();
        let completed_lines_count = self.remove_completed_lines();
        self.add_score_for(completed_lines_count);
        self.add_new_active_figure();
        self.update_state();
    }

    fn update_state(&mut self) {
        if self.check_is_game_over() {
            self.state = GameState::GameOver;
        }
    }

    // MOVEMENT FUNCTIONS

    pub fn perform(&mut self, action: Action) {
        match action {
            Action::MoveLeft => self.move_left(),
            Action::MoveRight => self.move_right(),
            Action::MoveDown => self.move_down(),
            Action::Rotate => self.rotate_active_figure(),
        }
    }

    fn move_left(&mut self) {
        self.update_active_with(self.active.moved_left());
    }

    fn move_right(&mut self) {
        self.update_active_with(self.active.moved_right());
    }

    fn move_down(&mut self) {
        self.update_active_with(self.active.moved_down());
    }

    fn rotate_active_figure(&mut self) {
        if let Some(rotated) = self.wall_kicked_rotated_active_figure() {
            self.update_active_with(rotated);
        }
    }

    // WALL KICK

    fn wall_kicked_rotated_active_figure(&self) -> Option<ActiveFigure> {
        return self
            .active
            .wall_kicked_rotation_tests()
            .into_iter()
            .find(|figure| has_valid_position(figure, &self.board));
    }

    // Game state mutation

    fn update_active_with(&mut self, new_active: ActiveFigure) {
        if has_valid_position(&new_active, &self.board) {
            self.active = new_active;
        }
    }

    fn add_active_figure_to_board(&mut self) {
        for point in self.active.to_cartesian() {
            self.board = self.board.replacing_figure_at_xy(
                point.x as usize,
                point.y as usize,
                Some(self.active.get_type()),
            );
        }
    }

    fn add_new_active_figure(&mut self) {
        let start_point = Game::figure_start_point(self.board.width());
        self.update_active_with(self.next.clone());
        self.next = Game::random_figure(start_point, &self.randomizer);
    }

    fn remove_completed_lines(&mut self) -> usize {
        let lines = self.lines_completed();
        self.board = self.board.removing_lines(&lines);
        return lines.len();
    }

    // Lines checks

    fn lines_completed(&self) -> Vec<usize> {
        let mut completed_lines: Vec<usize> = vec![];
        for line_number in 0..self.board.height() {
            if self.is_line_completed(line_number) {
                completed_lines.push(line_number);
            }
        }
        return completed_lines;
    }

    fn is_line_completed(&self, line_number: usize) -> bool {
        if let Some(line) = self.board.get_line(line_number) {
            return !line.contains(&None);
        }
        return false;
    }

    // Score

    fn add_score_for(&mut self, completed_lines: usize) {
        self.score += (completed_lines as u64) * 100;
    }

    fn check_is_game_over(&self) -> bool {
        return self.active.position().y == 0 && !has_valid_position(&self.active, &self.board);
    }

    pub fn get_score(&self) -> u64 {
        return self.score;
    }
}

#[cfg(test)]
mod game_tests {
    use super::*;

    struct Random {
        number: i32,
    }

    impl Randomizer for Random {
        fn random_between(&self, _first: i32, _last: i32) -> i32 {
            return self.number;
        }
    }

    #[test]
    fn test_active_figure_is_draw() {
        let game = get_game();
        let active_points = game.active.to_cartesian();
        let drawed_points = draw_to_cartesian(game.draw());

        assert_eq!(drawed_points, active_points);
    }
    #[test]
    fn test_active_figure_moves_down() {
        let mut game = get_game();
        let first_position = game.active.to_cartesian();
        let expected: Vec<Point> = first_position
            .iter()
            .map(|point| Point {
                x: point.x,
                y: point.y + 1,
            })
            .collect();
        game.perform(Action::MoveDown);
        let drawed_points = draw_to_cartesian(game.draw());

        assert_eq!(drawed_points, expected);
    }
    #[test]
    fn test_active_figure_does_not_move_lower_than_floor() {
        let mut game = get_game();
        let y = game.board.height() as i32 - 3; // 3 spaces before the floor
        game.active = ActiveFigure::new(FigureType::O, Point { x: 10, y });
        game.perform(Action::MoveDown);
        game.perform(Action::MoveDown);
        game.perform(Action::MoveDown);
        assert_eq!(game.active.bottom_edge(), 39);
        game.perform(Action::MoveDown);
        assert_eq!(game.active.bottom_edge(), 39);
    }
    #[test]
    fn test_rotate_active_figure() {
        let mut game = get_game();
        let rotated = game.active.rotated();
        game.perform(Action::Rotate);
        let drawed_points = draw_to_cartesian(game.draw());
        assert_eq!(drawed_points, rotated.to_cartesian());
    }

    #[test]
    fn test_move_left() {
        let mut game = get_game();
        game.active = ActiveFigure::new(FigureType::L, Point { x: 10, y: 0 });
        assert_eq!(game.active.left_edge(), 10);
        game.perform(Action::MoveLeft);
        assert_eq!(game.active.left_edge(), 9);
    }
    #[test]
    fn test_move_left_does_not_go_beyond_zero() {
        let mut game = get_game();
        game.active = ActiveFigure::new(FigureType::L, Point { x: 2, y: 0 });
        game.active = game.active.rotated(); // left edge is now at x: 3
        assert_eq!(game.active.left_edge(), 3);
        game.perform(Action::MoveLeft); // x: 2
        game.perform(Action::MoveLeft); // x: 1
        game.perform(Action::MoveLeft); // x: 0
        game.perform(Action::MoveLeft); // x: 0
        assert_eq!(game.active.left_edge(), 0);
    }
    #[test]
    fn test_move_right() {
        let mut game = get_game();
        game.active = ActiveFigure::new(FigureType::L, Point { x: 0, y: 0 });
        game.perform(Action::MoveRight);
        assert_eq!(game.active.position(), Point { x: 1, y: 0 });
    }
    #[test]
    fn test_move_right_does_not_go_beyond_board_edge() {
        let mut game = get_game();
        game.active = ActiveFigure::new(FigureType::I, Point { x: 16, y: 0 });
        game.active = game.active.rotated(); // right edge is now at 18
        assert_eq!(game.active.left_edge(), 18);
        game.perform(Action::MoveRight); // x: 19
        game.perform(Action::MoveRight); // x: 19
        assert_eq!(game.active.right_edge(), 19);
    }
    #[test]
    fn test_add_active_figure_to_board() {
        let mut game = get_game();
        assert!(game.draw_board().is_empty());
        game.add_active_figure_to_board();
        assert_eq!(game.draw_board().len(), 4);
    }
    #[test]
    fn test_active_figure_is_added_when_it_touches_the_floor() {
        let mut game = get_game_with_size(4, 10);

        assert_eq!(game.active.position().y, 0); // lowest figure block is at y: 1
        assert!(game.draw_board().is_empty());

        update_loops(&mut game, 3); // Should add figure to board and create new active

        assert_eq!(game.active.position().y, 0);
        assert_eq!(game.draw_board().len(), 4);
    }
    #[test]
    fn test_active_figure_is_added_when_touches_block() {
        let mut game = get_game_with_size(7, 10);
        game.active = ActiveFigure::new(FigureType::L, Point { x: 5, y: 5 });
        game.update(10.0); // current figure should be added to the board
        assert_eq!(game.draw_board().len(), 4); // Next figure should colide at y: 5

        update_loops(&mut game, 5); // Takes y from 1 to 5

        assert_eq!(game.active.position().y, 0);
        assert_eq!(game.draw_board().len(), 8);
    }
    #[test]
    fn test_start_point_pair() {
        let width = 10;
        let start_point = Game::figure_start_point(width);
        assert_eq!(start_point.x, 3);
    }
    #[test]
    fn test_start_point_odd() {
        let width = 11;
        let start_point = Game::figure_start_point(width);
        assert_eq!(start_point.x, 3);
    }
    #[test]
    fn test_wallkick_l_left() {
        let mut game = get_game();
        game.active = ActiveFigure::new(FigureType::L, Point { x: 0, y: 5 });
        game.perform(Action::Rotate);
        game.perform(Action::MoveLeft);
        game.perform(Action::Rotate);
        assert_eq!(game.active.position().x, 0);
    }
    #[test]
    fn test_is_game_over() {
        let mut game = get_game_with_size(6, 10);
        game.board = game.board.replacing_figure_at_xy(3, 1, Some(FigureType::L));
        game.board = game.board.replacing_figure_at_xy(4, 1, Some(FigureType::L));
        game.board = game.board.replacing_figure_at_xy(5, 1, Some(FigureType::L));
        game.update(10.0);
        assert!(game.is_game_over());
    }
    #[test]
    fn test_is_game_over_returns_false() {
        let mut game = get_game();
        update_loops(&mut game, 1);
        assert!(!game.is_game_over());
    }
    #[test]
    fn test_get_score() {
        let mut game = get_game_with_size(2, 2);
        assert_eq!(game.get_score(), 0);

        // Completing line
        game.board.replacing_figure_at_xy(0, 1, Some(FigureType::I));
        game.board.replacing_figure_at_xy(1, 1, Some(FigureType::I));
        game.update(10.0);

        assert_eq!(game.get_score(), 100);
    }
    #[test]
    fn test_double_line_score() {
        let mut game = get_game_with_size(2, 2);
        assert_eq!(game.get_score(), 0);

        game.active = ActiveFigure::new(FigureType::O, Point{ x: 0, y: 0});
        game.update(10.0);

        assert_eq!(game.get_score(), 200);
    }

    // HELPERS

    fn draw_to_cartesian(draw: Vec<Block>) -> Vec<Point> {
        return draw.iter().map(|block| block.position()).collect();
    }
    fn get_game() -> Game {
        return get_game_with_size(40, 20);
    }
    fn get_game_with_size(height: usize, width: usize) -> Game {
        let size = Size {
            height,
            width,
        };
        return Game::new(&size, get_randomizer());
    }
    fn get_randomizer() -> Box<dyn Randomizer> {
        return Box::new(Random { number: 5 });
    }
    fn update_loops(game: &mut Game, update_times: u32) {
        for _ in 0..update_times {
            game.update(10.0);
        }
    }
}
