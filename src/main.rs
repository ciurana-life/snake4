use macroquad::prelude::*;
use snake3::{
    named,
    snake::{Apple, SnakeDirection},
    SnakeGame,
};

// Game constants
const GRID_WIDTH: f32 = 20.0;
const BASE_UPDATE_INTERVAL: f32 = 0.5;
const MIN_MOVE_INTERVAL: f32 = 0.1;

/// Main game state manager
struct Game {
    /// The snake3 game instance
    snake_game: SnakeGame,
    /// Time accumulator for automatic movement
    update_timer: f32,
    /// Last time the snake moved (for input cooldown)
    last_move_time: f32,
    /// Current game score
    score: u32,
    /// High score record
    high_score: u32,
}

impl Game {
    /// Create a new game with specified grid dimensions
    pub fn new(columns: i16, rows: i16) -> Self {
        Self {
            snake_game: SnakeGame::new(columns, rows, None, None),
            update_timer: 0.0,
            last_move_time: 0.0,
            score: 0,
            high_score: 0,
        }
    }

    /// Handle player input and return whether movement occurred
    pub fn handle_input(&mut self) -> bool {
        let mut moved = false;

        if is_key_pressed(KeyCode::Right) {
            self.snake_game.snake.set_direction(SnakeDirection::Right);
            moved = true;
        } else if is_key_pressed(KeyCode::Left) {
            self.snake_game.snake.set_direction(SnakeDirection::Left);
            moved = true;
        } else if is_key_pressed(KeyCode::Down) {
            self.snake_game.snake.set_direction(SnakeDirection::Up);
            moved = true;
        } else if is_key_pressed(KeyCode::Up) {
            self.snake_game.snake.set_direction(SnakeDirection::Down);
            moved = true;
        }

        moved
    }

    /// Advance the game state by one tick
    pub fn update(&mut self, delta: f32) -> bool {
        self.update_timer += delta;
        let current_time = get_time() as f32;
        let input_moved = self.handle_input();

        // Determine if we should move based on either input or timer
        let should_move = (input_moved && (current_time - self.last_move_time) >= MIN_MOVE_INTERVAL)
            || (!input_moved && self.update_timer >= BASE_UPDATE_INTERVAL);

        if should_move {
            self.snake_game.snake.advance();

            // Handle collisions
            if self.snake_game.check_collisions() {
                return false; // Game over
            }

            // Handle apple eating
            if let Some(hit) = self.snake_game.check_entity_collision() {
                if hit.downcast_ref::<Apple>().is_some() {
                    self.snake_game.snake.grow();
                    self.score += 1;
                }
            }

            // Spawn new apple if needed
            if self.snake_game.entities.is_empty() {
                _ = self.snake_game.generate_entity(named!(Apple));
            }

            // Reset movement tracking
            self.last_move_time = current_time;
            if input_moved {
                self.update_timer = 0.0;
            } else {
                self.update_timer -= BASE_UPDATE_INTERVAL;
            }
        }

        true // Game continues
    }

    /// Render the current game state
    pub fn draw(&self) {
        // Draw snake
        for segment in &self.snake_game.snake.body {
            draw_rectangle(
                segment.x as f32 * GRID_WIDTH,
                segment.y as f32 * GRID_WIDTH,
                GRID_WIDTH,
                GRID_WIDTH,
                RED,
            );
        }

        // Draw apple if present
        if let Some(apple) = self.snake_game.entities.first() {
            draw_circle(
                (apple.x() as f32 * GRID_WIDTH) + GRID_WIDTH / 2.0,
                (apple.y() as f32 * GRID_WIDTH) + GRID_WIDTH / 2.0,
                GRID_WIDTH / 2.0,
                YELLOW,
            );
        }

        // Draw score
        draw_text(
            &format!("Score: {}", self.score),
            20.0,
            20.0,
            30.0,
            WHITE,
        );
    }
}

#[macroquad::main("Snake Game")]
async fn main() {
    // Initialize game with appropriate grid size
    let (columns, rows) = (
        (screen_width() / GRID_WIDTH).floor() as i16 - 1,
        (screen_height() / GRID_WIDTH).floor() as i16 - 1,
    );
    let mut game = Game::new(columns, rows);

    loop {
        clear_background(BLACK);

        // Update game state
        if !game.update(get_frame_time()) {
            // Game over - reset
            game.high_score = game.high_score.max(game.score);
            game = Game::new(columns, rows);
        }

        // Draw game
        game.draw();

        next_frame().await;
    }
}