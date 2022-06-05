use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Instant;
use crate::paddle::Direction;
use std::collections::hash_map::HashMap;

pub type Components<T> = HashMap<usize, T>;

pub trait Component {
    fn get_info(&self) -> (usize, &str); 
}

pub struct CCollision2D {
    pub id: usize,
    pub name: String,
    pub size: (u32, u32)
}

pub struct CMovement2D {
    pub id: usize,
    pub name: String,
    pub speed: f32,
    pub x: f32,
    pub y: f32
}

pub struct CPaddleInfo {
    pub id: usize,
    pub name: String,
    pub direction: Direction,
    pub ai_delay_timer: Instant,
    pub ai_delay: u128,
    pub is_ai: bool
}

pub struct CPosition2D {
    pub id: usize,
    pub name: String,
    pub pos: Point
}

pub struct CTexture {
    pub id: usize,
    pub name: String,
    pub size: (u32, u32),
    pub color: Color
}

pub struct CText {
    pub id: usize,
    pub name: String,
    pub text: String,
    pub size: u32,
    pub offset: Point,
    pub color: Color
}

pub struct CButtonInfo {
    pub id: usize,
    pub name: String,
    pub text: String,
    pub callback: Option<Box<dyn Fn() -> ()>>
}

impl CPaddleInfo {
    pub fn is_delay_done(&self) -> bool {
        self.ai_delay_timer.elapsed().as_millis() > self.ai_delay
    }
}

impl Component for CCollision2D {
    fn get_info(&self) -> (usize, &str) {
        (self.id, &self.name)
    }
}

impl Component for CPosition2D {
    fn get_info(&self) -> (usize, &str) {
        (self.id, &self.name)
    }
}