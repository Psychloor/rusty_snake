use std::collections::VecDeque;

use macroquad::prelude::*;

const WORLD_WIDTH: u32 = 25;
const WORLD_HEIGHT: u32 = 18;
const MOVE_SPEED: f64 = 1f64 / 8f64;
const GAME_OVER_TEXT: &str = "Game Over";

use nalgebra::Vector2;

type Position = Vector2<i16>;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new_direction_allowed(&self, new_dir: Direction) -> bool {
        match self {
            Direction::None => true,
            Direction::Up => new_dir != Direction::Down,
            Direction::Down => new_dir != Direction::Up,
            Direction::Left => new_dir != Direction::Right,
            Direction::Right => new_dir != Direction::Left,
        }
    }
}

struct GameState {
    snake: VecDeque<Position>,
    extensions: u8,
    fruit_position: Position,
    score: u32,
}

impl GameState {
    fn new() -> Self {
        Self {
            snake: VecDeque::new(),
            extensions: 2,
            fruit_position: Position::default(),
            score: 0,
        }
    }

    fn place_fruit(&mut self) {
        loop {
            let x = rand::gen_range(0, WORLD_WIDTH);
            let y = rand::gen_range(0, WORLD_HEIGHT);

            if self.collides_with_body(x as i16, y as i16, 0) {
                continue;
            }

            self.fruit_position = Position::new(x as i16, y as i16);
            break;
        }
    }

    fn move_direction(&mut self, direction: &Direction) -> bool {
        let current_position = self
            .snake
            .front()
            .expect("Snake should have at least 1 part before this method");
        let new_position = match direction {
            Direction::None => return false,
            Direction::Up => Position::new(current_position.x, current_position.y - 1),
            Direction::Down => Position::new(current_position.x, current_position.y + 1),
            Direction::Left => Position::new(current_position.x - 1, current_position.y),
            Direction::Right => Position::new(current_position.x + 1, current_position.y),
        };

        self.snake.push_front(new_position);

        if new_position == self.fruit_position {
            self.score += 1;
            self.extensions += 1;

            if self.snake.len() + 1 >= (WORLD_WIDTH * WORLD_HEIGHT) as usize {
                return true;
            } else {
                self.place_fruit();
            }
        }

        if new_position.x < 0
            || new_position.x >= WORLD_WIDTH as i16
            || new_position.y < 0
            || new_position.y >= WORLD_HEIGHT as i16
            || self.collides_with_body(new_position.x, new_position.y, 1)
        {
            return true;
        }

        if self.extensions > 0 {
            self.extensions -= 1;
        } else {
            self.snake.pop_back();
        }

        false
    }

    fn body_parts(&self) -> std::collections::vec_deque::Iter<'_, Position> {
        self.snake.iter()
    }

    fn collides_with_body(&self, x: i16, y: i16, skip: usize) -> bool {
        self.snake
            .iter()
            .skip(skip)
            .any(|pos| pos.x == x && pos.y == y)
    }

    fn reset(&mut self) {
        self.snake.clear();
        self.snake.push_front(Position::new(
            (WORLD_WIDTH / 2) as i16,
            (WORLD_HEIGHT / 2) as i16,
        ));

        self.extensions = 2;
        self.place_fruit();
        self.score = 0;
    }
}

fn render_game_state(game_state: &GameState, tile_width: f32, tile_height: f32, game_over: bool) {
            // Rendering
        clear_background(BLACK);

        for part in game_state.body_parts() {
            draw_rectangle(
                part.x as f32 * tile_width,
                part.y as f32 * tile_height,
                tile_width - 1f32,
                tile_height - 1f32,
                BLUE,
            );
        }

        draw_rectangle(
            game_state.fruit_position.x as f32 * tile_width,
            game_state.fruit_position.y as f32 * tile_height,
            tile_width - 1f32,
            tile_height - 1f32,
            RED,
        );

        draw_text(
            &format!("Score: {}", game_state.score),
            16f32,
            16f32,
            24f32,
            WHITE,
        );

        if game_over {
            let bounds = measure_text(GAME_OVER_TEXT, None, 64, 1f32);
            draw_text(
                GAME_OVER_TEXT,
                (screen_width() / 2f32) - (bounds.width / 2f32),
                (screen_height() / 2f32) - (bounds.height / 2f32),
                64f32,
                WHITE,
            );
        }
}

#[macroquad::main("Rusty Snake V2")]
async fn main() {
    let mut game_state = GameState::new();
    game_state.reset();

    let mut current_direction = Direction::None;

    let tile_width = screen_width() / WORLD_WIDTH as f32;
    let tile_height = screen_height() / WORLD_HEIGHT as f32;

    let mut time_since_move: f64 = 0f64;
    let mut game_over = false;

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if !game_over {
            if is_key_pressed(KeyCode::Up) && current_direction.new_direction_allowed(Direction::Up) {
                current_direction = Direction::Up
            }
            if is_key_pressed(KeyCode::Down) && current_direction.new_direction_allowed(Direction::Down) {
                current_direction = Direction::Down
            }
            if is_key_pressed(KeyCode::Left) && current_direction.new_direction_allowed(Direction::Left) {
                current_direction = Direction::Left
            }
            if is_key_pressed(KeyCode::Right) && current_direction.new_direction_allowed(Direction::Right) {
                current_direction = Direction::Right
            }

            if get_time() - time_since_move >= MOVE_SPEED && current_direction != Direction::None {
                time_since_move = get_time();
                game_over = game_state.move_direction(&current_direction);
            }
        }

        if game_over && is_key_pressed(KeyCode::R) {
            game_state.reset();
            current_direction = Direction::None;
            time_since_move = 0f64;
            game_over = false;
        }

        render_game_state(&game_state, tile_width, tile_height, game_over);

        next_frame().await
    }
}
