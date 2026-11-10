use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End,
}

const SCREEN_WIDTH: f32 = 80.0;
const SCREEN_HEIGHT: f32 = 50.0;
const FRAME_DURATION: f32 = 10.0;
const GRAVITY: f32 = 0.2;
const TERMINAL_VELOCITY: f32 = 1.6;

struct State {
    player: Player,
    mode: GameMode,
    frame_time: f32,
    obstacle: Obstacle,
    obstacle_two: Obstacle,
    score: i32,
}

impl State {
    fn new() -> Self {
        Self {
            player: Player::new(5.0, 25.0),
            mode: GameMode::Menu,
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            obstacle_two: Obstacle::new(SCREEN_WIDTH / 2.0, 0),
            score: 0,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap");
        ctx.print(0, 1, format!("Score: {}", self.score));

        self.obstacle.render(ctx, self.player.x);
        self.obstacle_two.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }
        if self.player.x > self.obstacle_two.x {
            self.score += 1;
            self.obstacle_two = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }
        if self.player.y > SCREEN_HEIGHT
            || self.obstacle.hit_obstacle(&self.player)
            || self.obstacle_two.hit_obstacle(&self.player)
        {
            self.mode = GameMode::End;
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5.0, 25.0);
        self.mode = GameMode::Playing;
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.obstacle_two = Obstacle::new(SCREEN_WIDTH / 2.0, 0);
        self.score = 0;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_color_centered(5, GREEN, BLACK, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(6, format!("You earned {} points", self.score));
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

struct Player {
    x: f32,
    y: f32,
    velocity: f32,
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            velocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y as i32, YELLOW, BLACK, to_cp437('@'));
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < TERMINAL_VELOCITY {
            self.velocity += GRAVITY;
        }
        self.y += self.velocity;
        self.x += 0.5;
        if self.y < 0.0 {
            self.y = 0.0;
        }
    }

    fn flap(&mut self) {
        self.velocity = -TERMINAL_VELOCITY;
    }
}

struct Obstacle {
    x: f32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new(x: f32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Self {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: f32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;

        // draw top half of obstacle
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x as i32, y, RED, BLACK, to_cp437('|'));
        }
        // draw the bottom half of the obstacle
        for y in self.gap_y + half_size..SCREEN_HEIGHT as i32 {
            ctx.set(screen_x as i32, y, RED, BLACK, to_cp437('|'));
        }
    }

    fn hit_obstacle(&mut self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x;
        let player_above_gap = player.y < (self.gap_y - half_size) as f32;
        let player_below_gap = player.y > (self.gap_y + half_size) as f32;
        does_x_match && (player_above_gap || player_below_gap)
    }
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .with_tile_dimensions(12, 12)
        .with_fancy_console(80, 50, "terminal8x8.png")
        .build()?;
    main_loop(context, State::new())
}
