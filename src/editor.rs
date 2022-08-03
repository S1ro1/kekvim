use crate::Document;
use crate::Row;
use crate::Terminal;
use std::env;
use termion::event::Key;
use colored::Colorize;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    normal_mode: bool,
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
}

impl Editor {
    pub fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let document = if args.len() > 1 {
            let file_name = &args[1];
            Document::open(&file_name).unwrap_or_default()
        } else {
            Document::default()
        };
        Self { 
            should_quit : false,
            normal_mode : true,
            terminal: Terminal::default().expect("Failed to launch terminal"),
            document,
            cursor_position: Position::default(),
        }
    }

    pub fn run(&mut self) {

        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }

            if self.should_quit {
                Terminal::change_cursor_type("normal");
                break;
            }

            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }

    }
    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Esc => {
                Terminal::change_cursor_type("normal");
                self.normal_mode = true;
            }
            Key::Char('i') => {
                if self.normal_mode {
                    Terminal::change_cursor_type("insert");
                    self.normal_mode = false;
                }
            },

            Key::Char('h') | Key::Char('j') | Key::Char('k') | Key::Char('l') => self.move_cursor(pressed_key),
            _ => (),
        }
        Ok({})
    }

    fn move_cursor(&mut self, key: Key) {
        
        if self.normal_mode {
            let Position { mut x, mut y } = self.cursor_position;

            let size = self.terminal.size();

            let height = size.height.saturating_sub(1) as usize;
            let width = size.width.saturating_sub(1) as usize;

            match key {
                Key::Char('k') => y = y.saturating_sub(1),
                Key::Char('j') => {
                    if y < height {
                        y = y.saturating_add(1);
                    }
                },

                Key::Char('h') => x = x.saturating_sub(1),
                Key::Char('l') => {
                    if x < width {
                        x = x.saturating_add(1);
                    }
                },
                _ => (),
            };

            self.cursor_position = Position{ x, y };

        }
    }

    fn draw_welcome_message(&self) {
        let mut welcome_message = format!("Kekvim -- version {}\r", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();

        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));

        welcome_message = format!("~{}{}", spaces, welcome_message);
        welcome_message.truncate(width);
        println!("{}\r", welcome_message.magenta());
    }

    pub fn draw_row(&self, row: &Row, index: usize) {
        let start = 0;
        let end = self.terminal.size().width as usize;

        let row = row.render(start, end);
        println!("{} {}\r",index.to_string().cyan(), row);
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;

        for terminal_row in 0..height-1 {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(terminal_row as usize) {
                self.draw_row(row, terminal_row as usize);
            } else if terminal_row == height / 3 && self.document.is_empty() {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        if self.normal_mode {
        
        }
        Terminal::cursor_position(&Position::default());
        if self.should_quit {
            Terminal::clear_screen();
            println!("Succesfully exited...");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
