use crate::{plottable::PlotPoint, plotter::Plotter};
use std::{
    cmp::{max, min},
    str::FromStr,
};
use tokio::runtime::Runtime;
use tonic::Request;

use self::axidraw_over_http::{axidraw_over_http_client::AxidrawOverHttpClient, Command};

mod axidraw_over_http {
    tonic::include_proto!("axidraw_over_http");
}

pub struct RemotePlotter {
    runtime: Runtime,
    client: AxidrawOverHttpClient<tonic::transport::Channel>,
    width: i32,
    height: i32,
    x: i32,
    y: i32,
    commands: Vec<String>,
}

impl RemotePlotter {
    pub fn new(hostname: String, port_number: u16, width: i32, height: i32) -> RemotePlotter {
        let runtime = Runtime::new().unwrap();

        let client: AxidrawOverHttpClient<tonic::transport::Channel> = runtime
            .block_on(AxidrawOverHttpClient::connect(format!(
                "http://{hostname}:{port_number}"
            )))
            .unwrap();

        let mut result = RemotePlotter {
            runtime,
            client,
            width,
            height,
            x: 0,
            y: 0,
            commands: Vec::new(),
        };
        result.queue("EM,1,1");
        result.pen_up();
        result
    }

    fn pen_up(&mut self) {
        self.queue("SP,1,500");
    }

    fn pen_down(&mut self) {
        self.queue("SP,0,500");
    }

    fn pen_move(&mut self, x: i32, y: i32) {
        let x = max(min(x, self.width), 0);
        let y = max(min(y, self.height), 0);

        let dx = x - self.x;
        let dy = y - self.y;

        if dx == 0 && dy == 0 {
            return;
        }

        let (command, dx, dy) = get_move_command(dx, dy);

        self.queue(command.as_str());

        self.x += dx;
        self.y += dy;
    }

    fn queue(&mut self, command: &str) {
        self.commands.push(String::from_str(command).unwrap());
    }

    fn flush(&mut self) {
        let request = Request::new(tokio_stream::iter(
            self.commands
                .iter()
                .map(|command| Command {
                    contents: command.clone(),
                })
                .collect::<Vec<Command>>(),
        ));

        let result = self.runtime.block_on(self.client.stream(request));

        self.commands.clear();

        match result {
            Ok(_) => {}
            Err(err) => println!("{err}"),
        }
    }
}

impl Plotter for RemotePlotter {
    fn plot(&mut self, lines: Vec<Vec<PlotPoint>>) {
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
