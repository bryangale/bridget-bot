#[derive(Clone, Copy, Debug)]
pub struct PlotPoint {
    pub x: i32,
    pub y: i32,
}

pub trait Plottable {
    fn get_lines(&self) -> Vec<Vec<PlotPoint>>;
}
