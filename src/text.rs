use crate::plottable::{PlotPoint, Plottable};
use hershey_parser::HersheyFont;

pub struct Text<'a> {
    font: &'a HersheyFont,
    string: String,
    x: f32,
    y: f32,
    height: f32,
}

impl<'a> Text<'a> {
    pub fn new(font: &'a HersheyFont, string: String, x: f32, y: f32, height: f32) -> Text<'a> {
        Text {
            font,
            string,
            x,
            y,
            height,
        }
    }
}

impl<'a> Plottable for Text<'a> {
    fn get_lines(&self) -> Vec<Vec<crate::plottable::PlotPoint>> {
        let mut output = Vec::new();

        let scale = self.height / (self.font.bottom - self.font.top) as f32;

        let mut x: f32 = -self
            .font
            .get_glyph(self.string.chars().next().unwrap())
            .unwrap()
            .left as f32
            * scale;
        let y = -(self.font.bottom + self.font.top) as f32 * 0.5 * scale;

        for character in self.string.chars() {
            let glyph = self.font.get_glyph(character).unwrap();

            for stroke in &glyph.paths {
                output.push(
                    stroke
                        .iter()
                        .map(|point| PlotPoint {
                            x: (self.x + x + point.x as f32 * scale).round() as i32,
                            y: (self.y + y + point.y as f32 * scale).round() as i32,
                        })
                        .collect(),
                )
            }

            x += (glyph.right - glyph.left) as f32 * scale;
        }

        output
    }
}
