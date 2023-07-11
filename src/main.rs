mod RK4;
mod springs;
use springs::Spring;

use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};


struct Model {
    // window: Window,
    egui: Egui,
    time: f32,
    spring: Spring,
}



fn main() {
    nannou::app(model).update(update).run();
    
}

fn model(app: &App) -> Model {
    let window_id = app.new_window().view(view).raw_event(raw_window_event).build().unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);
    let time = 0.0;
    let spring = Spring::create(1.0, 0.0);
    Model {
        egui,
        time,
        spring
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    egui.set_elapsed_time(update.since_start);

    let ctx = egui.begin_frame();

    egui::Window::new("Rum window").show(&ctx, |ui| {
        ui.label("res");
        let next_step = ui.add(egui::Button::new("next step"));
        if next_step.clicked() {
            model.spring.update(model.time, 1.0);
            model.time += 1.0;
        }
    });
    model.spring.update(model.time, 1.0);
    println!("spring: {:?}", model.spring);
    model.time += 1.0;


}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent){
    model.egui.handle_raw_event(event);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    draw.ellipse().radius(10.0).xy(vec2(model.spring.position,0.0)).color(BLACK);    
    draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
