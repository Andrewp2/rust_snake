use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::graphics::DrawParam;
use ggez::*;
use ggez::{Context, ContextBuilder, GameResult};

use rand::Rng;

const BOARD_WIDTH: usize = 26;
const BOARD_HEIGHT: usize = 26;

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("snake", "Andrew Peterson")
        .build()
        .unwrap();

    let mut my_game = MyGame::new(&mut ctx);
    my_game.put_food();
    ggez::graphics::set_window_title(&ctx, "snake");

    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => (),
        Err(e) => println!("Error occured: {}", e),
    }
    println!("You lost!");
}

struct MyGame {
    board: [TileState; BOARD_WIDTH * BOARD_HEIGHT],
    position: (usize, usize),
    screen_width: f32,
    screen_height: f32,
    facing: Facing,
    prev_facing: Facing,
    snake_length: usize,
    tail: Vec<(usize, usize)>,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        let mut board: [TileState; BOARD_WIDTH * BOARD_HEIGHT] =
            [TileState::Empty; BOARD_WIDTH * BOARD_HEIGHT];
        board[0] = TileState::SnakeHead;
        let (width, height) = graphics::drawable_size(ctx);
        MyGame {
            board: board,
            position: (0, 0),
            screen_width: width,
            screen_height: height,
            facing: Facing::Up,
            prev_facing: Facing::Right,
            snake_length: 3,
            tail: Vec::new(),
        }
    }

    fn put_food(&mut self) {
        let mut rng = rand::thread_rng();
        let mut n: usize = rng.gen_range(0, BOARD_HEIGHT * BOARD_WIDTH);
        while self.board[n] == TileState::SnakeBody || self.board[n] == TileState::SnakeHead {
            n = rng.gen_range(0, BOARD_HEIGHT * BOARD_WIDTH);
        }
        self.board[n] = TileState::Food;
    }

    fn handle_movement(&mut self, ctx: &mut Context) {
        let facing = facing_to_direction(self.facing);
        self.prev_facing = self.facing;
        match self.board[convert_coords(
            (self.position.0 as i32 + facing.0) as usize,
            (self.position.1 as i32 + facing.1) as usize,
        )] {
            TileState::Empty => {
                self.board[convert_coords(
                    (self.position.0 as i32 + facing.0) as usize,
                    (self.position.1 as i32 + facing.1) as usize,
                )] = TileState::SnakeHead;
                self.tail.push((self.position.0, self.position.1));
                self.position.0 = (self.position.0 as i32 + facing.0) as usize;
                self.position.1 = (self.position.1 as i32 + facing.1) as usize;
                for i in &self.tail {
                    self.board[convert_coords(i.0, i.1)] = TileState::Empty;
                }
                if self.tail.len() > self.snake_length {
                    self.tail.drain(0..self.tail.len() - self.snake_length);
                }
                for i in &self.tail {
                    self.board[convert_coords(i.0, i.1)] = TileState::SnakeBody;
                }
            }
            TileState::Food => {
                self.board[convert_coords(
                    (self.position.0 as i32 + facing.0) as usize,
                    (self.position.1 as i32 + facing.1) as usize,
                )] = TileState::SnakeHead;
                self.tail.push((self.position.0, self.position.1));
                self.position.0 = (self.position.0 as i32 + facing.0) as usize;
                self.position.1 = (self.position.1 as i32 + facing.1) as usize;
                self.snake_length += 1;
                for i in &self.tail {
                    self.board[convert_coords(i.0, i.1)] = TileState::Empty;
                }
                if self.tail.len() > self.snake_length {
                    self.tail.drain(0..self.tail.len() - self.snake_length);
                }
                for i in &self.tail {
                    self.board[convert_coords(i.0, i.1)] = TileState::SnakeBody;
                }
                self.put_food();
                //Put another food in
            }
            TileState::SnakeBody => {
                ggez::event::quit(ctx);
            }
            _ => println!("Whaaa?"),
        };
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 5;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            match self.facing {
                Facing::Up => {
                    if self.position.1 == BOARD_HEIGHT - 1 {
                        ggez::event::quit(ctx);
                    } else {
                        self.handle_movement(ctx);
                    }
                }
                Facing::Down => {
                    if self.position.1 == 0 {
                        ggez::event::quit(ctx);
                    } else {
                        self.handle_movement(ctx);
                    }
                }
                Facing::Left => {
                    if self.position.0 == 0 {
                        ggez::event::quit(ctx);
                    } else {
                        self.handle_movement(ctx);
                    }
                }
                Facing::Right => {
                    if self.position.0 == BOARD_WIDTH - 1 {
                        ggez::event::quit(ctx);
                    } else {
                        self.handle_movement(ctx);
                    }
                }
            };
        }
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Escape => {
                ggez::event::quit(ctx);
            }
            KeyCode::Up => {
                if self.prev_facing != Facing::Down {
                    self.facing = Facing::Up
                }
            }
            KeyCode::Down => {
                if self.prev_facing != Facing::Up {
                    self.facing = Facing::Down
                }
            }
            KeyCode::Left => {
                if self.prev_facing != Facing::Right {
                    self.facing = Facing::Left
                }
            }
            KeyCode::Right => {
                if self.prev_facing != Facing::Left {
                    self.facing = Facing::Right
                }
            }
            _ => (),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        for x in 0..BOARD_WIDTH {
            for y in 0..BOARD_HEIGHT {
                let color = match self.board[convert_coords(x, y)] {
                    TileState::Food => graphics::Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    },
                    TileState::SnakeBody => graphics::Color {
                        r: 0.0,
                        g: 0.75,
                        b: 0.0,
                        a: 1.0,
                    },
                    TileState::SnakeHead => graphics::Color {
                        r: 0.0,
                        g: 1.0,
                        b: 0.0,
                        a: 1.0,
                    },
                    TileState::Empty => graphics::BLACK,
                };
                let rect = graphics::Rect::new(
                    x as f32 * self.screen_width / BOARD_WIDTH as f32,
                    (self.screen_height - self.screen_height / BOARD_HEIGHT as f32)
                        - y as f32 * self.screen_height / BOARD_HEIGHT as f32,
                    self.screen_width / BOARD_WIDTH as f32,
                    self.screen_height / BOARD_HEIGHT as f32,
                );
                let r1 =
                    graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, color)?;
                graphics::draw(ctx, &r1, DrawParam::default())?;
            }
        }
        graphics::present(ctx)
    }
}

//x is the position left/right,
//y is up/down
fn convert_coords(x: usize, y: usize) -> usize {
    x + y * BOARD_WIDTH
}

#[derive(Copy, Clone, PartialEq)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

fn facing_to_direction(facing: Facing) -> (i32, i32) {
    match facing {
        Facing::Up => (0, 1),
        Facing::Down => (0, -1),
        Facing::Left => (-1, 0),
        Facing::Right => (1, 0),
    }
}

#[derive(Copy, Clone, PartialEq)]
enum TileState {
    Food,
    Empty,
    SnakeBody,
    SnakeHead,
}
