use std::f32::consts::PI;

use crate::curve::Curve;

pub enum Range {
    FullCircle,
    HalfCirleTop,
    HalfCirlceRight,
    HalfCircleBottom,
    HalfCircleLeft,
    QuarterCircleRightTop,
    QuarterCircleRightBottom,
    QuarterCircleLeftBottom,
    QuarterCircleLeftTop,
}

pub fn create_circle<'a>(x: f32, y: f32, radius: f32, range: Range) -> Curve<'a> {
    let (start, end) = match range {
        Range::FullCircle => (0.0, 2.0 * PI),
        Range::HalfCirleTop => (PI, 2.0 * PI),
        Range::HalfCirlceRight => (1.5 * PI, 2.5 * PI),
        Range::HalfCircleBottom => (0.0, PI),
        Range::HalfCircleLeft => (0.5 * PI, 1.5 * PI),
        Range::QuarterCircleRightTop => (1.5 * PI, 2.0 * PI),
        Range::QuarterCircleRightBottom => (0.0, 0.5 * PI),
        Range::QuarterCircleLeftBottom => (0.5 * PI, PI),
        Range::QuarterCircleLeftTop => (PI, 1.5 * PI),
    };

    Curve::new(
        move |t| {
            let a: f32 = start + (end - start) * t;
            (x + radius * a.cos(), y + radius * a.sin())
        },
        0.1,
    )
}
