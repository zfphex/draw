use crate::{vertex, Renderer, Vec2, Vec4, Vertex};
use freetype::face::LoadFlag;
use freetype::{Library, RenderMode};

pub use glow::HasContext;

const GLYPH_METRICS_CAPACITY: usize = 128;
const FONT_SIZE: u32 = 48;

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
    pub fn draw_text(
        &self,
        rd: &mut Renderer,
        text: &str,
        mut x: f32,
        y: f32,
        scale: f32,
        color: Vec4,
    ) {
        // Iterate through all characters
        for c in text.chars() {
            let ch = &self.glyphs[c as usize];

            let xpos = x + ch.bearing.x * scale;
            let ypos = y - (ch.height - ch.bearing.y) * scale;

            let w = ch.width * scale;
            let h = ch.height * scale;

            //TODO: Orthographic projection is not working.
            let vert = [
                vertex!((xpos, ypos + h)),
                vertex!((xpos, ypos)),
                vertex!((xpos + w, ypos)),
                vertex!((xpos, ypos + h)),
                vertex!((xpos + w, ypos)),
                vertex!((xpos + w, ypos + h)),
            ];

            rd.vertices.extend(vert);

            // Advance cursors for the next glyph
            x += (ch.advance.x) as f32 * scale;

            // rd.vertices.extend(vertices);
        }
    }
}

pub unsafe fn load_font(rd: &Renderer, font: &[u8]) -> Atlas {
    let gl = &rd.gl;

    let lib = Library::init().unwrap();
    let face = lib.new_memory_face2(font, 0).unwrap();
    face.set_pixel_sizes(0, FONT_SIZE).unwrap();

    let texture = unsafe { gl.create_texture().unwrap() };
    let mut atlas: Atlas = Atlas {
        width: 0,
        height: 0,
        texture,
        #[allow(invalid_value)]
        glyphs: std::mem::zeroed(),
    };

    //Load symbols, numbers and letters.
    for i in 32..127 {
        face.load_char(i, LoadFlag::RENDER).unwrap();
        let glyph = face.glyph();

        atlas.width += glyph.bitmap().width();

        if atlas.height < glyph.bitmap().rows() {
            atlas.height = glyph.bitmap().rows();
        }

        glyph.render_glyph(RenderMode::Normal).unwrap();
        //Bitshift by 6 to get value in pixels. (2^6 = 64, advance is 1/64 pixels)
        atlas.glyphs[i].advance = Vec2::new(
            (glyph.advance().x >> 6) as f32,
            (glyph.advance().y >> 6) as f32,
        );
        atlas.glyphs[i].width = glyph.bitmap().width() as f32;
        atlas.glyphs[i].height = glyph.bitmap().rows() as f32;
        atlas.glyphs[i].bearing = Vec2::new(glyph.bitmap_left() as f32, glyph.bitmap_top() as f32);
        atlas.glyphs[i].buffer = glyph.bitmap().buffer().to_vec();
    }

    unsafe {
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
            // glow::MIRRORED_REPEAT as i32,
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_T,
            glow::CLAMP_TO_EDGE as i32,
            // glow::MIRRORED_REPEAT as i32,
        );
        gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);
        gl.tex_image_2d(
            glow::TEXTURE_2D,
            0,
            glow::RED as i32,
            atlas.width as i32,
            atlas.height as i32,
            0,
            glow::RED,
            glow::UNSIGNED_BYTE,
            None,
        );

        let mut x = 0;
        for i in 32..127 {
            atlas.glyphs[i].uv = x as f32 / atlas.width as f32;
            let slice = &atlas.glyphs[i].buffer;

            gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                0,
                x,
                0,
                atlas.glyphs[i].width as i32,
                atlas.glyphs[i].height as i32,
                glow::RED,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(slice),
            );

            x += atlas.glyphs[i].width as i32;
        }
    }

    atlas
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
        vertex!(top_right, uv_top_right, color),
        vertex!(top_left, uv_top_left, color),
        vertex!(bottom_left, uv_bottom_left, color),
        vertex!(bottom_left, uv_bottom_left, color),
        vertex!(bottom_right, uv_bottom_right, color),
        vertex!(top_right, uv_top_right, color),
    ];

    rd.vertices.extend(vert);
}
