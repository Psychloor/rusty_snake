use std::collections::LinkedList;

use macroquad::prelude::*;

const WORLD_WIDTH: u32 = 25;
const WORLD_HEIGHT: u32 = 18;
const MOVE_SPEED: f64 = 1f64 / 8f64;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
struct Position {
    pub x: i16,
    pub y: i16,
}

impl Position {
    fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}

struct GameState {
    snake: LinkedList<Position>,
    extensions: u8,
    fruit_position: Position,
    score: u32,
    game_over: bool,
    score_text: String,
}

impl GameState {
    fn new() -> Self {
        Self {
            snake: LinkedList::new(),
            extensions: 2,
            game_over: false,
            score: 0,
            fruit_position: Position::default(),
            score_text: format!("Score: {}", 0),
        }
    }

    fn place_fruit(&mut self) -> bool {
        let max_tries = WORLD_WIDTH * WORLD_HEIGHT * 5;

        for _ in 0..=max_tries {
            let x = rand::gen_range(0, WORLD_WIDTH);
            let y = rand::gen_range(0, WORLD_HEIGHT);

            if self.collides_with_body(x as i16, y as i16, 0) {
                continue;
            }

            self.fruit_position = Position::new(x as i16, y as i16);
            return true;
        }

        false
    }

    fn move_direction(&mut self, direction: &Direction) {
        let current_position = self
            .snake
            .front()
            .expect("Snake should have at least 1 part before this method");
        match direction {
            Direction::None => todo!(),
            Direction::Up => self
                .snake
                .push_front(Position::new(current_position.x, current_position.y - 1)),
            Direction::Down => self
                .snake
                .push_front(Position::new(current_position.x, current_position.y + 1)),
            Direction::Left => self
                .snake
                .push_front(Position::new(current_position.x - 1, current_position.y)),
            Direction::Right => self
                .snake
                .push_front(Position::new(current_position.x + 1, current_position.y)),
        }

        if *self.snake.front().unwrap() == self.fruit_position {
            self.score += 1;
            self.score_text = format!("Score: {}", self.score);
            self.extensions += 1;
            self.place_fruit();
        }

        let head_pos = self.snake.front().unwrap();
        if head_pos.x < 0
            || head_pos.x >= WORLD_WIDTH as i16
            || head_pos.y < 0
            || head_pos.y >= WORLD_HEIGHT as i16
        {
            self.game_over = true;
            return;
        }

        if self.collides_with_body(
            self.snake.front().unwrap().x,
            self.snake.front().unwrap().y,
            1,
        ) {
            self.game_over = true;
            return;
        }

        if self.extensions > 0 {
            self.extensions -= 1;
        } else {
            self.snake.pop_back();
        }
    }

    fn body_parts(&self) -> std::collections::linked_list::Iter<'_, Position> {
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
        self.game_over = false;
        self.place_fruit();
        self.score = 0;
        self.score_text = format!("Score: {}", self.score);
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

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if !game_state.game_over {
            if is_key_pressed(KeyCode::Up) {
                current_direction = Direction::Up
            }
            if is_key_pressed(KeyCode::Down) {
                current_direction = Direction::Down
            }
            if is_key_pressed(KeyCode::Left) {
                current_direction = Direction::Left
            }
            if is_key_pressed(KeyCode::Right) {
                current_direction = Direction::Right
            }

            if get_time() - time_since_move >= MOVE_SPEED && current_direction != Direction::None {
                time_since_move = get_time();
                game_state.move_direction(&current_direction)
            }
        }

        if game_state.game_over && is_key_pressed(KeyCode::R) {
            game_state.reset();
            current_direction = Direction::None;
            time_since_move = 0f64;
        }

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

        draw_text(&game_state.score_text, 16f32, 16f32, 24f32, WHITE);

        if game_state.game_over {
            let game_over_text = "Game Over";
            let bounds = measure_text(game_over_text, None, 64, 1f32);
            draw_text(
                game_over_text,
                (screen_width() / 2f32) - (bounds.width / 2f32),
                (screen_height() / 2f32) - (bounds.height / 2f32),
                64f32,
                WHITE,
            );
        }

        next_frame().await
    }
}
