use crate::{ball, util};
use crate::paddle::Direction;
use crate::util::{CanvasRc, EventRc, FontRc, CManagerRc, GameStateRc};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::rc::Rc;
use std::time::Instant;

pub trait System {
    fn execute(&mut self, delta_time: f32);
}

pub struct SysBallMovement {
    cmanager: CManagerRc,
    canvas: CanvasRc
}

pub struct SysPaddleMovement {
    cmanager: CManagerRc,
    event_runner: EventRc,
    canvas: CanvasRc
}

pub struct SysRenderTexture {
    cmanager: CManagerRc,
    canvas: CanvasRc
}

pub struct SysRenderText<'ttf> {
    cmanager: CManagerRc,
    canvas: CanvasRc,
    font_manager: FontRc<'ttf>
}

pub struct SysButtonInput {
    cmanager: CManagerRc,
    event_runner: EventRc,
    canvas: CanvasRc,
}

pub struct SysScoring<'ttf> {
    cmanager: CManagerRc,
    canvas: CanvasRc,
    font_manager: FontRc<'ttf>,
    score_p1: u8,
    score_p2: u8
}

impl<'ttf> SysRenderText<'ttf> {
    pub fn new(cmanager: &CManagerRc, canvas: &CanvasRc, font_manager: &FontRc<'ttf>) -> Self {
        Self {
            cmanager: Rc::clone(cmanager),
            canvas: Rc::clone(canvas),
            font_manager: Rc::clone(font_manager)
        }
    }

    pub fn render(&self, id: &usize) {
        let cmanager = &*self.cmanager.borrow();
        if let Some(text) = cmanager.ctext.get(id) {
            if let Some(position) = cmanager.cposition_2d.get(id) {
                self.font_manager.borrow_mut().render_text(&text.text, position.pos, "arial", 36, &self.canvas, Color::BLACK);
            }
        }
    }
}

impl SysBallMovement {
    pub fn new(cmanager: &CManagerRc, canvas: &CanvasRc) -> Self {
        Self {
            cmanager: Rc::clone(cmanager),
            canvas: Rc::clone(canvas)
        }
    }

    pub fn move_ball(&self, ball_id: &usize, delta_time: f32) {
        let cmanager = &mut *self.cmanager.borrow_mut();
        let position = cmanager.cposition_2d.get_mut(ball_id);
        let movement = cmanager.cmovement_2d.get_mut(ball_id);

        if matches!(position, None) || matches!(movement, None) {
            return;
        }

        let position = position.unwrap();
        let movement = movement.unwrap();
        
        let x_offset = (movement.x * movement.speed * delta_time) as i32;
        let y_offset = (movement.y * movement.speed * delta_time) as i32;
        position.pos = position.pos.offset(x_offset, y_offset);

        let collision = cmanager.ccollision_2d.get_mut(ball_id).unwrap();
        let (_, height) = self.canvas.borrow().window().size();

        let ball_collision_rect = Rect::from_center(position.pos, collision.size.0, collision.size.1);

        if ball_collision_rect.top() < 0 {
            position.pos = Point::new(position.pos.x, (collision.size.1 / 2) as i32);
            movement.y *= -1.0;
        }
        else if ball_collision_rect.bottom() > height as i32 {
            position.pos = Point::new(position.pos.x, (height - (collision.size.1 / 2)) as i32);
            movement.y *= -1.0;
        }

        let ball_collision_rect = Rect::from_center(position.pos, collision.size.0, collision.size.1);
        
        let mut paddle_ids = Vec::new();
        
        for key in cmanager.cpaddle_info.keys() {
            paddle_ids.push(*key);
        }

        for paddle_id in paddle_ids.iter() {
            let collision = cmanager.ccollision_2d.get_mut(paddle_id).unwrap();
            let position = cmanager.cposition_2d.get_mut(paddle_id).unwrap();

            let paddle_collision_rect = Rect::from_center(position.pos, collision.size.0, collision.size.1);
            let intersects = paddle_collision_rect.has_intersection(ball_collision_rect);
            if intersects {
                movement.x *= -1.0;
            }
        }
    }
}

