use crate::{check_error, Renderer};
use freetype::face::LoadFlag;
use freetype::{Library, RenderMode};
use glow::HasContext;
use nalgebra_glm::{Vec2, Vec4};

const GLYPH_METRICS_CAPACITY: usize = 128;

#[derive(Debug)]
pub struct Glyph {
    pub ax: f32, // advance.x
    pub ay: f32, // advance.y
    pub bw: f32, // bitmap.width;
    pub bh: f32, // bitmap.rows;
    pub bl: f32, // bitmap_left;
    pub bt: f32, // bitmap_top;
    pub tx: f32, // x offset of glyph in texture coordinates
    pub buffer: *const [u8],
}

#[derive(Debug)]
pub struct Atlas {
    pub width: i32,
    pub height: i32,
    pub texture: glow::NativeTexture,
    pub glyphs: [Glyph; 128],
}

pub fn load_font(rd: &Renderer, font: &[u8]) -> Atlas {
    let gl = &rd.gl;

    let lib = Library::init().unwrap();
    let face = lib.new_memory_face2(font, 0).unwrap();
    face.set_char_size(40 * 64, 0, 50, 0).unwrap();

    let texture = unsafe { gl.create_texture().unwrap() };
    let mut atlas: Atlas = Atlas {
        width: 0,
        height: 0,
        texture,
        glyphs: unsafe { std::mem::zeroed() },
    };
    let face_height = face.glyph().bitmap().rows();

    // let mut s: Vec<u8> = Vec::new();

    //Load symbols, numbers and letters.
    for i in 32..127 {
        //Missing SDF which is 5?
        face.load_char(i, LoadFlag::RENDER).unwrap();

        let glyph = face.glyph();
        let bitmap = glyph.bitmap();

        atlas.width += bitmap.width();

        if atlas.height < face_height {
            atlas.height = face_height;
        }

        glyph.render_glyph(RenderMode::Normal).unwrap();

        let advance = glyph.advance();

        atlas.glyphs[i].ax = (advance.x >> 6) as f32;
        atlas.glyphs[i].ay = (advance.y >> 6) as f32;
        atlas.glyphs[i].bw = bitmap.width() as f32;
        atlas.glyphs[i].bh = bitmap.rows() as f32;
        atlas.glyphs[i].bl = glyph.bitmap_left() as f32;
        atlas.glyphs[i].bt = glyph.bitmap_top() as f32;
        atlas.glyphs[i].buffer = bitmap.buffer() as *const [u8];

        // s.extend(bitmap.buffer());
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
        );
        gl.tex_parameter_i32(
            glow::TEXTURE_2D,
            glow::TEXTURE_WRAP_T,
            glow::CLAMP_TO_EDGE as i32,
        );

        gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1); //This must match the image format.
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
            // atlas.glyphs[i].tx = i as f32 / atlas.width as f32;
            // let slice = &*atlas.glyphs[i].buffer;
            // assert!(atlas.glyphs[i].bw as i32 >= 0);
            // assert!(atlas.glyphs[i].bh as i32 >= 0);
            // assert!(!(x + atlas.width > atlas.width));
            // assert!(!(0 > atlas.height));

            // gl.tex_sub_image_2d(
            //     glow::TEXTURE_2D,
            //     0,
            //     x,
            //     0,
            //     atlas.glyphs[i].bw as i32,
            //     atlas.glyphs[i].bh as i32,
            //     glow::RED,
            //     glow::UNSIGNED_BYTE,
            //     glow::PixelUnpackData::Slice(slice),
            // );

            // check_error(gl);
            // let error = gl.get_error();
            // if error != 0 {
            //     panic!("\'{}\' {:?}", i as u8 as char, atlas.glyphs[i]);
            // }

            //////////////////////////////////////

            face.load_char(i, LoadFlag::RENDER).unwrap();
            let glyph = face.glyph();
            glyph.render_glyph(RenderMode::Normal).unwrap();

            atlas.glyphs[i].ax = (glyph.advance().x >> 6) as f32;
            atlas.glyphs[i].ay = (glyph.advance().y >> 6) as f32;
            atlas.glyphs[i].bw = glyph.bitmap().width() as f32;
            atlas.glyphs[i].bh = glyph.bitmap().rows() as f32;
            atlas.glyphs[i].bl = glyph.bitmap_left() as f32;
            atlas.glyphs[i].bt = glyph.bitmap_top() as f32;
            atlas.glyphs[i].tx = x as f32 / atlas.width as f32;

            gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);

            gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                0,
                x,
                0,
                glyph.bitmap().width(),
                glyph.bitmap().rows(),
                glow::RED,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(glyph.bitmap().buffer()),
            );

            x += glyph.bitmap().width();
        }

        // dbg!(s.len());
        // gl.tex_image_2d(
        //     glow::TEXTURE_2D,
        //     0,
        //     glow::RED as i32,
        //     atlas.width,
        //     atlas.height,
        //     0,
        //     glow::RED,
        //     glow::UNSIGNED_BYTE,
        //     Some(&s),
        // );

        gl.generate_mipmap(glow::TEXTURE_2D);
    }

    atlas
}

pub fn draw_line(atlas: &Atlas, rd: &mut Renderer, text: &str, mut pos: Vec2, color: Vec4) {
    let chars = text.as_bytes();

    for c in chars {
        let mut c = *c as usize;

        if c > GLYPH_METRICS_CAPACITY {
            c = '?' as usize;
        }

        let metric = &atlas.glyphs[c];

        let x2 = pos.x + metric.bl;
        let y2 = pos.y - metric.bt;
        let w = metric.bw;
        let h = metric.bh;
        pos.x += metric.ax;
        pos.y += metric.ay;

        rd.texture(
            Vec2::new(x2, -y2),
            Vec2::new(w, -h),
            Vec2::new(metric.tx, 0.0),
            Vec2::new(
                metric.bw / atlas.width as f32,
                metric.bh / atlas.height as f32,
            ),
            color,
        );
    }
}
