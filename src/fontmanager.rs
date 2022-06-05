use sdl2::ttf::Sdl2TtfContext;
use sdl2::ttf::Font;
use std::path::Path;
use std::collections::hash_map::HashMap;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::rect::Point;
use sdl2::render::TextureQuery;
use crate::util::CanvasRc;

pub struct FontManager<'ttf> {
    ttf_context: &'ttf Sdl2TtfContext,
    fonts: HashMap<String, Font<'ttf, 'ttf>>
}

impl<'ttf> FontManager<'ttf> {
    pub fn new(ttf_context: &'ttf Sdl2TtfContext) -> Self {
        Self {
            ttf_context: ttf_context,
            fonts: HashMap::new()
        }
    }

    pub fn get_font(&mut self, font_name: &str, font_size: u16) -> Option<&Font> {
        let font_name_size = format!("{}-{}", font_name, font_size);
        if !self.fonts.contains_key(&font_name_size) {
            self.load_font(font_name, font_size);
        }
        self.fonts.get(&font_name_size)
    }

    fn load_font(&mut self, font_name: &str, font_size: u16) {
        let font_path_str = format!("fonts/{}.ttf", font_name);
        let font_path = Path::new(&font_path_str);

        if let Ok(font) = self.ttf_context.load_font(font_path, font_size) {
            let font_name = format!("{}-{}", font_name, font_size);
            self.fonts.insert(font_name.clone(), font);
            println!("Loaded {}", font_name);
        }
        else {
            println!("Could not load font.");
        }
    }

    pub fn render_text(&mut self, text: &str, pos: Point, font_name: &str, size: u16, canvas: &CanvasRc, color: Color) {
        let texture_creator = canvas.borrow_mut().texture_creator();
        if let Some(font) = self.get_font(font_name.clone(), size) {
            if let Ok(surface) = font.render(text).blended(color) {
                if let Ok(texture) = texture_creator.create_texture_from_surface(surface) {
                    let TextureQuery { width, height, .. } = texture.query();
                    let dest_rect = Rect::from_center(Point::new(pos.x, pos.y), width, height);
                    canvas.borrow_mut().copy(&texture, None, dest_rect).unwrap();
                }
            }
        }
    }
}