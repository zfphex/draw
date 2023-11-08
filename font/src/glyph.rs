use crate::*;
use freetype::face::LoadFlag;
use freetype::{Library, RenderMode};

pub use glow::HasContext;

const GLYPH_METRICS_CAPACITY: usize = 128;
const FONT_SIZE: u32 = 48;

///https://learnopengl.com/img/in-practice/glyph.png
#[derive(Debug, Clone, Default)]
pub struct Glyph {
    /// Padding
    pub advance: Vec2,
    pub width: f32,
    pub height: f32,
    pub bearing: Vec2,
    /// X offset of glyph in texture.
    pub uv: f32,
    //TODO: Remove
    pub buffer: Vec<u8>,
}

#[derive(Debug)]
pub struct Atlas {
    pub width: i32,
    pub height: i32,
    pub texture: glow::NativeTexture,
    pub glyphs: [Glyph; 128],
}

impl Atlas {
    //TODO: Figure out how to scale a texture.
    pub fn draw_text(&self, rd: &mut Renderer, text: &str, mut x: f32, y: f32, color: Vec4) {
        for c in text.chars() {
            let ch = &self.glyphs[c as usize];

            let xpos = x + ch.bearing.x;
            let ypos = y - (ch.height - ch.bearing.y);

            let w = ch.width;
            let h = ch.height;

            //The y UV is flipped here. !uv.y
            let uv = ch.uv;
            let uv_right = uv + (w / self.width as f32);

            //Top left
            //Bottom left
            //Bottom right

            //Bottom right
            //Top right
            //Top left

            #[rustfmt::skip]
            let vert = [
                vertex!((xpos, ypos + h),     color, (uv, 0.0)),
                vertex!((xpos, ypos),         color, (uv, 1.0)),
                vertex!((xpos + w, ypos),     color, (uv_right, 1.0)),
                vertex!((xpos + w, ypos),     color, (uv_right, 1.0)),
                vertex!((xpos + w, ypos + h), color, (uv_right, 0.0)),
                vertex!((xpos, ypos + h),     color, (uv, 0.0)),
            ];

            rd.vertices.extend(vert);

            // Advance cursors for the next glyph
            x += (ch.advance.x) as f32;
        }
    }
}

pub unsafe fn load_font(rd: &Renderer, font: &[u8]) -> Atlas {
    let gl = &rd.gl;

    let lib = Library::init().unwrap();
    let face = lib.new_memory_face2(font, 0).unwrap();
    face.set_pixel_sizes(0, FONT_SIZE).unwrap();

    let mut width = 0;
    let mut height = 0;
    #[allow(invalid_value)]
    let mut glyphs: [Glyph; 128] = std::mem::zeroed();

    //Load symbols, numbers and letters.
    for i in 32..127 {
        face.load_char(i, LoadFlag::RENDER).unwrap();
        let glyph = face.glyph();
        let bitmap = glyph.bitmap();

        width += bitmap.width();

        if height < bitmap.rows() {
            height = bitmap.rows();
        }

        glyph.render_glyph(RenderMode::Normal).unwrap();
        //Bitshift by 6 to get value in pixels. (2^6 = 64, advance is 1/64 pixels)
        glyphs[i].advance = Vec2::new(
            (glyph.advance().x >> 6) as f32,
            (glyph.advance().y >> 6) as f32,
        );
        glyphs[i].width = bitmap.width() as f32;
        glyphs[i].height = bitmap.rows() as f32;
        glyphs[i].bearing = Vec2::new(glyph.bitmap_left() as f32, glyph.bitmap_top() as f32);
        glyphs[i].buffer = bitmap.buffer().to_vec();
    }

    let texture = unsafe { gl.create_texture().unwrap() };
    gl.active_texture(glow::TEXTURE0);
    gl.bind_texture(glow::TEXTURE_2D, Some(texture));
    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_MAG_FILTER,
        glow::LINEAR as i32,
    );
    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_MIN_FILTER,
        glow::LINEAR as i32,
    );
    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_WRAP_S,
        glow::CLAMP_TO_EDGE as i32,
    );
    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_WRAP_T,
        glow::CLAMP_TO_EDGE as i32,
    );
    gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);
    //If we don't zero this texture, bad things will happen.
    gl.tex_image_2d(
        glow::TEXTURE_2D,
        0,
        glow::RED as i32,
        width as i32,
        height as i32,
        0,
        glow::RED,
        glow::UNSIGNED_BYTE,
        Some(&vec![0; (width * height) as usize]),
        // None,
    );

    let mut x = 0;

    for i in 32..127 {
        glyphs[i].uv = x as f32 / width as f32;

        let slice = glyphs[i].buffer.as_slice();

        gl.tex_sub_image_2d(
            glow::TEXTURE_2D,
            0,
            x,
            0,
            glyphs[i].width as i32,
            glyphs[i].height as i32,
            glow::RED,
            glow::UNSIGNED_BYTE,
            glow::PixelUnpackData::Slice(slice),
        );

        x += glyphs[i].width as i32;
    }

    Atlas {
        width,
        height,
        texture,
        glyphs,
    }
}

pub const RED: Vec4 = Vec4::new(1.0, 0.0, 0.0, 1.0);
pub const GREEN: Vec4 = Vec4::new(0.0, 1.0, 0.0, 1.0);
pub const BLUE: Vec4 = Vec4::new(0.0, 0.0, 1.0, 1.0);

#[allow(unused)]
pub fn draw_character(atlas: &Atlas, rd: &mut Renderer, c: char, x: f32, y: f32, color: Vec4) {
    let mut c = c as usize;
    if c > GLYPH_METRICS_CAPACITY {
        c = '?' as usize;
    }

    let metrics = &atlas.glyphs[c];

    let aw = atlas.width as f32;
    let ah = atlas.height as f32;
    let uv = metrics.uv;

    let scale_x = 1.0 / metrics.width;
    let scale_y = 1.0 / metrics.height;

    let top_left = (0.0, 0.0);
    let top_right = (metrics.advance.x * scale_x, 0.0);
    let bottom_left = (0.0, metrics.height * scale_y);
    let bottom_right = (metrics.advance.x * scale_x, metrics.height * scale_y);

    // let uv_top_left = (tx, 0.0);
    // let uv_top_right = (tx + metrics.bw / aw, 0.0);
    // let uv_bottom_left = (tx, metrics.bh / ah);
    // let uv_bottom_right = (tx + metrics.bw / aw, metrics.bh / ah);

    let uv_top_left = (uv, 1.0);
    let uv_top_right = (uv + metrics.width / aw, 1.0);
    let uv_bottom_left = (uv, 1.0 - (metrics.height / ah));
    let uv_bottom_right = (uv + metrics.width / aw, 1.0 - (metrics.height / ah));

    let vert = [
        vertex!(top_right, color, uv_top_right),
        vertex!(top_left, color, uv_top_left),
        vertex!(bottom_left, color, uv_bottom_left),
        vertex!(bottom_left, color, uv_bottom_left),
        vertex!(bottom_right, color, uv_bottom_right),
        vertex!(top_right, color, uv_top_right),
    ];

    rd.vertices.extend(vert);
}