impl SysButtonInput {
    pub fn new(cmanager: &CManagerRc, event_runner: &EventRc, canvas: &CanvasRc) -> Self {
        Self {
            cmanager: Rc::clone(cmanager),
            event_runner: Rc::clone(event_runner),
            canvas: Rc::clone(canvas),
        }
    }

    pub fn handle_input(&self, id: &usize) {
        let cmanager = &*self.cmanager.borrow();

        if matches!(cmanager.cbutton_info.get(id), None) {
            return;
        }

        let button_info = cmanager.cbutton_info.get(id).unwrap();
        let pos = cmanager.cposition_2d.get(id).unwrap();
        let collision = cmanager.ccollision_2d.get(id).unwrap();

        for event in &self.event_runner.borrow().event_list {
            match event {
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
                    let button_rect = Rect::from_center(pos.pos, collision.size.0, collision.size.1);
                    if button_rect.contains_point(Point::new(*x, *y)) {
                        println!("You clicked at {}-{}", x, y);
                        let callback = button_info.callback.as_ref().unwrap();
                        callback();
                    }
                },
                _ => {}
            }
        }
    }
}

impl SysPaddleMovement {
    pub fn new(cmanager: &CManagerRc, event_runner: &EventRc, canvas: &CanvasRc) -> Self {
        Self {
            cmanager: Rc::clone(cmanager),
            event_runner: Rc::clone(event_runner),
            canvas: Rc::clone(canvas)
        }
    }

    pub fn move_paddle(&self, id: &usize, delta_time: f32) {
        let ball_id = ball::get_ball_id(&self.cmanager);

        let cmanager = &mut *self.cmanager.borrow_mut();
        let paddle_info = cmanager.cpaddle_info.get_mut(id).unwrap();

        if paddle_info.is_ai && paddle_info.is_delay_done() && !matches!(ball_id, None) {
            let ball_pos_y = cmanager.cposition_2d.get_mut(&ball_id.unwrap()).unwrap().pos.y;
            let paddle_pos = cmanager.cposition_2d.get_mut(id).unwrap();
            let direction = ball_pos_y - paddle_pos.pos.y;
            if direction < 0 {
                paddle_info.direction = Direction::Up;
            }
            else if direction > 1 {
                paddle_info.direction = Direction::Down;
            }
            else {
                paddle_info.direction = Direction::Stationary;
            }

            paddle_info.ai_delay_timer = Instant::now();
        }
        else {
            for event in &self.event_runner.borrow().event_list {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        //*play = false;
                    },
                    Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                        paddle_info.direction = Direction::Up;
                    },
                    Event::KeyUp { keycode: Some(Keycode::W), .. } => {
                        paddle_info.direction = Direction::Stationary;
                    },
                    Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                        paddle_info.direction = Direction::Down;
                    },
                    Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                        paddle_info.direction = Direction::Stationary;
                    },
                    _ => {}
                }
            }
        }

        let position = cmanager.cposition_2d.get_mut(id).unwrap();
        let movement = cmanager.cmovement_2d.get_mut(id).unwrap();
        let paddle_info = cmanager.cpaddle_info.get_mut(id).unwrap();

        
        match paddle_info.direction {
            Direction::Up => position.pos = position.pos.offset(0, (-movement.speed * delta_time) as i32),
            Direction::Down => position.pos = position.pos.offset(0, (movement.speed * delta_time) as i32),
            _ => ()
        }

        let collision = cmanager.ccollision_2d.get_mut(id).unwrap();
        let (_, height) = self.canvas.borrow().window().size();

        let collision_rect = Rect::from_center(position.pos, collision.size.0, collision.size.1);

        if collision_rect.top() < 0 {
            position.pos = Point::new(position.pos.x, (collision.size.1 / 2) as i32);
        }
        else if collision_rect.bottom() > height as i32 {
            position.pos = Point::new(position.pos.x, (height - (collision.size.1 / 2)) as i32);
        }
    }
}

