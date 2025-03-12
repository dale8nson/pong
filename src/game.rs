extern crate sdl2;

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::WindowCanvas,
    sys::{SDL_CreateRenderer, SDL_Renderer, SDL_RendererFlags},
    video::Window,
    EventPump, Sdl, VideoSubsystem,
};

const THICKNESS: u32 = 15;
const PADDLE_LENGTH: u32 = 200;

pub struct Game {
    is_running: bool,
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    canvas: WindowCanvas,
    event_pump: EventPump,
    paddle_pos: Vector2,
    ball_pos: Vector2,
}

impl Game {
    pub fn new() -> Game {
        let is_running = false;
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window: Window = video_subsystem
            .window("Pong!", 1024, 768)
            .position_centered()
            // .fullscreen_desktop()
            .build()
            .unwrap();
        let canvas: WindowCanvas = window.into_canvas().build().unwrap();
        let event_pump: EventPump = sdl_context.event_pump().unwrap();

        Game {
            is_running,
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
            ball_pos: Vector2 {
                x: 1024f32 / 2.,
                y: 768f32 / 2.,
            },
            paddle_pos: Vector2 {
                x: THICKNESS as f32 / 2.,
                y: 768f32 / 2.,
            },
        }
    }

    pub fn initialize(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(0, 0, 255, 255));
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn run_loop(&mut self) {
        self.is_running = true;
        while self.is_running {
            self.process_input();
            self.update_game();
            self.generate_output();
        }
    }

    fn process_input(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.is_running = false;
                }
                _ => {}
            }
        }
    }

    pub fn update_game(&mut self) {}

    pub fn generate_output(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(0, 0, 255, 255));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));

        let top_wall = Rect::new(0, 0, 1024, THICKNESS);
        let bottom_wall = Rect::new(0, 768 - THICKNESS as i32, 1024, THICKNESS);
        let right_wall = Rect::new(1024 - THICKNESS as i32, 0, THICKNESS, 768);
        let ball = Rect::new(
            self.ball_pos.x as i32 - THICKNESS as i32 / 2,
            self.ball_pos.y as i32 - THICKNESS as i32 / 2,
            THICKNESS,
            THICKNESS,
        );

        let pp = self.paddle_pos.clone();

        let paddle = Rect::new(
            pp.x as i32 - pp.x as i32 / THICKNESS as i32,
            pp.y as i32 - PADDLE_LENGTH as i32 / 2,
            THICKNESS,
            PADDLE_LENGTH,
        );

        self.canvas.fill_rect(top_wall).ok().unwrap();
        self.canvas.fill_rect(bottom_wall).ok().unwrap();
        self.canvas.fill_rect(right_wall).ok().unwrap();
        self.canvas.fill_rect(ball).ok().unwrap();
        self.canvas.fill_rect(paddle).ok().unwrap();
        self.canvas.present();
    }
}

#[derive(Clone)]
struct Vector2 {
    x: f32,
    y: f32,
}
