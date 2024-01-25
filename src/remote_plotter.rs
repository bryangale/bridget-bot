use crate::{plottable::PlotPoint, plotter::Plotter};
use reqwest::blocking::Client;
use std::{
    cell::RefCell,
    cmp::{max, min},
    str::FromStr,
};

static URL: &str = "http://localhost:7878/batch-queue";

pub struct RemotePlotter {
    client: Client,
    width: i32,
    height: i32,
    x: RefCell<i32>,
    y: RefCell<i32>,
    commands: RefCell<Vec<String>>,
}

impl RemotePlotter {
    pub fn new(width: i32, height: i32) -> RemotePlotter {
        let result = RemotePlotter {
            client: Client::new(),
            width,
            height,
            x: RefCell::new(0),
            y: RefCell::new(0),
            commands: RefCell::new(Vec::new()),
        };
        result.queue("EM,1,1");
        result.pen_up();
        result
    }

    fn pen_up(&self) {
        self.queue("SP,1,500");
    }

    fn pen_down(&self) {
        self.queue("SP,0,500");
    }

    fn pen_move(&self, x: i32, y: i32) {
        let x = max(min(x, self.width), 0);
        let y = max(min(y, self.height), 0);

        let dx = x - *self.x.borrow();
        let dy = y - *self.y.borrow();

        if dx == 0 && dy == 0 {
            return;
        }

        let (command, dx, dy) = get_move_command(dx, dy);

        self.queue(command.as_str());

        *self.x.borrow_mut() += dx;
        *self.y.borrow_mut() += dy;
    }

    fn queue(&self, command: &str) {
        self.commands
            .borrow_mut()
            .push(String::from_str(command).unwrap());
    }

    fn flush(&self) {
        let result = self
            .client
            .post(URL)
            .body(self.commands.borrow_mut().join("\n"))
            .send();

        match result {
            Ok(_) => {}
            Err(err) => println!("{err}"),
        }

        self.commands.borrow_mut().clear();
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
        self.flush();
    }
}

impl Drop for RemotePlotter {
    fn drop(&mut self) {
        self.pen_move(0, 0);
        self.queue("EM,0,0");
        self.flush();
    }
}

fn get_move_command(dx: i32, dy: i32) -> (String, i32, i32) {
    let mut a = dx + dy;
    let mut b = dx - dy;

    let max_steps = max(a.abs(), b.abs());
    let duration = (max_steps as f32 * 0.5).round() as i32;

    if a != 0 && (duration / 1311) >= a.abs() {
        a = 0;
    }

    if b != 0 && (duration / 1311) >= b.abs() {
        b = 0;
    }

    let dx = (a + b) / 2;
    let dy = (a - b) / 2;

    (format!("SM,{},{},{}", duration, a, b), dx, dy)
}
