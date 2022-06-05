use crate::components::{CCollision2D, CTexture, CPosition2D, CButtonInfo, CText};
use crate::util::CManagerRc;
use sdl2::rect::Point;
use sdl2::{rect::{Rect}, pixels::Color};

pub fn create(
        cmanager: &CManagerRc,
        name: &str,
        rect: Rect,
        color: Color,
        text: &str,
        callback: Option<Box<dyn Fn() -> ()>>) {
    let mut cmanager = cmanager.borrow_mut();
    let id = cmanager.id_allocator.get_number();

    let collision_2d = CCollision2D {
        id,
        name: name.to_string(),
        size: rect.size()
    };

    let texture = CTexture {
        id,
        name: name.to_string(),
        size: rect.size(),
        color
    };

    let position_2d = CPosition2D {
        id,
        name: name.to_string(),
        pos: rect.center()
    };

    let button_info = CButtonInfo {
        id,
        name: name.to_string(),
        text: text.to_string(),
        callback
    };

    let text_component = CText {
        id,
        name: name.to_string(),
        text: text.to_string(),
        size: 72,
        offset: Point::new(0, 0),
        color: Color::BLACK
    };

    cmanager.ccollision_2d.insert(id, collision_2d);
    cmanager.cposition_2d.insert(id, position_2d);
    cmanager.ctexture.insert(id, texture);
    cmanager.cbutton_info.insert(id, button_info);
    cmanager.ctext.insert(id, text_component);
}

/*
pub struct Button {
    pub callback: Option<Box<dyn Fn() -> ()>>,
    pos: (i32, i32),
    rect: (u32, u32),
    color: Color,
    texture: Option<String>,
    text: Option<String>,
    font_size: u32
}

impl Button {
    pub fn new(x: i32, y: i32, width: u32, height: u32, color: Color) -> Button {
        Button {
            callback: Some(Box::new(|| println!("Hello World!"))),
            pos: (x, y),
            rect: (width, height),
            color: color,
            texture: None,
            text: None,
            font_size: 12
        }
    }
}
 */

/*
impl GameObject for Button {
    fn process_input(&mut self, event_runner: EventRc) { }

    fn update(&mut self, delta_time: f32, canvas: &Canvas<Window>) { }

    fn draw(&self, canvas: CanvasRc) {
        let x_pos = self.pos.0 - self.rect.0 as i32 / 2;
        let y_pos = self.pos.1 - self.rect.1 as i32 / 2;

        canvas.borrow_mut().set_draw_color(self.color);
        canvas.borrow_mut().fill_rect(Rect::new(x_pos, y_pos, self.rect.0, self.rect.1)).unwrap();
    }
}
 */