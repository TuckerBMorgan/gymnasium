use ndarray::prelude::*;
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;
use crate::enviroment::*;
use crate::renderer::*;

pub struct Cartpole {
    gravity: f32,
    masspole: f32,
    total_mass: f32,
    length: f32,
    polemass_length: f32,
    force_mag: f32,
    tau: f32,
    state: Array2<f32>,
    x_threshold: f32,
    theta_threshold_radians: f32,
    steps_beyond_done: isize,
    renderer: Renderer
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
            state: Array2::<f32>::zeros((1, 4)),
            x_threshold: 2.4f32,
            theta_threshold_radians: 12.0 * 2.0 * std::f32::consts::PI / 360.0f32,
            steps_beyond_done: -1,
            renderer: Renderer::new(600, 400)
        }
    }
}

impl Enviroment for Cartpole {
    fn step(&mut self, action: usize) -> StepReturn {
        if action > 1 {
            panic!("Bad input {} ", action);
        }
        let force;
        if action == 1 {
            force = self.force_mag;
        }
        else {
            force = -self.force_mag;
        }

        let x = self.state[[0, 0]];
        let x_dot = self.state[[0, 1]];
        let theta = self.state[[0, 2]];
        let theta_dot = self.state[[0, 3]];
        let costheta = theta.cos();
        let sintheta = theta.sin();

        let temp = (force + self.polemass_length * theta_dot.powf(2.0) * sintheta) / self.total_mass;
        let thetaacc = (self.gravity * sintheta - costheta * temp) / (self.length * (4.0 / 3.0 - self.masspole * costheta.powf(2.0f32) / self.total_mass));
        let xacc = temp - self.polemass_length * thetaacc * costheta / self.total_mass;
        
        let x = x + self.tau * x_dot;
        let x_dot = x_dot + self.tau * xacc;
        let theta = theta + self.tau * theta_dot;
        let theta_dot = theta_dot + self.tau * thetaacc;

        self.state[[0, 0]] = x;
        self.state[[0, 1]] = x_dot;
        self.state[[0, 2]] = theta;
        self.state[[0, 3]] = theta_dot;
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
        return (self.state.clone(), reward, done)
    }

    fn reset(&mut self) -> Array2<f32> {
        let mut rng = thread_rng();
        let side = Uniform::new(-0.05, 0.05);
        self.steps_beyond_done = -1;
        self.state = Array2::from_shape_vec((1, 4), vec![rng.sample(side), rng.sample(side), rng.sample(side), rng.sample(side)]).unwrap();
        return self.state.clone();
    }

    fn opservation_space(&self) -> Vec<usize> {
        return vec![1, 4];
    }

    fn action_space(&self) -> Vec<usize> {
        return vec![1, 2];
    }

    fn render(&mut self) {
        let world_width = self.x_threshold * 2.0f32;
        let screen_width = 600;
        let scale = screen_width as f32 / world_width as f32;
        let carty = 100;
        let polewidth = 10.0f32;
        let polelen = scale * (2.0f32 * self.length);
        let cartwidth = 50.0;
        let cartheight = 30.0;

        let l = -cartwidth / 2.0;
        let r = cartwidth / 2.0;
        let t = cartheight / 2.0;
        let b = -cartheight / 2.0;

        let cart_polygons = vec![(l as usize, b as usize), 
                                 (l as usize, t as usize), 
                                 (r as usize, t as usize), 
                                 (r as usize, b as usize),
                                 (l as usize, b as usize)];
        let l = -polewidth / 2.;
        let r = polewidth / 2.;
        let t = polelen - polewidth / 2.;
        let b = -polewidth / 2.;

        let pole_polygons = vec![(l as usize, b as usize), 
                                 (l as usize, t as usize), 
                                 (r as usize, t as usize), 
                                 (r as usize, b as usize),
                                 (l as usize, b as usize)];

        let axeloffset = cartheight / 4.0f32;
        let cartx = self.state[[0, 0]] * scale + screen_width as f32 / 2.0;
        let cart_transform = Renderer::create_transform((cartx , carty as f32), 1., 0.0);
        let pole_transform = Renderer::create_transform((cartwidth / 4., axeloffset), 1.0, -self.state[[0, 2]]);
        let pole_in_cart_space = &cart_transform.dot(&pole_transform);

        self.renderer.clear_screen();
        self.renderer.draw_polygon(&cart_polygons, &cart_transform, 0);
        self.renderer.draw_polygon(&pole_polygons, &pole_in_cart_space, 14730072);//I think that that is tan?
        self.renderer.render();
    }
}