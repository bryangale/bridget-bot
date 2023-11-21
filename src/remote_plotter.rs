use crate::{plottable::PlotPoint, plotter::Plotter};
use reqwest::blocking::Client;
use std::{collections::HashMap, thread::sleep, time::Duration};

static PEN_URL: &str = "http://localhost:4242/v1/pen";
static MOTORS_URL: &str = "http://localhost:4242/v1/motors";

pub struct RemotePlotter {
    client: Client,
    x_steps: i32,
    y_steps: i32,
}

impl RemotePlotter {
    pub fn new(x_steps: i32, y_steps: i32) -> RemotePlotter {
        RemotePlotter {
            client: Client::new(),
            x_steps,
            y_steps,
        }
    }

    fn pen_up(&self) {
        let map = HashMap::from([("state", 0)]);
        self.put(&map);
        sleep(Duration::from_millis(300));
    }

    fn pen_down(&self) {
        let map = HashMap::from([("state", 1)]);
        self.put(&map);
    }

    fn pen_move(&self, x: i32, y: i32) {
        let map = HashMap::from([
            ("x", x as f32 * 100.0 / self.x_steps as f32),
            ("y", y as f32 * 100.0 / self.y_steps as f32),
        ]);
        self.put(&map);
    }

    fn put<T: serde::ser::Serialize>(&self, map: &HashMap<&str, T>) {
        let result = self.client.put(PEN_URL).json(&map).send();

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
        let _ = self.client.delete(MOTORS_URL).send();
    }
}
