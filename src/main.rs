// use itertools::izip;
use nannou::color::*;
use nannou::prelude::{App, Closed, Draw, Event, Frame, Update};
// use ndarray::prelude::*;
// use ndarray_linalg::Scalar;
use rand::Rng;
use std::time::Duration;
// use std::vec::Vec;

#[derive(Clone, Copy, Default)]
struct Loc {
    x: f32,
    y: f32,
}
impl std::ops::Div<Self> for Loc {
    type Output = Loc;
    fn div(self, other: Self) -> Loc {
        return Loc {
            x: self.x / other.x,
            y: self.y / other.y,
        };
    }
}
impl std::ops::Mul<Self> for Loc {
    type Output = Loc;
    fn mul(self, other: Self) -> Loc {
        return Loc {
            x: self.x * other.x,
            y: self.y * other.y,
        };
    }
}
impl std::ops::Mul<f32> for Loc {
    type Output = Loc;
    fn mul(self, other: f32) -> Loc {
        return Loc {
            x: self.x * other,
            y: self.y * other,
        };
    }
}
impl std::ops::Div<f32> for Loc {
    type Output = Loc;
    fn div(self, other: f32) -> Loc {
        return Loc {
            x: self.x / other,
            y: self.y / other,
        };
    }
}
impl std::ops::Add for Loc {
    type Output = Loc;
    fn add(self, other: Loc) -> Loc {
        return Loc {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
impl std::ops::Sub for Loc {
    type Output = Loc;
    fn sub(self, other: Loc) -> Loc {
        return Loc {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}
impl nannou::prelude::Zero for Loc {
    fn zero() -> Self {
        return Loc { x: 0.0, y: 0.0 };
    }
    fn is_zero(&self) -> bool {
        return self.x == 0.0 && self.y == 0.0;
    }
}

#[derive(Clone, Default)]
struct Charges {
    loc: ndarray::Array1<Loc>,
    q: ndarray::Array1<f32>,
}

struct Model {
    charges: Charges,
    sim_delta: std::time::Duration,
    origin: nannou::prelude::Vec2,
    matrix_x: ndarray::Array2<f32>,
    matrix_y: ndarray::Array2<f32>,
    // slow: bool,
}

// TODO make this less global
const SIM_DELTA: Duration = Duration::new(0, 20_000);
const JUMP: f32 = 1.0;
const NUM_CHARGES: usize = 100;
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
    let mut loc: ndarray::Array1<Loc> = ndarray::Array1::zeros(NUM_CHARGES);
    for l in &mut loc {
        l.x = rng.gen_range(0.0, W);
        l.y = rng.gen_range(0.0, H);
    }
    let q = ndarray::Array::from_elem(NUM_CHARGES, 3.0);
    Model {
        charges: Charges {
            loc: loc,
            q: q,
        },
        sim_delta: std::time::Duration::ZERO,
        origin: nannou::prelude::Vec2::new(-W / 2.0, -H / 2.0),
        matrix_x: ndarray::Array2::zeros((NUM_CHARGES, NUM_CHARGES)),
        matrix_y: ndarray::Array2::zeros((NUM_CHARGES, NUM_CHARGES)),
        // slow: false,
    }
}

fn dir_dist(a: &Loc, b: &Loc) -> Loc {
    let ddd = *a - *b;
    let sq: f32 = ddd.x.powf(2.0) + ddd.y.powf(2.0);
    assert!(sq != 0.0);
    return ddd/sq;
}
fn update_matrix(m: &mut Model) {
    let it = m.charges.loc.iter().zip(m.charges.q.iter()).enumerate();
    for (i, (c_i_loc, c_i_q)) in it.clone() {
        for (j, (c_j_loc, _)) in it.clone() {
            // println!("i {i} j {j}");
            if i == j {
                continue;
            }
            let q = Loc {x: *c_i_q, y: *c_i_q};
            let d = dir_dist(c_i_loc, c_j_loc);
            m.matrix_x[[i, j]] = (q*d).x;
            m.matrix_y[[i, j]] = (q*d).y;
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
                update_matrix(model);
                move_charges(model);
            }
        }
        _ => {}
    }
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
    let draw: Draw = _app.draw();
    let it = _model.charges.loc.iter().zip(_model.charges.q.iter());
    for (loc, charge) in it.clone() {
        let v = nannou::prelude::Vec2::new(loc.x, loc.y);
        draw.rect()
            .color(AZURE)
            .height(*charge)
            .width(*charge)
            .xy(v + _model.origin);
    }

    for x in 0..X_RES {
        for y in 0..Y_RES {
            let here = nannou::prelude::Vec2::new(x as f32 * X_STEP, y as f32 * Y_STEP);
            let mut potential: f32 = 0.0;
            for (loc, charge) in it.clone() {
                let v = nannou::prelude::Vec2::new(loc.x, loc.y);
                potential += charge / v.distance(here);
            }
            potential /= MAX_POT;
            draw.rect()
                .height(X_STEP)
                .width(Y_STEP)
                .xy(here + _model.origin)
                .rgba(1.0, 0.0, 0.0, potential);
            //TODO oversample and average
        }
    }
    // if _model.slow {
    //     draw.text("Slow simulation :(").color(WHITE);
    // }
    draw.background().color(BLACK);
    draw.to_frame(_app, &_frame).unwrap();
}
