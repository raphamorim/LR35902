use crate::gameboy::Gameboy;
use crate::input::KeypadKey;
use std::env;
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{
            self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind,
            KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
            PushKeyboardEnhancementFlags,
        },
        execute,
        terminal::{
            disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
        },
    },
    Terminal,
};

use image::DynamicImage;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Color,
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use ratatui_image::{
    picker::Picker,
    protocol::{Protocol, StatefulProtocol},
    Resize, StatefulImage,
};

const MAX_SCALE: u32 = 4;

pub fn run(gameboy: &mut Gameboy) -> Result<(), Box<dyn Error>> {
    if let Ok(false) = ratatui::crossterm::terminal::supports_keyboard_enhancement() {
        println!("WARN: The terminal doesn't support use_kitty_protocol config.\r");
        return Ok(());
    }

    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        disable_raw_mode().unwrap();
        ratatui::crossterm::execute!(io::stdout(), LeaveAlternateScreen).unwrap();
        original_hook(panic);
    }));

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    execute!(
        stdout,
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::all())
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new(&mut terminal, gameboy);

    // run app
    let res = run_app(&mut terminal, app, gameboy);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        PopKeyboardEnhancementFlags
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    gameboy: &mut Gameboy,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = Duration::from_millis(5);
        if ratatui::crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                let is_pressed = key.kind == KeyEventKind::Press;

                if let KeyCode::Char(c) = key.code {
                    app.on_key(c, gameboy, is_pressed);
                } else if let KeyCode::Up = key.code {
                    if is_pressed {
                        gameboy.keydown(KeypadKey::Up);
                    } else {
                        gameboy.keyup(KeypadKey::Up);
                    }
                } else if let KeyCode::Down = key.code {
                    if is_pressed {
                        gameboy.keydown(KeypadKey::Down);
                    } else {
                        gameboy.keyup(KeypadKey::Down);
                    }
                } else if let KeyCode::Left = key.code {
                    if is_pressed {
                        gameboy.keydown(KeypadKey::Left);
                    } else {
                        gameboy.keyup(KeypadKey::Left);
                    }
                } else if let KeyCode::Right = key.code {
                    if is_pressed {
                        gameboy.keydown(KeypadKey::Right);
                    } else {
                        gameboy.keyup(KeypadKey::Right);
                    }
                }
            }
        }
        if last_tick.elapsed() >= app.tick_rate {
            gameboy.frame();
            app.on_tick(gameboy);
            last_tick = Instant::now();
        }
        if app.should_quit {
            return Ok(());
        }
    }
}

struct App {
    should_quit: bool,
    scale: u32,
    tick_rate: Duration,
    split_percent: u16,

    image_static_offset: (u16, u16),

    picker: Picker,
    image_source: DynamicImage,
    image_static: Protocol,
    image_fit_state: StatefulProtocol,
}

fn size() -> Rect {
    Rect::new(0, 0, 30, 16)
}

#[inline]
fn get_image(gameboy: &mut Gameboy, scale: u32) -> image::DynamicImage {
    // let harvest_moon = "/Users/rapha/harvest-moon.png";
    // image::io::Reader::open(harvest_moon).unwrap().decode().unwrap()

    let width = gameboy.width;
    let height = gameboy.height;

    // Get the raw image data as a vector
    let input: &[u8] = gameboy.image();

    // Allocate a new buffer for the RGB image, 3 bytes per pixel
    let mut output_data = vec![0u8; width as usize * height as usize * 3];

    let mut i = 0;
    // Iterate through 4-byte chunks of the image data (RGBA bytes)
    for chunk in input.chunks(4) {
        // ... and copy each of them to output, leaving out the A byte
        output_data[i..i + 3].copy_from_slice(&chunk[0..3]);
        i += 3;
    }

    let mut buffer = image::ImageBuffer::from_raw(width, height, output_data).unwrap();
    if scale > 1 {
        buffer = image::imageops::resize(
            &buffer,
            width * scale,
            height * scale,
            image::imageops::FilterType::Nearest,
        );
    }
    image::DynamicImage::ImageRgb8(buffer)
}

