use crate::{button, GAME_STATE};
use crate::gamestate::{GameState, GameStates};
use crate::systems::{SysRenderTexture, System, SysButtonInput, SysRenderText};
use crate::util::{CanvasRc, EventRc, FontRc, PlayRc, GameStateRc, self, CManagerRc};
use sdl2::pixels::Color;
use sdl2::rect::{Rect, Point};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::rc::Rc;

pub struct MainMenuState<'ttf> {
    cmanager: CManagerRc,
    render_system: SysRenderTexture,
    button_input_system: SysButtonInput,
    text_render_system: SysRenderText<'ttf>,
    canvas: CanvasRc,
    event_runner: EventRc,
    play: PlayRc,
}

impl<'ttf> MainMenuState<'ttf> {
    pub fn new(
            canvas: &CanvasRc,
            event_runner: &EventRc,
            font_manager: &FontRc<'ttf>,
            play: &PlayRc) -> Self {
        let cmanager: CManagerRc = util::create_component_manager();
        
        let srender = SysRenderTexture::new(&cmanager, canvas);
        let sbutton_input = SysButtonInput::new(&cmanager, event_runner, canvas);
        let srendertext = SysRenderText::new(&cmanager, canvas, font_manager);

        let main_menu_rect = Rect::from_center(Point::new(640, 360), 192, 96);
        let main_menu_callback = Box::new(|| {
            println!("Main Menu!");
            util::change_gamestate(GameStates::Pong);
        });
        button::create(&cmanager, "bmain_menu", main_menu_rect, Color::WHITE, "Main Menu", Some(main_menu_callback));

        let exit_rect = Rect::from_center(Point::new(640, 472), 192, 96);
        let exit_callback = Box::new(|| println!("Exit Game!"));
        button::create(&cmanager, "bexit", exit_rect, Color::WHITE, "Exit", Some(exit_callback));

        Self {
            cmanager,
            render_system: srender,
            button_input_system: sbutton_input,
            canvas: Rc::clone(&canvas),
            event_runner: Rc::clone(&event_runner),
            play: Rc::clone(&play),
            text_render_system: srendertext
        }
    }

    fn process_input(&mut self){
        for event in &self.event_runner.borrow().event_list {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    let play = &mut *self.play.borrow_mut();
                    *play = false;
                },
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    util::change_gamestate(GameStates::Pong);
                },
                _ => {}
            }
        }
    }
}

/*
        let screen_center = canvas.borrow().window().size().0 as i32 / 2;
        MainMenuState {
            bnew_game: Button::new(screen_center, 280, 175, 100, Color::WHITE),
            bload_game: Button::new(screen_center, 405, 175, 100, Color::WHITE),
            bexit_game: Button::new(screen_center, 530, 175, 100, Color::WHITE)
        }
*/

impl<'ttf> GameState for MainMenuState<'ttf> {
    fn execute(&mut self, delta_time: f32) {
        self.process_input();
        self.button_input_system.execute(delta_time);
        self.render_system.execute(delta_time);
        self.text_render_system.execute(delta_time);
    }
}