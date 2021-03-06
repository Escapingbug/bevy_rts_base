use ab_glyph::{self, Glyph, Point, ScaleFont};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

pub trait FontExtension {
    fn render_text(
        &self,
        text: &str,
        color: Color,
        font_size: f32,
        width: usize,
        height: usize,
    ) -> Image;
}

impl FontExtension for Font {
    fn render_text(
        &self,
        text: &str,
        color: Color,
        font_size: f32,
        width: usize,
        height: usize,
    ) -> Image {
        let scale = ab_glyph::PxScale::from(font_size);

        let scaled_font = ab_glyph::Font::as_scaled(&self.font, scale);

        let mut glyphs = Vec::new();
        layout_paragraph(
            scaled_font,
            ab_glyph::point(0.0, 0.0),
            width as f32,
            text,
            &mut glyphs,
        );

        let color_u8 = [
            (color.r() * 255.0) as u8,
            (color.g() * 255.0) as u8,
            (color.b() * 255.0) as u8,
        ];

        let mut alpha = vec![0.0; width * height];
        for glyph in glyphs {
            if let Some(outlined) = scaled_font.outline_glyph(glyph) {
                let bounds = outlined.px_bounds();
                // Draw the glyph into the image per-pixel by using the draw closure
                outlined.draw(|x, y, v| {
                    // Offset the position by the glyph bounding box
                    // Turn the coverage into an alpha value (blended with any previous)
                    let offset_x = x as usize + bounds.min.x as usize;
                    let offset_y = y as usize + bounds.min.y as usize;
                    if offset_x >= width || offset_y >= height {
                        return;
                    }
                    alpha[offset_y * width + offset_x] = v;
                });
            }
        }

        Image::new(
            Extent3d {
                width: width as u32,
                height: height as u32,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            alpha
                .iter()
                .map(|a| {
                    vec![
                        color_u8[0],
                        color_u8[1],
                        color_u8[2],
                        (color.a() * a * 255.0) as u8,
                    ]
                })
                .flatten()
                .collect::<Vec<u8>>(),
            TextureFormat::Rgba8UnormSrgb,
        )
    }
}

// TODO Move to another file
fn layout_paragraph<F, SF>(
    font: SF,
    position: Point,
    max_width: f32,
    text: &str,
    target: &mut Vec<Glyph>,
) where
    F: ab_glyph::Font,
    SF: ScaleFont<F>,
{
    let v_advance = font.height() + font.line_gap();
    let mut caret = position + ab_glyph::point(0.0, font.ascent());
    let mut last_glyph: Option<Glyph> = None;
    for c in text.chars() {
        if c.is_control() {
            if c == '\n' {
                caret = ab_glyph::point(position.x, caret.y + v_advance);
                last_glyph = None;
            }
            continue;
        }
        let mut glyph = font.scaled_glyph(c);
        if let Some(previous) = last_glyph.take() {
            caret.x += font.kern(previous.id, glyph.id);
        }
        glyph.position = caret;

        last_glyph = Some(glyph.clone());
        caret.x += font.h_advance(glyph.id);

        if !c.is_whitespace() && caret.x > position.x + max_width {
            caret = ab_glyph::point(position.x, caret.y + v_advance);
            glyph.position = caret;
            last_glyph = None;
        }

        target.push(glyph);
    }
}