impl App {
    pub fn new<B: Backend>(_: &mut Terminal<B>, gameboy: &mut Gameboy) -> Self {
        let image_source = get_image(gameboy, 1);

        let mut picker = Picker::from_query_stdio().unwrap();
        picker.set_background_color([0, 0, 0, 0]);

        let image_static = picker
            .new_protocol(image_source.clone(), size(), Resize::Fit(None))
            .unwrap();
        let image_fit_state = picker.new_resize_protocol(image_source.clone());

        Self {
            should_quit: false,
            scale: 1,
            tick_rate: Duration::from_millis(5),
            split_percent: 40,
            picker,
            image_source,

            image_static,
            image_fit_state,

            image_static_offset: (0, 0),
        }
    }
    pub fn on_key(&mut self, c: char, gameboy: &mut Gameboy, is_pressed: bool) {
        match (c, is_pressed) {
            ('q', true) => {
                self.should_quit = true;
            }
            ('i', true) => {
                self.picker
                    .set_protocol_type(self.picker.protocol_type().next());
                self.reset_images();
            }
            ('o', true) => {
                if self.scale >= MAX_SCALE {
                    self.scale = 1;
                } else {
                    self.scale += 1;
                }
            }
            ('H', true) => {
                if self.split_percent >= 10 {
                    self.split_percent -= 10;
                }
            }
            ('L', true) => {
                if self.split_percent <= 90 {
                    self.split_percent += 10;
                }
            }
            ('h', true) => {
                if self.image_static_offset.0 > 0 {
                    self.image_static_offset.0 -= 1;
                }
            }
            ('j', true) => {
                self.image_static_offset.1 += 1;
            }
            ('k', true) => {
                if self.image_static_offset.1 > 0 {
                    self.image_static_offset.1 -= 1;
                }
            }
            ('l', true) => {
                self.image_static_offset.0 += 1;
            }
            ('a', true) | ('A', true) => {
                gameboy.keydown(KeypadKey::A);
            }
            ('b', true) | ('B', true) => {
                gameboy.keydown(KeypadKey::B);
            }
            ('z', true) | ('Z', true) => {
                gameboy.keydown(KeypadKey::Select);
            }
            ('x', true) | ('X', true) => {
                gameboy.keydown(KeypadKey::Start);
            }
            ('a', false) | ('A', false) => {
                gameboy.keyup(KeypadKey::A);
            }
            ('b', false) | ('B', false) => {
                gameboy.keyup(KeypadKey::B);
            }
            ('z', false) | ('Z', false) => {
                gameboy.keyup(KeypadKey::Select);
            }
            ('x', false) | ('X', false) => {
                gameboy.keyup(KeypadKey::Start);
            }
            _ => {}
        }
    }

    fn reset_images(&mut self) {
        self.image_static = self
            .picker
            .new_protocol(self.image_source.clone(), size(), Resize::Fit(None))
            .unwrap();
        self.image_fit_state = self.picker.new_resize_protocol(self.image_source.clone());
    }

    #[inline]
    pub fn on_tick(&mut self, gameboy: &mut Gameboy) {
        self.image_source = get_image(gameboy, self.scale);
        self.image_static = self
            .picker
            .new_protocol(self.image_source.clone(), size(), Resize::Fit(None))
            .unwrap();
        self.image_fit_state = self.picker.new_resize_protocol(self.image_source.clone());
    }

    fn render_resized_image(&mut self, f: &mut Frame<'_>, resize: Resize, area: Rect) {
        let title = format!(
            "Gameboy on {} terminal",
            env::var("TERM").unwrap_or("unknown".to_string())
        );
        let (state, name, _color) = (&mut self.image_fit_state, title, Color::Black);
        let block = block(&name);
        let inner_area = block.inner(area);
        let image = StatefulImage::default().resize(resize);
        f.render_stateful_widget(image, inner_area, state);
        f.render_widget(block, area);
    }
}

fn ui(f: &mut Frame<'_>, app: &mut App) {
    let outer_block = Block::default();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(app.split_percent),
                Constraint::Percentage(100 - app.split_percent),
            ]
            .as_ref(),
        )
        .split(outer_block.inner(f.area()));
    f.render_widget(outer_block, f.area());

    app.render_resized_image(f, Resize::Fit(None), chunks[0]);

    let block_right_bottom = block("Controls");
    let area = block_right_bottom.inner(chunks[1]);
    f.render_widget(
        paragraph(vec![
            Line::from("Controls:"),
            Line::from("arrows: movement"),
            Line::from("Key a/A: A"),
            Line::from("Key s/S: B"),
            Line::from("Key z/Z: select"),
            Line::from("Key x/X: start"),
            Line::from("H/L: resize splits"),
            Line::from(format!("o: scale image (current: {:?})", app.scale)),
            Line::from(format!(
                "i: cycle image protocols (current: {:?})",
                app.picker.protocol_type()
            )),
        ]),
        area,
    );
}

fn paragraph<'a, T: Into<Text<'a>>>(str: T) -> Paragraph<'a> {
    Paragraph::new(str).wrap(Wrap { trim: true })
}

fn block(name: &str) -> Block<'_> {
    Block::default().borders(Borders::ALL).title(name)
}
