use f32;
use ndarray::prelude::*;
use rand::prelude::*;
use rand::distributions::Uniform;
use crate::enviroment::*;
use crate::renderer::*;


const LINK_LENGTH_1 : f32 = 1.;
const LINK_LENGTH_2 : f32 = 1.;
const LINK_MASS_1 : f32 = 1.;
const LINK_MASS_2 : f32 = 1.;
const LINK_COM_POS_1 : f32 = 0.5;
const LINK_COM_POS_2 : f32 = 0.5;
const LINK_MOI : f32 = 1.;
const MAX_VEL_1 : f32 = 4. * std::f32::consts::PI;
const MAX_VEL_2 : f32 = 9. * std::f32::consts::PI;

const AVAIL_TORQUE : [f32; 3] = [-1., 0., 1.];

pub struct Acrobot {
    state: Vec<f32>,
    renderer: Renderer
}

impl Acrobot {

    pub fn new() -> Acrobot {
        Acrobot {
            state: vec![],
            renderer: Renderer::new(500, 500)
        }
    }

    fn wrap(x: f32, min: f32, max: f32) -> f32 {
        let diff = max - min;

        let mut local_x = x;
        while local_x > max {
            local_x -= diff;
        }
        while local_x < min {
            local_x += diff;
        }
        return local_x;    
    }

    fn rk4(dsdt: fn(Vec<f32>) -> Vec<f32>, s: Vec<f32>, t: Vec<f32>) -> Vec<f32> {
        let mut yout : Vec<Vec<f32>> = vec![];
        yout.push(s);
        for i in 0..(t.len() - 1) {
            let thist = t[i];
            let dt = t[i + 1] - thist;
            let dt2 = dt / 2.0f32;

            let use_yo = yout[i].clone();

            let k1 = dsdt(use_yo.clone());
            let k1_diff = k1.iter().zip(use_yo.clone()).map(|(a, b)| b + a * dt2).collect();

            let k2 = dsdt(k1_diff);
            let k2_diff = k2.iter().zip(use_yo.clone()).map(|(a, b)| b + a * dt2).collect();
            let k3 = dsdt(k2_diff);
            let k3_diff = k3.iter().zip(use_yo.clone()).map(|(a, b)| b + a * dt).collect();
            let k4 = dsdt(k3_diff);
            let k2_mapped = k2.iter().map(|a| a * 2.0f32);
            let k3_mapped = k3.iter().map(|a| a * 3.0f32);
            
            let ks_summed = k1.iter().zip(k2_mapped).map(|(a, b)| a + b).zip(k3_mapped).map(|(a, b)| a + b).zip(k4).map(|(a, b)| a + b).map(|a| a * 6.0);            
            let result = use_yo.iter().map(|a| a + dt).zip(ks_summed).map(|(a, b)| a / b);
            yout.push(result.collect());
        }

        return yout[yout.len() - 1].clone();
    }
    
    //In the original python there is a unsued "t" paramter, what, why, stop 
    fn _dsdt(s_augmented: Vec<f32>) -> Vec<f32> {
        let m1 = LINK_MASS_1;
        let m2 = LINK_MASS_2;
        let l1 = LINK_LENGTH_1;
        let lc1 = LINK_COM_POS_1;
        let lc2 = LINK_COM_POS_2;
        let I1 = LINK_MOI;
        let I2 = LINK_MOI;

        let g = 9.8;
        let a = s_augmented[s_augmented.len() - 1]; // In the OG python this is a = s_augmented[-1]

        //s_augmented.remove(s_augmented.len() - 1);// In the OG python this is a = s_augmented[:-1]
        let theta1 = s_augmented[0];
        let theta2 = s_augmented[1];
        let dtheta1 = s_augmented[2];
        let dtheta2 = s_augmented[3];

        let d1 = m1 * lc1.powf(2.0) + m2 * (l1.powf(2.0) + lc2.powf(2.0) + 2. * l1 * lc2 * theta2.cos()) + I1 + I2;
        let d2 = m2 * (lc2.powf(2.0)  + l1 * lc2 * theta2.cos()) + I2;
        let phi2 = m2 * lc2 * g * (theta1 + theta2 - std::f32::consts::PI / 2.0f32);
        let phi1_1 = -m2 * l1 * lc2 * dtheta2.powf(2.0f32) * theta2.sin();
        let phi1_2 = -2.0f32 * m2 * l1 * dtheta2 * dtheta1 * theta2.sin();
        let phi1_3 = (m1 * lc1 + m2 * l1) * g * (theta1 - std::f32::consts::PI / 2.0f32) + phi2;
        let phi1 = phi1_1 + phi1_2 + phi1_3;

        let ddtheta2 = (a + d2 / d1 * phi1 - m2 * l1 * lc2 * dtheta1.powf(2.0f32) * theta2.sin() - phi2) / (m2 * lc2.powf(2.0f32) + I2 - d2.powf(2.0f32) / d1);
        let ddtheta1 = -(d2 * ddtheta2 + phi1) / d1;

        return vec![dtheta1, dtheta2, ddtheta1, ddtheta2, 0.0f32];
    }

