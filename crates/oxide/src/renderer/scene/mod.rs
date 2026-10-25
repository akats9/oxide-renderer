use crate::{AnySprite, ColorRGBA, Renderer, Sprite};

pub struct Scene {
    sprites: Vec<AnySprite>,
    bg: ColorRGBA,
}

impl Scene {
    pub fn from(sprites: Vec<AnySprite>, bg: ColorRGBA) -> Self {
        Self {
            sprites,
            bg
        }
    }

    pub fn set_bg(&mut self, bg: ColorRGBA) {
        self.bg = bg;
    }

    pub fn push_sprite(&mut self, s: AnySprite) {
        self.sprites.push(s);
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            sprites: Vec::new(),
            bg: ColorRGBA(0,0,0,0),
        }
    }
}

impl Renderer {
    pub fn render_scene(&mut self, scene: Scene) {
        self.clear(scene.bg);
        let mut sprites = scene.sprites;
        sprites.sort_by_key(|s| s.z_index());
        for mut s in sprites {
            s.draw(self);
        }
    }
}