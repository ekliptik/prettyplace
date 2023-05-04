// use itertools::izip;
use nannou::color::*;
use nannou::prelude::WindowEvent::KeyPressed;
use nannou::prelude::{App, Closed, Draw, Event, Frame, Update};
// use ndarray::prelude::*;
// use ndarray_linalg::Scalar;
use rand::Rng;
use std::time::Duration;
// use std::vec::Vec;
mod loc;
use loc::Loc;

#[derive(Clone, Default)]
struct Charges {
    loc: ndarray::Array1<Loc>,
    q: ndarray::Array1<f32>,
}

struct Model {
    charges: Charges,
    sim_delta: std::time::Duration,
    fps_delta: std::time::Duration,
    sim_steps: u64,
    updates: u64,
    fps: u64,
    origin: nannou::prelude::Vec2,
    matrix_x: ndarray::Array2<f32>,
    matrix_y: ndarray::Array2<f32>,
    potential: ndarray::Array2<f32>,
}

// TODO make this less global
const SIM_DELTA: Duration = Duration::new(0, 16_666_666);
const SIMS_PER_DENSITY: u64 = 10;
const JUMP: f32 = 5.0;
const NUM_CHARGES: usize = 200;
const W: f32 = 600.0;
const H: f32 = 600.0;
const X_RES: usize = 16;
const Y_RES: usize = 16;
const MAX_POT: f32 = 14.0 * (NUM_CHARGES as f32 / 100.0);
const X_STEP: f32 = W / X_RES as f32;
const Y_STEP: f32 = H / Y_RES as f32;

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn random_init() -> Model {
    let mut rng = rand::thread_rng();
    let mut loc: ndarray::Array1<Loc> = ndarray::Array1::zeros(NUM_CHARGES);
    for l in &mut loc {
        l.x = rng.gen_range(0.0, W);
        l.y = rng.gen_range(0.0, H);
    }
    let q = ndarray::Array::from_elem(NUM_CHARGES, 3.0);
    Model {
        fps: 0,
        updates: 0,
        charges: Charges { loc: loc, q: q },
        sim_delta: std::time::Duration::ZERO,
        fps_delta: std::time::Duration::ZERO,
        origin: nannou::prelude::Vec2::new(-W / 2.0, -H / 2.0),
        matrix_x: ndarray::Array2::zeros((NUM_CHARGES, NUM_CHARGES)),
        matrix_y: ndarray::Array2::zeros((NUM_CHARGES, NUM_CHARGES)),
        potential: ndarray::Array2::zeros((X_RES, Y_RES)),
        sim_steps: 0,
    }
}

fn model(_app: &App) -> Model {
    random_init()
}

fn dir_dist(a: &Loc, b: &Loc) -> Loc {
    let ddd = *a - *b;
    let sq: f32 = ddd.x.powf(2.0) + ddd.y.powf(2.0);
    if sq == 0.0 {
        return Loc { x: 0.0, y: 0.0 };
    } else {
        return ddd / sq;
    }
}
fn update_matrix(m: &mut Model) {
    let it = m.charges.loc.iter().zip(m.charges.q.iter()).enumerate();
    for (i, (c_i_loc, c_i_q)) in it.clone() {
        for (j, (c_j_loc, _)) in it.clone() {
            if i == j {
                continue;
            }
            let q = Loc {
                x: *c_i_q,
                y: *c_i_q,
            };
            let d = dir_dist(c_i_loc, c_j_loc);
            m.matrix_x[[i, j]] = (q * d).x;
            m.matrix_y[[i, j]] = (q * d).y;
        }
    }
}

fn move_charges(m: &mut Model) {
    let add_x = m.matrix_x.dot(&m.charges.q);
    let add_y = m.matrix_y.dot(&m.charges.q);
    for (i, loc) in &mut m.charges.loc.iter_mut().enumerate() {
        loc.x = (loc.x + add_x[i] * JUMP).max(0.0).min(W);
        loc.y = (loc.y + add_y[i] * JUMP).max(0.0).min(H);
    }
}

fn calc_fps(m: &mut Model) {
    if m.fps_delta > Duration::new(1, 0) {
        m.fps_delta -= Duration::new(1, 0);
        m.fps = m.updates;
        m.updates = 1;
    } else {
        m.updates += 1;
    };
}
fn calc_potential(m: &mut Model) {
    m.sim_steps += 1;
    if m.sim_steps % SIMS_PER_DENSITY == 0 {
        for x in 0..X_RES {
            for y in 0..Y_RES {
                let here = nannou::prelude::Vec2::new(x as f32 * X_STEP, y as f32 * Y_STEP);
                m.potential[[x, y]] = 0.0;
                let it = m.charges.loc.iter().zip(m.charges.q.iter());
                for (loc, charge) in it {
                    let v = nannou::prelude::Vec2::new(loc.x, loc.y);
                    m.potential[[x, y]] += charge / v.distance(here);
                }
                m.potential[[x, y]] /= MAX_POT;
            }
        }
    }
}
fn event(_app: &App, m: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { id: _, simple } => match simple {
            Some(Closed) => std::process::exit(0),
            Some(KeyPressed(keycode)) => match keycode {
                nannou::prelude::Key::R => *m = model(_app),
                _ => {}
            },
            _ => {}
        },
        Event::Update(Update {
            since_last,
            since_start: _,
        }) => {
            m.sim_delta += since_last;
            m.fps_delta += since_last;
            if m.sim_delta > SIM_DELTA {
                m.sim_delta -= SIM_DELTA;
                update_matrix(m);
                move_charges(m);
                calc_potential(m);
            }
            calc_fps(m);
        }
        _ => {}
    }
}

fn view(_app: &App, m: &Model, _frame: Frame) {
    let draw: Draw = _app.draw();
    let it = m.charges.loc.iter().zip(m.charges.q.iter());
    for (loc, charge) in it.clone() {
        let v = nannou::prelude::Vec2::new(loc.x, loc.y);
        draw.rect()
            .color(AZURE)
            .height(*charge)
            .width(*charge)
            .xy(v + m.origin);
    }
    for x in 0..X_RES {
        for y in 0..Y_RES {
            let here = nannou::prelude::Vec2::new(x as f32 * X_STEP, y as f32 * Y_STEP);
            let center = nannou::prelude::Vec2::new(X_STEP / 2.0, Y_STEP / 2.0);
            draw.rect()
                .height(X_STEP)
                .width(Y_STEP)
                .xy(here + m.origin + center)
                .rgba(1.0, 0.0, 0.0, m.potential[[x, y]]);
            //TODO oversample and average
        }
    }
    if m.fps != 0 {
        draw.text(&m.fps.to_string()).y(m.origin.y - 10.0);
    }
    draw.background().color(BLACK);
    draw.to_frame(_app, &_frame).unwrap();
}
