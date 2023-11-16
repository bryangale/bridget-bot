use std::{cmp, fs};

use crate::plottable::PlotPoint;

pub struct HersheyFont {
    pub top: i32,
    pub bottom: i32,
    pub glyphs: Vec<HersheyGlyph>,
}

pub struct HersheyGlyph {
    pub left: i32,
    pub right: i32,
    pub paths: Vec<Vec<PlotPoint>>,
}

impl HersheyFont {
    pub fn new(path: &str) -> HersheyFont {
        let contents = fs::read_to_string(path).unwrap_or_default();

        let glyphs: Vec<_> = contents
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(line_to_hershey_glyph)
            .collect();

        let points_iter = glyphs
            .iter()
            .flat_map(|glyph| glyph.paths.clone())
            .flatten();

        let (top, bottom) =
            points_iter.fold((i32::MAX, i32::MIN), |(accum_top, accum_bottom), point| {
                (
                    cmp::min(accum_top, point.y),
                    cmp::max(accum_bottom, point.y),
                )
            });

        HersheyFont {
            top,
            bottom,
            glyphs,
        }
    }

    pub fn get_glyph(&self, glyph: char) -> &HersheyGlyph {
        self.glyphs.get((glyph as usize) - 32).unwrap()
    }
}

fn line_to_hershey_glyph(line: &str) -> HersheyGlyph {
    let contents = &line[5..];

    let num_pairs = (&contents[..3].trim().parse::<i32>().unwrap() - 1) as usize;

    let left = char_to_int(&contents.chars().nth(3).unwrap());
    let right = char_to_int(&contents.chars().nth(4).unwrap());

    let mut paths = Vec::new();
    let mut path = Vec::new();

    for i in 0..num_pairs {
        let pair = &contents[5 + (i * 2)..7 + (i * 2)];

        if pair == " R" && !path.is_empty() {
            paths.push(path);
            path = Vec::new()
        } else {
            path.push(PlotPoint {
                x: char_to_int(&pair.chars().next().unwrap()),
                y: char_to_int(&pair.chars().nth(1).unwrap()),
            })
        }
    }

    if !path.is_empty() {
        paths.push(path);
    }

    HersheyGlyph { left, right, paths }
}

fn char_to_int(char: &char) -> i32 {
    (*char as i32) - ('R' as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_to_int_works() {
        assert_eq!(char_to_int(&'R'), 0);
        assert_eq!(char_to_int(&'M'), -5);
        assert_eq!(char_to_int(&'W'), 5);
        assert_eq!(char_to_int(&'Q'), -1);
    }
}
