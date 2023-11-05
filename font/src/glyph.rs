use crate::Renderer;
use freetype::face::LoadFlag;
use freetype::{Library, RenderMode};

pub use glow::HasContext;
pub use nalgebra_glm::{Vec2, Vec3, Vec4};

const GLYPH_METRICS_CAPACITY: usize = 128;
const FONT_SIZE: u32 = 12;

#[derive(Debug, Clone, Default)]
pub struct Glyph {
    /// advance.x
    pub ax: f32,
    /// advance.y
    pub ay: f32,
    /// bitmap.width
    pub bw: f32,
    /// bitmap.rows
    pub bh: f32,
    /// bitmap_left
    pub bl: f32,
    /// bitmap_top
    pub bt: f32,
    /// x offset of glyph in texture coordinates
    pub tx: f32,
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

pub unsafe fn load_font(rd: &Renderer, font: &[u8]) -> Atlas {
    let gl = &rd.gl;

    let lib = Library::init().unwrap();
    let face = lib.new_memory_face2(font, 0).unwrap();
    face.set_pixel_sizes(FONT_SIZE, FONT_SIZE).unwrap();

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
        //Missing SDF which is 5?
        face.load_char(i, LoadFlag::RENDER).unwrap();
        let glyph = face.glyph();

        atlas.width += glyph.bitmap().width();

        if atlas.height < glyph.bitmap().rows() {
            atlas.height = glyph.bitmap().rows();
        }

        glyph.render_glyph(RenderMode::Normal).unwrap();

        atlas.glyphs[i].ax = (glyph.advance().x >> 6) as f32;
        atlas.glyphs[i].ay = (glyph.advance().y >> 6) as f32;
        atlas.glyphs[i].bw = glyph.bitmap().width() as f32;
        atlas.glyphs[i].bh = glyph.bitmap().rows() as f32;
        atlas.glyphs[i].bl = glyph.bitmap_left() as f32;
        atlas.glyphs[i].bt = glyph.bitmap_top() as f32;
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
            atlas.glyphs[i].tx = x as f32 / atlas.width as f32;
            let slice = &atlas.glyphs[i].buffer;

            gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                0,
                x,
                0,
                atlas.glyphs[i].bw as i32,
                atlas.glyphs[i].bh as i32,
                glow::RED,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(slice),
            );

            x += atlas.glyphs[i].bw as i32;
        }
    }

    atlas
}

// pub fn draw_line(atlas: &Atlas, rd: &mut Renderer, text: &str, x: f32, y: f32, color: Vec4) {
//     let chars = text.as_bytes();

//     for c in chars {
//         let mut c = *c as usize;

//         if c > GLYPH_METRICS_CAPACITY {
//             c = '?' as usize;
//         }

//         let metric = &atlas.glyphs[c];
//     }
// }

pub fn draw_character(atlas: &Atlas, rd: &mut Renderer, c: char, x: f32, y: f32, color: Vec4) {
    let mut c = c as usize;
    if c > GLYPH_METRICS_CAPACITY {
        c = '?' as usize;
    }

    let metric = &atlas.glyphs[c];
}
