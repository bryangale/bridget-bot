use crate::plottable::{PlotPoint, Plottable};

pub struct Curve<'a> {
    function: Box<dyn Fn(f32) -> (f32, f32) + 'a>,
    min_step: f32,
}

struct Segment {
    start_point: PlotPoint,
    end_point: PlotPoint,
    start_t: f32,
    end_t: f32,
}

impl<'a> Curve<'a> {
    pub fn new<F: Fn(f32) -> (f32, f32) + Copy + 'a>(function: F, min_step: f32) -> Curve<'a> {
        Curve {
            function: Box::new(function),
            min_step,
        }
    }
}

impl<'a> Plottable for Curve<'a> {
    fn get_lines(&self) -> Vec<Vec<PlotPoint>> {
        let mut closed = Vec::new();
        let mut open = Vec::new();

        let start_point = get_point(&self.function.as_ref(), 0.0);
        let end_point = get_point(&self.function.as_ref(), 1.0);

        open.push(Segment {
            start_point,
            end_point,
            start_t: 0.0,
            end_t: 1.0,
        });

        while let Some(top) = open.pop() {
            if top.end_t - top.start_t <= self.min_step {
                if top.end_point == top.start_point {
                    continue;
                } else if is_unit(top.start_point, top.end_point) {
                    closed.push(top);
                    continue;
                }
            }

            let mid_t = top.start_t + (top.end_t - top.start_t) * 0.5;
            let mid_point = get_point(&self.function.as_ref(), mid_t);

            let (start_segment, end_segment) = split(&top, mid_point, mid_t);

            open.push(end_segment);
            open.push(start_segment);
        }

        vec![closed
            .into_iter()
            .map(|segment| PlotPoint {
                x: segment.start_point.x,
                y: segment.start_point.y,
            })
            .collect()]
    }
}

fn is_unit(a: PlotPoint, b: PlotPoint) -> bool {
    (b.x - a.x).abs() <= 1 && (b.y - a.y).abs() <= 1
}

fn split(segment: &Segment, mid_point: PlotPoint, mid_t: f32) -> (Segment, Segment) {
    let start_segment = Segment {
        start_point: segment.start_point,
        end_point: mid_point,
        start_t: segment.start_t,
        end_t: mid_t,
    };
    let end_segment = Segment {
        start_point: mid_point,
        end_point: segment.end_point,
        start_t: mid_t,
        end_t: segment.end_t,
    };

    (start_segment, end_segment)
}

fn get_point<F: Fn(f32) -> (f32, f32)>(function: &F, t: f32) -> PlotPoint {
    let x_y = function(t);

    PlotPoint {
        x: x_y.0.round() as i32,
        y: x_y.1.round() as i32,
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::*;

    #[test]
    fn get_lines_works() {
        let circle = |t: f32| {
            let x = (t * 2.0 * PI).cos();
            let y = (t * 2.0 * PI).sin();

            (x, y)
        };

        let result = Curve::new(circle, 0.25).get_lines();

        assert_eq!(
            result,
            vec![vec![
                PlotPoint { x: 1, y: 0 },
                PlotPoint { x: 0, y: 1 },
                PlotPoint { x: -1, y: 0 },
                PlotPoint { x: 0, y: -1 }
            ]]
        );
    }
}
