use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

pub struct Cartpole {
    gravity: f32,
    masspole: f32,
    total_mass: f32,
    length: f32,
    polemass_length: f32,
    force_mag: f32,
    tau: f32,
    state: (f32, f32, f32, f32),
    x_threshold: f32,
    theta_threshold_radians: f32,
    steps_beyond_done: isize
}

impl Cartpole {
    pub fn new() -> Cartpole {
        let gravity =  9.8f32;
        let masscart = 1.0;
        let masspole = 0.1;
        let total_mass = masspole + masscart;
        let length = 0.5;
        let polemass_length = masspole * length;
        let force_mag = 10.0;
        let tau = 0.02;

        Cartpole {
            gravity,
            masspole,
            total_mass,
            length,
            polemass_length,
            force_mag,
            tau,
            state: (0.0f32, 0.0f32, 0.0f32, 0.0f32),
            x_threshold: 2.4f32,
            theta_threshold_radians: 12.0 * 2.0 * std::f32::consts::PI / 360.0f32,
            steps_beyond_done: -1

        }
    }

    pub fn step(&mut self, action: usize) -> ((f32, f32, f32, f32), f32, bool) {
        let force;
        if action == 1 {
            force = self.force_mag;
        }
        else {
            force = -self.force_mag;
        }

        let (x, x_dot, theta, theta_dot) = self.state;
        let costheta = theta.cos();
        let sintheta = theta.sin();

        let temp = (force + self.polemass_length * theta_dot.powf(2.0) * sintheta) / self.total_mass;
        let thetaacc = (self.gravity * sintheta - costheta * temp) / (self.length * (4.0 / 3.0 - self.masspole * costheta.powf(2.0f32) / self.total_mass));
        let xacc = temp - self.polemass_length * thetaacc * costheta / self.total_mass;
        
        let x = x + self.tau * x_dot;
        let x_dot = x_dot + self.tau * xacc;
        let theta = theta + self.tau * theta_dot;
        let theta_dot = theta_dot + self.tau * thetaacc;

        self.state = (x, x_dot, theta, theta_dot);
        let done =     x < -self.x_threshold 
                    || x > self.x_threshold
                    || theta < -self.theta_threshold_radians
                    || theta > self.theta_threshold_radians;

        let reward;
        if !done {
            reward = 1.0f32
        }
        else if self.steps_beyond_done == -1 {
            reward = 1.0f32;
            self.steps_beyond_done = 0;
        }
        else {
            self.steps_beyond_done += 1;
            reward = 0.0f32;
        }
        return (self.state, reward, done)
    }

    pub fn reset(&mut self) -> ((f32, f32, f32, f32), f32, bool) {
        let mut rng = thread_rng();
        let side = Uniform::new(-0.05, 0.05);
        self.steps_beyond_done = -1;
        return ((rng.sample(side), rng.sample(side), rng.sample(side), rng.sample(side)), 1.0f32, false);
    }
}