use crate::components::{CMovement2D, CCollision2D};
use crate::components::{CPaddleInfo, CTexture, CPosition2D};
use crate::util::CManagerRc;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Instant;

const START_P1: (i32, i32) = (64, 360);
const START_P2: (i32, i32) = (1216, 360);

pub enum Direction {
    Up,
    Down,
    Stationary
}

pub fn create(cmanager: &CManagerRc, is_left: bool, is_ai: bool, name: &str) {
    let mut cmanager = cmanager.borrow_mut();
    let id = cmanager.id_allocator.get_number();

    let collision_2d = CCollision2D {
        id,
        name: name.to_string(),
        size: (10, 100)
    };
    
    let movement_2d = CMovement2D {
        id,
        name: name.to_string(),
        speed: 540.0,
        x: 0.0,
        y: 0.0
    };

    let position_2d: CPosition2D;

    if is_left {
        position_2d = CPosition2D {
            id,
            name: name.to_string(),
            pos: Point::new(START_P1.0, START_P1.1)
        };
    }
    else {
        position_2d = CPosition2D {
            id,
            name: name.to_string(),
            pos: Point::new(START_P2.0, START_P2.1)
        };
    }

    let texture = CTexture {
        id,
        name: name.to_string(),
        size: (10, 100),
        color: Color::WHITE
    };

    let paddle_info = CPaddleInfo {
        id,
        name: name.to_string(),
        direction: Direction::Stationary,
        ai_delay: 125,
        ai_delay_timer: Instant::now(),
        is_ai
    };
    
    cmanager.ccollision_2d.insert(id, collision_2d);
    cmanager.cmovement_2d.insert(id, movement_2d);
    cmanager.cpaddle_info.insert(id, paddle_info);
    cmanager.cposition_2d.insert(id, position_2d);
    cmanager.ctexture.insert(id, texture);
}