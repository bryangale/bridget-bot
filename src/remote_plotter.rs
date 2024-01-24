use crate::{plottable::PlotPoint, plotter::Plotter};
use reqwest::blocking::Client;
use std::{cmp::max, str::FromStr, thread::sleep, time::Duration};

static URL: &str = "http://localhost:7878";

pub struct RemotePlotter {
    client: Client,
}

impl RemotePlotter {
    pub fn new() -> RemotePlotter {
        let result = RemotePlotter {
            client: Client::new(),
        };
        result.push("EM,0,0");
        result
    }

    fn pen_up(&self) {
        self.push("SP,1");
        sleep(Duration::from_millis(300));
    }

    fn pen_down(&self) {
        self.push("SP,0");
    }

    fn pen_move(&self, x: i32, y: i32) {
        let a = (x + y).abs();
        let b = (x - y).abs();

        let max_steps = max(a, b);
        let duration = (max_steps as f32 * 0.5).round() as i32;

        self.push(format!("XM,{},{},{}", duration, x, y).as_str());
    }

    fn push(&self, command: &str) {
        let result = self
            .client
            .post(URL)
            .body(String::from_str(command).unwrap())
            .send();

        match result {
            Ok(_) => {}
            Err(err) => println!("{err}"),
        }
    }
}

impl Plotter for RemotePlotter {
    fn plot(&self, lines: Vec<Vec<PlotPoint>>) {
        for line in &lines {
            let mut iter = line.iter();
            match iter.next() {
                Some(point) => {
                    self.pen_move(point.x, point.y);
                    self.pen_down();
                }
                None => continue,
            }
            iter.for_each(|point| self.pen_move(point.x, point.y));
            self.pen_up();
        }
    }
}

impl Drop for RemotePlotter {
    fn drop(&mut self) {
        self.pen_move(0, 0);
        self.push("EM,0,0");
    }
}
