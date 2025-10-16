use macroquad::prelude::*;

// Game constants
const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const PLAYER_SIZE: f32 = 40.0;
const PLAYER_SPEED: f32 = 300.0;
const BULLET_SIZE: f32 = 5.0;
const BULLET_SPEED: f32 = 400.0;
const ENEMY_SIZE: f32 = 30.0;
const ENEMY_SPEED: f32 = 150.0;
const ENEMY_SPAWN_INTERVAL: f32 = 1.5; // seconds

/// Represents the player character
#[derive(Debug, Clone)]
struct Player {
    position: Vec2,
    size: Vec2,
    color: Color,
}

impl Player {
    /// Create a new player at the bottom center of the screen
    fn new() -> Self {
        Self {
            position: Vec2::new(
                SCREEN_WIDTH / 2.0 - PLAYER_SIZE / 2.0,
                SCREEN_HEIGHT - PLAYER_SIZE - 20.0,
            ),
            size: Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
            color: BLUE,
        }
    }

    /// Update player position based on input
    fn update(&mut self) {
        // Handle left/right movement
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.position.x -= PLAYER_SPEED * get_frame_time();
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.position.x += PLAYER_SPEED * get_frame_time();
        }

        // Keep player within screen bounds
        self.position.x = self.position.x.clamp(0.0, SCREEN_WIDTH - self.size.x);
    }

    /// Draw the player
    fn draw(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            self.color,
        );
    }

    /// Get the center position of the player (for bullet spawning)
    fn center(&self) -> Vec2 {
        Vec2::new(
            self.position.x + self.size.x / 2.0,
            self.position.y + self.size.y / 2.0,
        )
    }

    /// Get the rectangle bounds for collision detection
    fn rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.size.x, self.size.y)
    }
}

/// Represents a bullet fired by the player
#[derive(Debug, Clone)]
struct Bullet {
    position: Vec2,
    size: Vec2,
    velocity: Vec2,
    color: Color,
}

impl Bullet {
    /// Create a new bullet at the given position
    fn new(position: Vec2) -> Self {
        Self {
            position,
            size: Vec2::new(BULLET_SIZE, BULLET_SIZE * 2.0),
            velocity: Vec2::new(0.0, -BULLET_SPEED),
            color: YELLOW,
        }
    }

    /// Update bullet position
    fn update(&mut self) {
        self.position += self.velocity * get_frame_time();
    }

    /// Draw the bullet
    fn draw(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            self.color,
        );
    }

    /// Check if bullet is off-screen
    fn is_off_screen(&self) -> bool {
        self.position.y + self.size.y < 0.0
    }

    /// Get the rectangle bounds for collision detection
    fn rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.size.x, self.size.y)
    }
}

/// Represents an enemy
#[derive(Debug, Clone)]
struct Enemy {
    position: Vec2,
    size: Vec2,
    velocity: Vec2,
    color: Color,
}

impl Enemy {
    /// Create a new enemy at a random position at the top of the screen
    fn new() -> Self {
        let x = rand::gen_range(0.0, SCREEN_WIDTH - ENEMY_SIZE);
        Self {
            position: Vec2::new(x, -ENEMY_SIZE),
            size: Vec2::new(ENEMY_SIZE, ENEMY_SIZE),
            velocity: Vec2::new(0.0, ENEMY_SPEED),
            color: RED,
        }
    }

    /// Update enemy position
    fn update(&mut self) {
        self.position += self.velocity * get_frame_time();
    }

    /// Draw the enemy
    fn draw(&self) {
        draw_rectangle(
            self.position.x,
            self.position.y,
            self.size.x,
            self.size.y,
            self.color,
        );
    }

    /// Check if enemy is off-screen
    fn is_off_screen(&self) -> bool {
        self.position.y > SCREEN_HEIGHT
    }

    /// Get the rectangle bounds for collision detection
    fn rect(&self) -> Rect {
        Rect::new(self.position.x, self.position.y, self.size.x, self.size.y)
    }
}

/// Main game state
struct Game {
    player: Player,
    bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
    score: u32,
    last_shot_time: f32,
    last_enemy_spawn: f32,
    game_over: bool,
}

impl Game {
    /// Create a new game instance
    fn new() -> Self {
        Self {
            player: Player::new(),
            bullets: Vec::new(),
            enemies: Vec::new(),
            score: 0,
            last_shot_time: 0.0,
            last_enemy_spawn: 0.0,
            game_over: false,
        }
    }

    /// Reset the game to initial state
    fn reset(&mut self) {
        *self = Self::new();
    }