    fn bound(x: f32, min_val: f32, max_val: f32) -> f32 {
        return min_val.min(max_val.max(x));
    }

    fn is_terminal(&self) -> bool {
        return (-self.state[0].cos() - (self.state[1] + self.state[0]).cos()) > 1.0f32;
    }

    fn get_obs(&self) -> Array2<f32> {
        return Array2::<f32>::from_shape_vec((1, 6), vec![-self.state[0].cos(), 
                                                           self.state[0].sin(),
                                                           self.state[1].sin(), 
                                                           self.state[1].cos(), 
                                                           self.state[2], 
                                                           self.state[3]]).unwrap();
    }
}

impl Enviroment for Acrobot {

    fn reset(&mut self) -> Array2<f32> {
        let mut rng = thread_rng();
        let side = Uniform::new(-0.1, 0.1);
        let new_state = vec![rng.sample(side), rng.sample(side), rng.sample(side), rng.sample(side)];
        self.state = new_state.clone();
        return self.get_obs();
    }

    fn step(&mut self, action: usize) -> StepReturn {
        let mut s = self.state.clone();

        s.push(AVAIL_TORQUE[action]);
        let mut result = Acrobot::rk4(Acrobot::_dsdt, s,vec![0.0, 0.2]);

        result.remove(result.len() - 1);
        result[0] = Acrobot::wrap(result[0], -std::f32::consts::PI, std::f32::consts::PI);
        result[1] = Acrobot::wrap(result[1], -std::f32::consts::PI, std::f32::consts::PI);
        result[2] = Acrobot::bound(result[2], -MAX_VEL_1, MAX_VEL_1);
        result[3] = Acrobot::bound(result[3], -MAX_VEL_2, MAX_VEL_2);
        self.state = result.clone();
        let terminal = self.is_terminal();
        let mut reward = -1.0;
        if terminal {
            reward = 0.0;
        }
        
        return (self.get_obs(), reward, terminal);
    }
    
    fn opservation_space(&self) -> Vec<usize> {
        return vec![1, 6]
    }

    fn action_space(&self) -> Vec<usize> {
        return vec![1, 2]
    }

    fn render(&mut self) {
        //This might not be needed :|
        let bound = LINK_LENGTH_1 + LINK_LENGTH_2 + 0.2;
        let p1 = vec![-LINK_LENGTH_1 * self.state[0].cos(),LINK_LENGTH_1 * self.state[0].sin()];

        let p2 = vec![p1[0] - LINK_LENGTH_1 * (self.state[0] + self.state[1]).cos(),
                      p1[1] + LINK_LENGTH_2 * (self.state[0] + self.state[1].sin())];
        let xys = vec![p2, p1, vec![0., 0.]];
        let thetas = vec![self.state[0] - std::f32::consts::PI / 2.0, self.state[0] + self.state[1] - std::f32::consts::PI / 2.];
        let link_lengths = vec![LINK_LENGTH_1, LINK_LENGTH_2];
        //This has a decial x value in the original python, I have done my thing

       // self.renderer.draw_line(1, 0, 5, 0);

        let l = 0;
        let t = 100;
        let b = 0;
        self.renderer.clear_screen();
        let scale_x = 
        let world_transform = Renderer::create_transform((0, 1000 * ), scale: f32, rotation: f32)

        for i in 0..1 {
            let r = link_lengths[i];
            let transform = Renderer::create_transform((xys[i][0] as f32, xys[i][1] as f32), 1., 0.);
            let link_polygons = vec![(l, b), (l, t), (r as usize, t), (r as usize, b)];
            self.renderer.draw_polygon(&link_polygons, &transform, 0);
        }z`

        self.renderer.render();
    }

}