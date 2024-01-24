use circle::create_circle;
use circle::Range;
use hershey_font::HersheyFont;
use plottable::Plottable;
use plotter::Plotter;
use rand::Rng;
use rand::SeedableRng;
use rand_pcg::Pcg32;
use rectangle::Rectangle;
use remote_plotter::RemotePlotter;
use text::Text;

mod circle;
mod curve;
mod hershey_font;
mod plottable;
mod plotter;
mod rectangle;
mod remote_plotter;
mod text;

pub const PLOTTER_WIDTH: i32 = 24000;
pub const PLOTTER_HEIGHT: i32 = 17440;

pub const PLOTTER_WIDTH_MM: f32 = 300.0;
pub const PLOTTER_HEIGHT_MM: f32 = 218.0;

pub const A4_WIDTH_MM: f32 = 297.0;
pub const A4_HEIGHT_MM: f32 = 210.0;

pub const GRID_CIRCLE_PROBABILITY: f32 = 0.25;
pub const CROSS_CIRCLE_PROBABILITY: f32 = 0.25;

struct Page {
    pub page_border: i32,
    pub page_width: i32,
    pub page_height: i32,
    canvas_border: i32,
    pub canvas_height: i32,
    pub circles_x: i32,
    pub circles_y: i32,
    pub circle_diameter: f32,
}

impl Page {
    fn new() -> Page {
        let mm_to_steps = PLOTTER_WIDTH as f32 / PLOTTER_WIDTH_MM;

        let page_border = (A4_WIDTH_MM * 0.1 * mm_to_steps).round() as i32;

        let page_width = (A4_WIDTH_MM * mm_to_steps).round() as i32 - 2 * page_border;
        let page_height = (A4_HEIGHT_MM * mm_to_steps).round() as i32 - 2 * page_border;

        let canvas_border = (A4_WIDTH_MM * 0.15 * mm_to_steps).round() as i32;

        let canvas_width = (A4_WIDTH_MM * mm_to_steps).round() as i32 - 2 * canvas_border;
        let canvas_height = (A4_HEIGHT_MM * mm_to_steps).round() as i32 - 2 * canvas_border;

        let circles_x = 10;

        let circle_diameter = canvas_width as f32 / circles_x as f32;

        let circles_y = (canvas_height as f32 / circle_diameter).floor() as i32;

        Page {
            page_border,
            page_width,
            page_height,
            canvas_border,
            canvas_height,
            circles_x,
            circles_y,
            circle_diameter,
        }
    }
}

fn main() {
    let plotter = RemotePlotter::new();

    let page = Page::new();

    plot_border(&page, &plotter);
    let seed: u64 = rand::random::<u64>();

    let mut gen = Pcg32::seed_from_u64(seed);

    plot_grid_circles(&page, &mut gen, &plotter);
    plot_cross_circles(&page, &mut gen, &plotter);

    plot_signature(seed, &page, &plotter);
}

fn plot_border(page: &Page, plotter: &RemotePlotter) {
    let page_rectangle = Rectangle::new(
        page.page_border,
        page.page_border + page.page_width,
        page.page_border + page.page_height,
        page.page_border,
    );

    plotter.plot(page_rectangle.get_lines());
}

fn plot_grid_circles(page: &Page, gen: &mut rand_pcg::Lcg64Xsh32, plotter: &RemotePlotter) {
    for i in 0..page.circles_x {
        for j in 0..page.circles_y {
            if gen.gen::<f32>() < GRID_CIRCLE_PROBABILITY {
                let circle = create_circle(
                    (i as f32 + 0.5) * page.circle_diameter + page.canvas_border as f32,
                    (j as f32 + 0.5) * page.circle_diameter + page.canvas_border as f32,
                    page.circle_diameter * 0.5,
                    Range::FullCircle,
                );
                plotter.plot(circle.get_lines());
            }
        }
    }
}

fn plot_cross_circles(page: &Page, gen: &mut rand_pcg::Lcg64Xsh32, plotter: &RemotePlotter) {
    for i in 0..page.circles_x + 1 {
        for j in 0..page.circles_y + 1 {
            let range = if i == 0 && j == 0 {
                Range::QuarterCircleRightBottom
            } else if i == page.circles_x && j == 0 {
                Range::QuarterCircleLeftBottom
            } else if i == page.circles_x && j == page.circles_y {
                Range::QuarterCircleLeftTop
            } else if i == 0 && j == page.circles_y {
                Range::QuarterCircleRightTop
            } else if i == 0 {
                Range::HalfCirlceRight
            } else if j == 0 {
                Range::HalfCircleBottom
            } else if i == page.circles_x {
                Range::HalfCircleLeft
            } else if j == page.circles_y {
                Range::HalfCirleTop
            } else {
                Range::FullCircle
            };

            if gen.gen::<f32>() < CROSS_CIRCLE_PROBABILITY {
                let circle = create_circle(
                    (i as f32) * page.circle_diameter + page.canvas_border as f32,
                    (j as f32) * page.circle_diameter + page.canvas_border as f32,
                    page.circle_diameter * 0.5,
                    range,
                );
                plotter.plot(circle.get_lines());
            }
        }
    }
}

fn plot_signature(seed: u64, page: &Page, plotter: &RemotePlotter) {
    let font = HersheyFont::new("data/rowmans.jhf");

    let printed_max: u128 = u64::MAX as u128 + 1;
    let printed_seed: u128 = seed as u128 + 1;

    let text = Text::new(
        &font,
        format!("bridget-bot {printed_seed}/{printed_max}"),
        page.canvas_border as f32,
        page.canvas_border as f32
            + page.canvas_height as f32
            + ((page.canvas_border - page.page_border) as f32 * 0.5).round(),
        (page.canvas_border - page.page_border) as f32 * 0.25,
    );

    plotter.plot(text.get_lines());
}
