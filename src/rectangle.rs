use crate::plottable::{PlotPoint, Plottable};

pub struct Rectangle {
    top: i32,
    right: i32,
    bottom: i32,
    left: i32,
}

impl Rectangle {
    pub fn new(top: i32, right: i32, bottom: i32, left: i32) -> Rectangle {
        Rectangle {
            top,
            right,
            bottom,
            left,
        }
    }
}

impl Plottable for Rectangle {
    fn get_lines(&self) -> Vec<Vec<PlotPoint>> {
        vec![vec![
            PlotPoint {
                x: self.right,
                y: self.top,
            },
            PlotPoint {
                x: self.right,
                y: self.bottom,
            },
            PlotPoint {
                x: self.left,
                y: self.bottom,
            },
            PlotPoint {
                x: self.left,
                y: self.top,
            },
            PlotPoint {
                x: self.right,
                y: self.top,
            },
        ]]
    }
}
