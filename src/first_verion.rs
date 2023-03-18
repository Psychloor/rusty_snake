// Only kept here to compare my first time using rust and making the snake game without knowing as much as second try
use macroquad::prelude::*;
 
#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Position {
    pub x: i8,
    pub y: i8,
}
 
impl Position {
    fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }
}
 
#[derive(Debug, Default, PartialEq, Eq)]
struct BodyPart {
    current: Position,
    previous: Position,
}
 
impl BodyPart {
    fn new(pos: &Position) -> Self {
        Self {
            current: pos.clone(),
            previous: pos.clone(),
        }
    }
    fn from_xy(x: i8, y: i8) -> Self {
        Self {
            current: Position { x, y },
            previous: Position { x, y },
        }
    }
 
    fn set_new_position(&mut self, x: i8, y: i8) {
        self.previous = self.current.clone();
 
        self.current.x = x;
        self.current.y = y;
    }
 
    fn current_position(&self) -> &Position {
        &self.current
    }
 
    fn previous_position(&self) -> &Position {
        &self.previous
    }
}
 
#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
}
 
fn place_fruit(
    snake_iter: &[BodyPart],
    world_width: i8,
    world_height: i8,
) -> Result<Position, &'static str> {
    let max_tries: u32 = world_width as u32 * world_height as u32 * 50;
 
    for _i in 0..max_tries {
        let x = rand::rand() % world_width as u32;
        let y = rand::rand() % world_height as u32;
 
        if snake_iter
            .iter()
            .any(|b| b.current_position().x as u32 == x && b.current_position().y as u32 == y)
        {
            continue;
        }
 
        return Ok(Position::new(x as i8, y as i8));
    }
 
    Err("Couldn't place the fruit anywhere")
}
 
#[macroquad::main("Rusty Snake")]
async fn main() {
    // Initialization
    let world_width = 25;
    let world_height = 18;
 
    let mut score: u32 = 0;
    let mut score_text = format!("Score: {}", score);
 
    let tile_width = screen_width() / world_width as f32;
    let tile_height = screen_height() / world_height as f32;
 
    let mut snake: Vec<BodyPart> = Vec::new();
    let mut current_direction: Direction = Direction::None;
    let mut extensions_left: u8 = 2;
 
    let mut game_over = false;
 
    let move_speed: f64 = 1f64 / 8f64;
    let mut time_since_move: f64 = 0f64;
 
    snake.push(BodyPart::from_xy(world_width / 2, world_height / 2));
 
    let mut fruit_position = place_fruit(&snake, world_width, world_height)
        .expect("Startup should always have a position to place");
 
    loop {
        // update
        if is_key_pressed(KeyCode::Left) {
            current_direction = Direction::Left;
        }
        if is_key_pressed(KeyCode::Right) {
            current_direction = Direction::Right;
        }
        if is_key_pressed(KeyCode::Up) {
            current_direction = Direction::Up;
        }
        if is_key_pressed(KeyCode::Down) {
            current_direction = Direction::Down;
        }
 
        if !game_over
            && current_direction != Direction::None
            && get_time() - time_since_move >= move_speed
        {
            time_since_move = get_time();
 
            for i in 0..snake.len() {
                if i == 0 {
                    let current_pos = snake[0].current_position().clone();
                    match current_direction {
                        Direction::Left => {
                            snake[0].set_new_position(current_pos.x - 1, current_pos.y)
                        }
                        Direction::Right => {
                            snake[0].set_new_position(current_pos.x + 1, current_pos.y)
                        }
                        Direction::Up => {
                            snake[0].set_new_position(current_pos.x, current_pos.y - 1)
                        }
                        Direction::Down => {
                            snake[0].set_new_position(current_pos.x, current_pos.y + 1)
                        }
                        Direction::None => (),
                    }
                }
 
                if i > 0 {
                    let previous_pos = snake[i - 1].previous_position().clone();
                    snake[i].set_new_position(previous_pos.x, previous_pos.y);
                }
            }
 
            let current_pos = snake[0].current_position().clone();
            for ele in snake.iter().skip(1) {
                if *ele.current_position() == current_pos {
                    game_over = true;
                    break;
                }
            }
 
            if *snake[0].current_position() == fruit_position {
                match place_fruit(&snake, world_width, world_height) {
                    Ok(position) => {
                        fruit_position = position;
                        score += 1;
                        extensions_left += 1;
                        score_text = format!("Score: {}", score);
                    }
                    Err(_error) => game_over = true,
                }
            }
 
            if snake[0].current_position().x < 0
                || snake[0].current_position().x >= world_width
                || snake[0].current_position().y < 0
                || snake[0].current_position().y >= world_height
            {
                game_over = true;
            }
 
            if extensions_left > 0 {
                extensions_left -= 1;
 
                snake.push(BodyPart::new(
                    &snake.last().expect("body parts should exist").previous,
                ));
            }
        }
 
        // Display
        clear_background(BLACK);
 
        if !snake.is_empty() {
            for ele in snake.iter() {
                draw_rectangle(
                    ele.current_position().x as f32 * tile_width,
                    ele.current_position().y as f32 * tile_height,
                    tile_width - 1f32,
                    tile_height - 1f32,
                    BLUE,
                );
            }
        }
 
        draw_rectangle(
            fruit_position.x as f32 * tile_width,
            fruit_position.y as f32 * tile_height,
            tile_width - 1f32,
            tile_height - 1f32,
            RED,
        );
 
        draw_text(score_text.as_str(), 16f32, 16f32, 28f32, WHITE);
 
        next_frame().await;
    }
}