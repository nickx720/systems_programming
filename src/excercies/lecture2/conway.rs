use nannou::prelude::*;
use std::collections::{HashMap, HashSet};
/// https://guide.nannou.cc/tutorials.html
/// https://rosettacode.org/wiki/Conway%27s_Game_of_Life#Rust

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

fn life(init: Vec<Cell>, iters: i32, width: i32, height: i32) {
    let mut col: Colony = init.into_iter().collect();
    for i in 0..iters + 1 {
        println!("({i})");
        if i != 0 {
            col = generation(col);
        }
        // Initiate drawing here
        print_colony(&col, width, height);
    }
}

fn print_colony(col: &Colony, width: i32, height: i32) {
    dbg!(width);
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.background().color(CORNFLOWERBLUE);

    let win = app.window_rect();
    draw.tri()
        .points(win.bottom_left(), win.top_left(), win.top_right())
        .color(VIOLET);

    draw.to_frame(app, &frame).unwrap();
}

pub fn main() {
    let blinker = vec![(1, 0), (1, 1), (1, 2)];
    life(blinker, 3, 3, 3);
    let glider = vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)];
    life(glider, 20, 8, 8);
}
