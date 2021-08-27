use rand::Rng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::console;
use ndarray::{Array2, Array1, Array, Axis, Slice, stack};

const NUMBER_OF_PARTICLES: usize = 1000;
const DT: f64 = 0.001;
const SOFTENING: f64 = 0.001;
const G: f64 = 1.0;

#[wasm_bindgen]
pub struct Universe {
    positions: Array2<f64>,
    velocities: Array2<f64>,
    acceleration: Array2<f64>,
    masses: Array1<f64>
}

impl Universe {
    pub fn get_acceleration(positions: &Array2<f64>, mass: &Array1<f64>, gravity: f64, softening: f64) -> Array2<f64> {
        let x = positions.slice_axis(Axis(1), Slice::from(0..1));
        let y = positions.slice_axis(Axis(1), Slice::from(1..2));
        let z = positions.slice_axis(Axis(1), Slice::from(2..3));

        let dx: Array2<f64> = &x.t() - &x.view();
        let dy: Array2<f64> = &y.t() - &y.view();
        let dz: Array2<f64> = &z.t() - &z.view();

        let mut inv_r3: Array2<f64> = dx.mapv(|a| a.powi(2)) + dy.mapv(|a| a.powi(2)) + dz.mapv(|a| a.powi(2)) + Array1::from_vec(vec![softening; NUMBER_OF_PARTICLES]);
        for mut element in inv_r3.iter_mut() {
            if element > &mut 0.0 {
                element = &mut element.powf(-1.5);
            }
        }

        let ax: Array1<f64> = (Array1::from_vec(vec![gravity; NUMBER_OF_PARTICLES]) * (dx * inv_r3.view())).dot(mass);
        let ay: Array1<f64> = (Array1::from_vec(vec![gravity; NUMBER_OF_PARTICLES]) * (dy * inv_r3.view())).dot(mass);
        let az: Array1<f64> = (Array1::from_vec(vec![gravity; NUMBER_OF_PARTICLES]) * (dz * inv_r3.view())).dot(mass);

        return stack![Axis(1), ax, ay, az];
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let mut rng = rand::thread_rng();
        let mut positions = Array2::zeros((NUMBER_OF_PARTICLES, 3));
        let mut velocities = Array2::zeros((NUMBER_OF_PARTICLES, 3));
        // let mut masses = Array::from_vec(vec![0.2; NUMBER_OF_PARTICLES]);
        let mut masses = Array1::zeros(NUMBER_OF_PARTICLES);
        for i in 0..NUMBER_OF_PARTICLES {
            masses[i] = rng.gen_range(0.1..1.0);
            for j in 0..3 {
                positions[[i, j]] = rng.gen_range(-1.0..1.0);
                velocities[[i, j]] = rng.gen_range(-0.25..0.25);
            }
        }
        let mean_mass: f64 = masses.iter().sum::<f64>() / NUMBER_OF_PARTICLES as f64;
        for i in 0..NUMBER_OF_PARTICLES {
            let mut row = velocities.row_mut(i);
            let mean_velocity = ((row[0] + row[1] + row[2]) * masses[i] / 3.0) / mean_mass;
            row[0] -= mean_velocity;
            row[1] -= mean_velocity;
            row[2] -= mean_velocity;
        }
        let acceleration = Universe::get_acceleration(&positions, &masses, G, SOFTENING);
        Universe {
            positions,
            velocities,
            acceleration,
            masses
        }
    }

    pub fn to_string(&self) {
        for position in self.positions.rows() {
            console::log_3(&JsValue::from_f64(position[0]), &JsValue::from_f64(position[1]), &JsValue::from_f64(position[2]));
        }
    }

    pub fn tick(&mut self) {
        // 1/2 Kick
        self.velocities = &self.velocities + (&self.acceleration * Array::from_elem((NUMBER_OF_PARTICLES, 3), DT/2.0));
        // Update positions and acceleration
        self.positions = &self.positions + (&self.velocities * Array::from_elem((NUMBER_OF_PARTICLES, 3), DT));
        self.acceleration = Universe::get_acceleration(&self.positions, &self.masses, G, SOFTENING);
        // 2/2 Kick
        self.velocities = &self.velocities + (&self.acceleration * Array::from_elem((NUMBER_OF_PARTICLES, 3), DT/2.0));

        let mut total_velocity = 0.0;
        for row in self.velocities.axis_iter(Axis(0)) {
            total_velocity = total_velocity + (row[0].powi(2) + row[1].powi(2) + row[2].powi(2)).sqrt();
        }
        // console::log_1(&JsValue::from_f64(total_velocity));
    }

    pub fn get_positions_ptr(&self) -> *const f64 {
        return self.positions.as_ptr();
    }

    pub fn get_masses_ptr(&self) -> *const f64 {
        return self.masses.as_ptr();
    }
}