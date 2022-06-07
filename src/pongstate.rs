use crate::util::{GameStateRc, EventRc, FontRc, PlayRc, CManagerRc, CanvasRc};
use crate::{paddle, ball, util, GAME_STATE};
use crate::gamestate::{GameState, GameStates};
use crate::systems::{System, SysRenderTexture, SysBallMovement, SysPaddleMovement, SysScoring};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::rc::Rc;

pub struct PongState<'a> {
    render_system: SysRenderTexture,
    ball_move_system: SysBallMovement,
    paddle_move_system: SysPaddleMovement,
    scoring_system: SysScoring<'a>,
    canvas: CanvasRc,
    event_runner: EventRc
}

impl<'ttf> PongState<'ttf> {
    pub fn new(
            canvas: &CanvasRc,
            event_runner: &EventRc,
            font_manager: &FontRc<'ttf>) -> Self {
        let cmanager: CManagerRc = util::create_component_manager();

        let srender = SysRenderTexture::new(&cmanager, canvas);
        let sball_move = SysBallMovement::new(&cmanager, canvas);
        let spaddle_move = SysPaddleMovement::new(&cmanager, event_runner, canvas);
        let sscoring = SysScoring::new(&cmanager,canvas, font_manager);

        paddle::create(&cmanager, true, false, "paddle_1");
        paddle::create(&cmanager, false, true, "paddle_2");
        ball::create(&cmanager);

        Self {
            render_system: srender,
            ball_move_system: sball_move,
            paddle_move_system: spaddle_move,
            scoring_system: sscoring,
            canvas: Rc::clone(canvas),
            event_runner: Rc::clone(event_runner),
        }
    }
}

impl PongState<'_> {
    fn process_input(&mut self) {
        for event in &self.event_runner.borrow().event_list {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    util::change_gamestate(GameStates::MainMenu);
                },
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    util::change_gamestate(GameStates::MainMenu);
                },
                _ => {}
            }
        }
    }

    fn render_divider(&mut self) {
        let line_width: u32 = 10;
        let line_height = 50;
        let mut line_iterator: u32 = 0;
        let line_spacing = 25;
        let bottom = self.canvas.borrow().window().size().1;
    
        self.canvas.borrow_mut().set_draw_color(Color::WHITE);
    
        loop {
            if line_iterator > bottom {
                break
            }
            let x_left_pos = 640 - (line_width as i32) / 2;
            let y_top_pos = line_iterator as i32;
            let line = Rect::new(x_left_pos, y_top_pos, line_width, line_height);
    
            self.canvas.borrow_mut().fill_rect(line).unwrap();
            line_iterator = line_height + line_spacing + line_iterator;
        }
    }
}


impl<'ttf> GameState for PongState<'ttf> {
    fn execute(&mut self, delta_time: f32) {
        self.process_input();
        self.paddle_move_system.execute(delta_time);
        self.ball_move_system.execute(delta_time);
        self.scoring_system.execute(delta_time);
        self.render_system.execute(delta_time);

        self.render_divider();
    }
}