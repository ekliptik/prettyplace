use nannou::prelude::{App, Closed, Draw, Event, Frame, Update};
use nannou::color::*;
use rand::Rng;
use std::time::Duration;
use std::vec::Vec;
use ndarray::prelude::*;

struct Charge {
    r: nannou::prelude::Vec2,
    q: f32,
}

struct Model {
    charges: Vec<Charge>,
    sim_delta: std::time::Duration,
    origin: nannou::prelude::Vec2,
    matrix: ndarray::Array2<f32>,
}

const SIM_DELTA: Duration = Duration::new(0, 10_000);
const JUMP: f32 = 1.0;
const NUM_CHARGES: usize = 40;
const W: f32 = 600.0;
const H: f32 = 600.0;
const X_RES: i64 = 16;
const Y_RES: i64 = 16;
const MAX_POT: f32 = 14.0;
const X_STEP: f32 = W / X_RES as f32;
const Y_STEP: f32 = H / Y_RES as f32;

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    let mut rng = rand::thread_rng();
    let mut charges: Vec<Charge> = Vec::new();
    for _ in 0..NUM_CHARGES {
        charges.push(Charge {
            r: nannou::prelude::Vec2::new(rng.gen_range(0.0..W), rng.gen_range(0.0..H)),
            q: 3.0,
        })
    }
    Model {
        charges,
        sim_delta: std::time::Duration::ZERO,
        origin: nannou::prelude::Vec2::new(- W / 2.0, - H / 2.0),
        matrix: ndarray::Array2::zeros((NUM_CHARGES, NUM_CHARGES)),
    }
}

fn move_charges(charges: &mut Vec<Charge>) {
    for charge in charges {
        charge.r.x += 1.0;
        charge.r.x = charge.r.x.max(0.0).min(W); //TODO
        charge.r.y = charge.r.y.max(0.0).min(H);
    }
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { id: _, simple } => match simple {
            Some(x) => match x {
                Closed => std::process::exit(0),
                _ => {}
            },
            None => {}
        },
        Event::Update(Update {
            since_last,
            since_start: _,
        }) => {
            model.sim_delta += since_last;
            if model.sim_delta > SIM_DELTA {
                model.sim_delta -= SIM_DELTA;
                // build_matrix(&mut model.charges, model.matrix;
                move_charges(&mut model.charges);
            }
        }
        _ => {}
    }
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
    let draw: Draw = _app.draw();
    for charge in &_model.charges {
        draw.rect()
            .color(STEELBLUE)
            .height(charge.q)
            .width(charge.q)
            .xy(charge.r + _model.origin);
    }

    for x in 0..X_RES {
        for y in 0..Y_RES {
            let here = nannou::prelude::Vec2::new(
                x as f32 * X_STEP,
                y as f32 * Y_STEP,
            );
            let mut potential: f32 = 0.0;
            for charge in &_model.charges {
                potential += charge.q / charge.r.distance(here);
            }
            potential /= MAX_POT;
            draw.rect()
                .height(X_STEP)
                .width(Y_STEP)
                .xy(here + _model.origin)
                .rgba(1.0, 0.0, 0.0, potential);
        }
    }
    draw.background().color(BLACK);
    draw.to_frame(_app, &_frame).unwrap();
}
