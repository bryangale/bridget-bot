use crate::plottable::PlotPoint;

pub trait Plotter {
    fn plot(&mut self, lines: Vec<Vec<PlotPoint>>);
}
