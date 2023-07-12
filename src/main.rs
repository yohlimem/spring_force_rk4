mod springs;

use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

const DELTA_TIME: f32 = 0.025;
const TIME_SPEED: f32 = 1.0;
struct Model {
    // window: Window,
    egui: Egui,
    spring: springs::Spring,
    scale: f32,
}

fn main() {
    nannou::app(model).update(update).run();
    
}

fn model(app: &App) -> Model {
    let window_id = app.new_window().view(view).raw_event(raw_window_event).build().unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let spring = springs::Spring::new(0.0, 0.0, 0.1, 3.0, 10.0, 80.0);
    let scale = 0.0;
    Model {
        egui,
        spring,
        scale,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    egui.set_elapsed_time(update.since_start);

    let ctx = egui.begin_frame();

    egui::Window::new("Rum window").show(&ctx, |ui| {
        ui.label("attributes");
        ui.add(egui::Slider::new(&mut model.spring.m, 1.0..=100.0).text("Mass"));
        ui.add(egui::Slider::new(&mut model.spring.len, 1.0..=100.0).text("Length"));
        ui.add(egui::Slider::new(&mut model.spring.k, 1.0..=100.0).text("Stiffness"));
        ui.add(egui::Slider::new(&mut model.spring.dampen, 0.0..=1.0).text("dampening"));
    });

    for _ in 0..(TIME_SPEED / DELTA_TIME) as u32 {
        model.scale += DELTA_TIME;
        model.spring.step_v(DELTA_TIME);
        model.spring.step_x(DELTA_TIME);

        // println!("spring: {:?}", model.spring);

    }


}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent){
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    draw.ellipse()
    .color(BLACK)
    .radius(model.spring.m)
    .xy(vec2(model.spring.x, 0.0));

    draw.line().start(vec2(0.0, 0.0)).end(vec2(model.spring.len, 0.0)).color(BLACK).stroke_weight(10.0);
    
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
