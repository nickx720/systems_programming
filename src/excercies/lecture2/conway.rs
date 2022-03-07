/// https://guide.nannou.cc/tutorials.html
/// https://reberhardt.com/cs110l/spring-2020/assignments/week-2-exercises/
use nannou::prelude::*;

pub fn main() {
    nannou::app(model).run();
}

struct Model {
    _window: WindowId,
}
fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(512, 512)
        .title("Conway")
        .view(view)
        .event(event)
        .build()
        .unwrap();
    Model { _window }
}

fn event(_app: &App, _model: &mut Model, event: WindowEvent) {
    println!("{event:?}");
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    frame.clear(CORNFLOWERBLUE);
}
