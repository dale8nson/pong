extern crate sdl2;

use sdl2::{
    event::Event,
    keyboard::{Keycode, Scancode},
    pixels::Color,
    rect::Rect,
    render::WindowCanvas,
    video::Window,
    EventPump, Sdl, TimerSubsystem,
};

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;
const THICKNESS: u32 = 15;
const PADDLE_LENGTH: u32 = 100;

pub struct Game {
    sdl_context: Sdl,
    is_running: bool,
    is_paused: bool,
    timer: Option<TimerSubsystem>,
    ticks: u32,
    canvas: WindowCanvas,
    event_pump: EventPump,
    paddle_pos: Vector2,
    ball_pos: Vector2,
    ball_vel: Vector2,
}

impl Game {
    pub fn new() -> Game {
        let is_running = false;
        let is_paused = false;
        let sdl_context = sdl2::init().unwrap();
        let timer: Option<TimerSubsystem> = None;
        let ticks = 0u32;
        let video_subsystem = sdl_context.video().unwrap();
        let window: Window = video_subsystem
            .window("Pong!", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .unwrap();
        let canvas: WindowCanvas = window.into_canvas().build().unwrap();
        let event_pump: EventPump = sdl_context.event_pump().unwrap();

        let ball_vel = Vector2 { x: -200., y: 235. };

        Game {
            is_running,
            is_paused,
            sdl_context,
            timer,
            ticks,
            canvas,
            event_pump,
            ball_pos: Vector2 {
                x: 1024f32 / 2.,
                y: 768f32 / 2.,
            },
            ball_vel,
            paddle_pos: Vector2 {
                x: THICKNESS as f32 / 2.,
                y: SCREEN_HEIGHT as f32 / 2.,
            },
        }
    }

    pub fn initialize(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(0, 0, 255, 255));
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn run_loop(&mut self) {
        self.timer = Some(self.sdl_context.timer().unwrap());
        self.is_running = true;
        while self.is_running {
            self.process_input();
            if self.is_paused {
                self.ticks +=
                    ((self.timer.as_ref().unwrap().ticks() - self.ticks) as f32 / 1000.) as u32;
                continue;
            }
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

                Event::KeyDown {
                    scancode: Some(Scancode::Space),
                    ..
                } => {
                    self.is_paused = !self.is_paused;
                }
                _ => {}
            }
        }
    }

    pub fn update_game(&mut self) {
        let timer = self.timer.as_ref().unwrap();
        while timer.ticks() < self.ticks + 16 {}
        let mut dt = (timer.ticks() - self.ticks) as f32 / 1000f32;
        self.ticks = timer.ticks();

        if dt > 0.05 {
            dt = 0.05;
        }

        let mut pd = 0;
        let ks = self.event_pump.keyboard_state();

        if ks.is_scancode_pressed(Scancode::W) {
            pd -= 1;
        }
        if ks.is_scancode_pressed(Scancode::S) {
            pd += 1;
        }

        let pp = &mut self.paddle_pos;

        if pd != 0 {
            pp.y += pd as f32 * 300. * dt;
        }

        if pp.y - PADDLE_LENGTH as f32 / 2. < 0. + THICKNESS as f32 {
            pp.y = PADDLE_LENGTH as f32 / 2. + THICKNESS as f32;
        }
        if pp.y + PADDLE_LENGTH as f32 / 2. > 768. - THICKNESS as f32 {
            pp.y = 768. - PADDLE_LENGTH as f32 / 2. - THICKNESS as f32;
        }

        let bp = &mut self.ball_pos;
        let bv = &mut self.ball_vel;

        if bp.y <= THICKNESS as f32 && bv.y < 0. {
            bv.y *= -1.;
        }

        if bp.y >= SCREEN_HEIGHT as f32 - THICKNESS as f32 && bv.y >= 0. {
            bv.y *= -1.;
        }

        if bp.x >= SCREEN_WIDTH as f32 - THICKNESS as f32 && bv.x >= 0. {
            bv.x *= -1.;
        }

        let diff = (pp.y - bp.y).abs();

        if diff <= PADDLE_LENGTH as f32 / 2. && bp.x <= 25. && bp.x >= 20. && bv.x < 0. {
            bv.x *= -1.;
        }

        bp.x += bv.x * dt;
        bp.y += bv.y * dt;
    }

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

        let pp = &self.paddle_pos;

        let paddle = Rect::new(
            pp.x as i32 - THICKNESS as i32 / 2,
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
