use sdl2::pixels::Color;
use sdl2::rect::Point;
use crate::components::{CMovement2D, CCollision2D};
use crate::components::{CTexture, CPosition2D};
use crate::components::Component;
use crate::util::{self, CManagerRc};

pub const START_BALL: (i32, i32) = (640, 360);

pub fn create(cmanager: &CManagerRc) {
    let mut cmanager = cmanager.borrow_mut();
    let id = cmanager.id_allocator.get_number();

    let name = "ball";

    let collision_2d = CCollision2D {
        id,
        name: name.to_string(),
        size: (10, 10)
    };

    let (x, y) = util::random_direction();
    let movement_2d = CMovement2D {
        id: id,
        name: name.to_string(),
        speed: 540.0,
        x,
        y
    };

    let position_2d = CPosition2D {
        id,
        name: name.to_string(),
        pos: Point::new(START_BALL.0, START_BALL.1)
    };

    let texture = CTexture {
        id,
        name: name.to_string(),
        size: (10, 10),
        color: Color::WHITE
    };

    cmanager.ccollision_2d.insert(id, collision_2d);
    cmanager.cmovement_2d.insert(id, movement_2d);
    cmanager.cposition_2d.insert(id, position_2d);
    cmanager.ctexture.insert(id, texture);
}

pub fn get_ball_id(cmanager: &CManagerRc) -> Option<usize> {
    let mut ball_id = None;

    for (_id, component) in &cmanager.borrow().cposition_2d {
        let (pos_id, name) = component.get_info();
        if name.eq_ignore_ascii_case("ball") {
            ball_id = Some(pos_id);
        }
    }

    ball_id
}