impl SysRenderTexture {
    pub fn new(cmanager: &CManagerRc, canvas: &CanvasRc) -> Self {
        Self {
            cmanager: Rc::clone(cmanager),
            canvas: Rc::clone(canvas)
        }
    }

    pub fn render(&self, id: &usize) {
        let cmanager = &*self.cmanager.borrow();
        if let Some(texture) = cmanager.ctexture.get(id) {
            if let Some(position) = cmanager.cposition_2d.get(id) {
                let draw_area = Rect::from_center(position.pos, texture.size.0, texture.size.1);

                self.canvas.borrow_mut().set_draw_color(texture.color);
                self.canvas.borrow_mut().fill_rect(draw_area).unwrap();
            }
        }
    }
}

impl<'ttf> SysScoring<'ttf> {
    pub fn new(cmanager: &CManagerRc, canvas: &CanvasRc, font_manager: &FontRc<'ttf>) -> Self {
        Self {
            cmanager: Rc::clone(cmanager),
            canvas: Rc::clone(canvas),
            font_manager: Rc::clone(font_manager),
            score_p1: 0,
            score_p2: 0,
        }
    }

    pub fn check_score(&mut self, ball_id: &usize, _delta_time: f32) {
        let (width, _) = self.canvas.borrow().window().size();

        let cmanager = &mut *self.cmanager.borrow_mut();
        let ball_position = cmanager.cposition_2d.get_mut(ball_id).unwrap();

        let mut has_scored = false;

        if ball_position.pos.x < 0 {
            ball_position.pos = Point::new(ball::START_BALL.0, ball::START_BALL.1);
            self.score_p2 += 1;
            has_scored = true;
        }
        else if ball_position.pos.x > width as i32 {
            ball_position.pos = Point::new(ball::START_BALL.0, ball::START_BALL.1);
            self.score_p1 += 1;
            has_scored = true;
        }

        if has_scored {
            let ball_movement = cmanager.cmovement_2d.get_mut(ball_id).unwrap();
            let (x, y) = util::random_direction();
            ball_movement.x = x;
            ball_movement.y = y;
        }

        let mut font_manager = self.font_manager.borrow_mut();
        let font = "arial";
        let pos_p1 = Point::new(540, 100);
        let pos_p2 = Point::new(740, 100);

        font_manager.render_text(&self.score_p1.to_string(), pos_p1, font, 144, &self.canvas, Color::WHITE);
        font_manager.render_text(&self.score_p2.to_string(), pos_p2, font, 144, &self.canvas, Color::WHITE);
    }
}

impl System for SysBallMovement {
    fn execute(&mut self, delta_time: f32) {
        if let Some(ball_id) = ball::get_ball_id(&self.cmanager) {
            self.move_ball(&ball_id, delta_time);
        }
    }
} 

impl System for SysPaddleMovement {
    fn execute(&mut self, delta_time: f32) {
        let mut ids = Vec::new();
        
        for key in self.cmanager.borrow().cpaddle_info.keys() {
            ids.push(*key);
        }

        for id in &ids[..] {
            self.move_paddle(id, delta_time);
        }
    }
} 

impl System for SysRenderTexture {
    fn execute(&mut self, _delta_time: f32) {
        let ids = util::get_allocated_ids(&self.cmanager);

        for id in &ids[..] {
            self.render(id);
        }
    }
}

impl<'tff> System for SysRenderText<'tff> {
    fn execute(&mut self, _delta_time: f32) {
        let ids = util::get_allocated_ids(&self.cmanager);

        for id in &ids[..] {
            self.render(id);
        }
    }
}

impl<'ttf> System for SysScoring<'ttf> {
    fn execute(&mut self, delta_time: f32) {
        if let Some(ball_id) = ball::get_ball_id(&self.cmanager) {
            self.check_score(&ball_id, delta_time);
        }
    }
}

impl System for SysButtonInput {
    fn execute(&mut self, _delta_time: f32) {
        let ids = util::get_allocated_ids(&self.cmanager);

        for id in &ids[..] {
            self.handle_input(id);
        }
    }
}