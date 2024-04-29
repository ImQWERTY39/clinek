use crossterm::{
    cursor,
    event::{poll, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{Color, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType},
};
use crossterm::{event, terminal};
use rand::Rng;
use std::{thread, time::Duration};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Apple {
    x: u16,
    y: u16,
}

impl Apple {
    pub fn new_random(width: u16, height: u16) -> Self {
        let mut rand = rand::thread_rng();

        Self {
            x: rand.gen_range(0..width),
            y: rand.gen_range(0..height),
        }
    }
}

struct Snake {
    x: u16,
    y: u16,
    direction: Direction,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            direction: Direction::Right,
        }
    }

    pub fn move_snake(&mut self, max_width: u16, max_height: u16) -> bool {
        match self.direction {
            Direction::Up if self.y > 0 => self.y -= 1,
            Direction::Down if self.y < max_height => self.y += 1,
            Direction::Left if self.x > 0 => self.x -= 1,
            Direction::Right if self.x < max_width => self.x += 1,
            _ => return false,
        }

        true
    }

    pub fn get_char(&self) -> char {
        match self.direction {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        }
    }

    pub fn get_wait_time(&self) -> Duration {
        match self.direction {
            Direction::Up | Direction::Down => Duration::from_millis(75),
            _ => Duration::from_millis(50),
        }
    }
}

fn main() {
    let (window_width, window_height) = terminal::size().unwrap();
    let mut stdout = std::io::stdout();
    terminal::enable_raw_mode().unwrap();
    execute!(stdout, cursor::Hide).unwrap();

    let mut score = 0;
    let mut snake = Snake::new();
    let mut apples: [Apple; 5] = [
        Apple::new_random(window_width, window_height),
        Apple::new_random(window_width, window_height),
        Apple::new_random(window_width, window_height),
        Apple::new_random(window_width, window_height),
        Apple::new_random(window_width, window_height),
    ];

    loop {
        if poll(snake.get_wait_time()).unwrap() {
            match event::read().unwrap() {
                Event::Key(KeyEvent { code, .. }) => match code {
                    KeyCode::Esc => break,
                    KeyCode::Left => snake.direction = Direction::Left,
                    KeyCode::Right => snake.direction = Direction::Right,
                    KeyCode::Up => snake.direction = Direction::Up,
                    KeyCode::Down => snake.direction = Direction::Down,
                    _ => (),
                },
                _ => (),
            }
        }

        if !snake.move_snake(window_width, window_height) {
            break;
        }

        execute!(stdout, Clear(ClearType::All)).unwrap();
        paint_pixel(
            &mut stdout,
            snake.y,
            snake.x,
            snake.get_char(),
            Color::DarkGreen,
            Color::Reset,
        );

        for val in apples.iter_mut() {
            if snake.x == val.x && snake.y == val.y {
                score += 1;
                *val = Apple::new_random(window_width, window_height);
            }

            paint_pixel(&mut stdout, val.y, val.x, '@', Color::Red, Color::Reset);
        }

        thread::sleep(snake.get_wait_time());
    }

    execute!(
        stdout,
        cursor::Show,
        Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .unwrap();
    terminal::disable_raw_mode().unwrap();

    println!("Your score was: {}", score);
}

fn paint_pixel(
    stdout: &mut std::io::Stdout,
    row: u16,
    col: u16,
    symbol: char,
    fg: Color,
    bg: Color,
) {
    execute!(
        stdout,
        cursor::MoveTo(col, row),
        PrintStyledContent(symbol.on(bg).with(fg).bold())
    )
    .unwrap();
}
