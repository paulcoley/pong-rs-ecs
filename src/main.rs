mod ball;
mod button;
mod componentmanager;
mod components;
mod fontmanager;
mod gamestate;
mod mainmenustate;
mod paddle;
mod pongstate;
mod systems;
mod util;

use fontmanager::FontManager;
use gamestate::{GameState, GameStates};
use mainmenustate::MainMenuState;
use pongstate::PongState;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::Sdl;
use sdl2::video::Window;
use std::collections::hash_map::HashMap;
use std::time::{Instant, Duration};
use util::{EventRunner, PlayRc, EventRc};

static mut GAME_STATE: GameStates = GameStates::MainMenu;
static mut PLAY: bool = false;

pub fn main() {
    // Base SDL2 bind classes
    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let canvas = util::rcf(get_canvas(&sdl_context));
    let font_manager = util::rcf(FontManager::new(&ttf_context));

    // Game State data
    let mut game_states: HashMap<GameStates, Box<dyn GameState>> = HashMap::new();

    let event_runner: EventRc = util::rcf(EventRunner {
        event_pump: sdl_context.event_pump().unwrap(),
        event_list: Vec::new()
    });
    let mut now = Instant::now();

    let pong_state = PongState::new(&canvas, &event_runner, &font_manager);
    let menu_state = MainMenuState::new(&canvas, &event_runner, &font_manager);

    game_states.insert(GameStates::Pong, Box::new(pong_state));
    game_states.insert(GameStates::MainMenu, Box::new(menu_state));

    util::set_play(true);

    'running: loop {
        // Sleep for 1/60th of a second, in nanoseconds
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        let delta_time = now.elapsed().as_millis() as f32 / 1_000.0;


        let mut state_key = GameStates::MainMenu;
        unsafe {
            state_key = GAME_STATE;
        }

        if let Some(state) = game_states.get_mut(&state_key) {
            event_runner.borrow_mut().refresh();

            unsafe {
                if !(PLAY) {
                    break 'running;
                }
            }

            canvas.borrow_mut().set_draw_color(Color::RGB(0, 0, 0));
            canvas.borrow_mut().clear();

            state.execute(delta_time);

            canvas.borrow_mut().present();
        }
        else {
            panic!("No game state is specified.");
        }

        now = Instant::now();
    }
}

fn get_canvas(sdl_context: &Sdl) -> Canvas<Window> {
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("rust-sdl2 demo", 1280, 720).position_centered().build().unwrap();
    
    window.into_canvas().build().unwrap()
}