    /// Update game state
    fn update(&mut self) {
        if self.game_over {
            // Check for restart
            if is_key_pressed(KeyCode::R) {
                self.reset();
            }
            return;
        }

        let current_time = get_time() as f32;

        // Update player
        self.player.update();

        // Handle shooting
        if is_key_pressed(KeyCode::Space) && current_time - self.last_shot_time > 0.2 {
            let bullet_pos = Vec2::new(
                self.player.center().x - BULLET_SIZE / 2.0,
                self.player.position.y,
            );
            self.bullets.push(Bullet::new(bullet_pos));
            self.last_shot_time = current_time;
        }

        // Update bullets
        for bullet in &mut self.bullets {
            bullet.update();
        }

        // Remove off-screen bullets
        self.bullets.retain(|bullet| !bullet.is_off_screen());

        // Spawn enemies
        if current_time - self.last_enemy_spawn > ENEMY_SPAWN_INTERVAL {
            self.enemies.push(Enemy::new());
            self.last_enemy_spawn = current_time;
        }

        // Update enemies
        for enemy in &mut self.enemies {
            enemy.update();
        }

        // Remove off-screen enemies
        self.enemies.retain(|enemy| !enemy.is_off_screen());

        // Check bullet-enemy collisions
        let mut bullets_to_remove = Vec::new();
        let mut enemies_to_remove = Vec::new();

        for (bullet_idx, bullet) in self.bullets.iter().enumerate() {
            for (enemy_idx, enemy) in self.enemies.iter().enumerate() {
                if bullet.rect().overlaps(&enemy.rect()) {
                    bullets_to_remove.push(bullet_idx);
                    enemies_to_remove.push(enemy_idx);
                    self.score += 10;
                }
            }
        }

        // Remove collided bullets and enemies (in reverse order to maintain indices)
        bullets_to_remove.sort_by(|a, b| b.cmp(a));
        enemies_to_remove.sort_by(|a, b| b.cmp(a));

        for &idx in &bullets_to_remove {
            if idx < self.bullets.len() {
                self.bullets.remove(idx);
            }
        }

        for &idx in &enemies_to_remove {
            if idx < self.enemies.len() {
                self.enemies.remove(idx);
            }
        }

        // Check player-enemy collisions (game over condition)
        for enemy in &self.enemies {
            if self.player.rect().overlaps(&enemy.rect()) {
                self.game_over = true;
                break;
            }
        }
    }

    /// Draw the game
    fn draw(&self) {
        // Clear the screen
        clear_background(BLACK);

        if self.game_over {
            // Draw game over screen
            let game_over_text = "GAME OVER!";
            let restart_text = "Press R to restart or ESC to quit";
            let final_score_text = format!("Final Score: {}", self.score);

            let game_over_size = measure_text(game_over_text, None, 60, 1.0);
            let restart_size = measure_text(restart_text, None, 30, 1.0);
            let score_size = measure_text(&final_score_text, None, 40, 1.0);

            draw_text(
                game_over_text,
                SCREEN_WIDTH / 2.0 - game_over_size.width / 2.0,
                SCREEN_HEIGHT / 2.0 - 60.0,
                60.0,
                RED,
            );

            draw_text(
                &final_score_text,
                SCREEN_WIDTH / 2.0 - score_size.width / 2.0,
                SCREEN_HEIGHT / 2.0,
                40.0,
                WHITE,
            );

            draw_text(
                restart_text,
                SCREEN_WIDTH / 2.0 - restart_size.width / 2.0,
                SCREEN_HEIGHT / 2.0 + 60.0,
                30.0,
                GRAY,
            );
        } else {
            // Draw game objects
            self.player.draw();

            for bullet in &self.bullets {
                bullet.draw();
            }

            for enemy in &self.enemies {
                enemy.draw();
            }

            // Draw UI
            self.draw_ui();
        }
    }

    /// Draw the user interface (score, instructions)
    fn draw_ui(&self) {
        // Draw score
        let score_text = format!("Score: {}", self.score);
        draw_text(&score_text, 10.0, 30.0, 30.0, WHITE);

        // Draw controls
        let controls = "Controls: Arrow Keys/WASD to move, Space to shoot";
        draw_text(controls, 10.0, SCREEN_HEIGHT - 20.0, 20.0, GRAY);
    }
}

/// Configuration for the game window
fn window_conf() -> Conf {
    Conf {
        window_title: "2D Shooter Game".to_owned(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

/// Main game loop
#[macroquad::main(window_conf)]
async fn main() {
    // Initialize the game
    let mut game = Game::new();

    // Main game loop
    loop {
        // Handle exit condition
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Update game state
        game.update();

        // Draw everything
        game.draw();

        // Wait for the next frame
        next_frame().await;
    }
}
