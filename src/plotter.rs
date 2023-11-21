use crate::plottable::PlotPoint;

pub trait Plotter {
    fn plot(&self, lines: Vec<Vec<PlotPoint>>);
}
