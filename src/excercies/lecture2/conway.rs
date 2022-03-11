use nannou::prelude::*;
use std::collections::{HashMap, HashSet};
/// https://guide.nannou.cc/tutorials.html
/// https://rosettacode.org/wiki/Conway%27s_Game_of_Life#Rust
// Todo Return vec from life with all the co-ordinates
type Cell = (i32, i32);
type Colony = HashSet<Cell>;

fn neighbours(&(x, y): &Cell) -> Vec<Cell> {
    vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}

fn neighbour_counts(col: &Colony) -> HashMap<Cell, i32> {
    let mut ncnts = HashMap::new();
    for cell in col.iter().flat_map(neighbours) {
        *ncnts.entry(cell).or_insert(0) += 1;
    }
    ncnts
}

fn generation(col: Colony) -> Colony {
    neighbour_counts(&col)
        .into_iter()
        .filter_map(|(cell, cnt)| match (cnt, col.contains(&cell)) {
            (2, true) | (3, ..) => Some(cell),
            _ => None,
        })
        .collect()
}

fn life(init: Vec<Cell>, iters: i32, width: i32, height: i32, draw: &Draw) {
    let mut col: Colony = init.into_iter().collect();
    for i in 0..iters + 1 {
        if i != 0 {
            col = generation(col);
        }
        // Initiate drawing here
        print_colony(&col, width, height, draw);
    }
}

fn print_colony(col: &Colony, width: i32, height: i32, draw: &Draw) {
    for y in 0..height {
        for x in 0..width {
            if col.contains(&(x, y)) {
                //                draw.ellipse().x_y(1.1, 2.1).w_h(20.0, 20.0).color(RED);
                print!("0")
            } else {
                //draw.rect().x_y(25.1, 25.2).w_h(20.0, 20.0).color(STEELBLUE);
                print!(".")
            }
        }
        println!();
    }
}

pub fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title("Life of Conway")
        .view(view)
        .build()
        .unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PLUM);
    // https://guide.nannou.cc/tutorials/basics/window-coordinates.html
    let blinker = vec![(1, 0), (1, 1), (1, 2)];
    life(blinker, 3, 3, 3, &draw);
    //    let glider = vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
    //    life(glider, 20, 8, 8, &draw);
    draw.to_frame(app, &frame).unwrap();
}
