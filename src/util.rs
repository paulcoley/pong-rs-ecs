use crate::GAME_STATE;
use crate::componentmanager::ComponentManager;
use crate::fontmanager::FontManager;
use crate::gamestate::GameStates;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::cell::RefCell;
use std::rc::Rc;

pub type CanvasRc = Rc<RefCell<Canvas<Window>>>;
pub type CManagerRc = Rc<RefCell<ComponentManager>>;
pub type EventRc = Rc<RefCell<EventRunner>>;
pub type FontRc<'ttf> = Rc<RefCell<FontManager<'ttf>>>;
pub type GameStateRc = Rc<RefCell<GameStates>>;
pub type PlayRc = Rc<RefCell<bool>>;

// Shorthand for initializing Rc<RefCell<T>> pattern
pub fn rcf<T>(value: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(value))
}

pub fn create_component_manager() -> CManagerRc {
    rcf(ComponentManager::default())
}

pub fn random_direction() -> (f32, f32) {
    let x = match rand::random() {
        true => 1.0,
        false => -1.0
    };

    let y = match rand::random() {
        true => 1.0,
        false => -1.0
    };

    (x, y)
}

pub fn get_allocated_ids(cmanage: &CManagerRc) -> Vec<usize> {
    cmanage.borrow().id_allocator.get_allocated_ids()
}

pub fn change_gamestate(new_state: GameStates) {
    unsafe {
        GAME_STATE = new_state;
    }
}

pub struct EventRunner {
    pub event_pump: EventPump,
    pub event_list: Vec<Event>
}

impl EventRunner {
    pub fn refresh(&mut self) {
        self.event_list.clear();
        for event in self.event_pump.poll_iter() {
            self.event_list.push(event);
        }
    }
}