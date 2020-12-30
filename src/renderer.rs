
use minifb::{Key, Window, WindowOptions};
use ndarray::prelude::*;

const MAX_VERTS : usize = 10000;
const CIRCLE_LOD : usize = 20;
//Scaleline algo apdated from: https://www.geeksforgeeks.org/scan-line-polygon-filling-using-opengl-c/

#[derive(Copy, Clone)]
struct EdgeBucket {
    y_max: usize,
    x_of_y_min: f32,
    slope_inverse: f32,
    color: u32
}

impl EdgeBucket {
    pub fn new() -> EdgeBucket {
        EdgeBucket {
            y_max: 0,
            x_of_y_min: 0.0f32,
            slope_inverse : 0.0f32,
            color: 0
        }
    }

    fn reset(&mut self) {
        self.y_max = 0;
        self.x_of_y_min = 0.0f32;
        self.slope_inverse = 0.0f32;
        self.color = 0;
    }
}

struct EdgeTable {
    counter_edge_bucket: usize,
    buckets: Vec<EdgeBucket>
}

impl EdgeTable {
    fn new() -> EdgeTable {
        EdgeTable {
            counter_edge_bucket: 0,
            buckets: vec![EdgeBucket::new(); MAX_VERTS]
        }
    }

    fn reset(&mut self) {
        for b in &mut self.buckets {
            b.reset();
        }
        self.counter_edge_bucket = 0;
    }

    fn store_edge_tuple(&mut self, ym: usize, xm: usize, slope_inverse: f32, color: u32) {
        self.buckets[self.counter_edge_bucket].y_max = ym;
        self.buckets[self.counter_edge_bucket].x_of_y_min = xm as f32;
        self.buckets[self.counter_edge_bucket].slope_inverse = slope_inverse;
        self.buckets[self.counter_edge_bucket].color = color;
        self.insertion_sort();
        self.counter_edge_bucket += 1;

    }

    fn remove_edge_by_y_max(&mut self, yy: usize) {

        let mut i : isize = 0;
        while i < self.counter_edge_bucket as isize {
            let index = i as usize;
            if self.buckets[index].y_max == yy {
                for j in index..(self.counter_edge_bucket - 1) {
                    self.buckets[j].y_max = self.buckets[j + 1].y_max;
                    self.buckets[j].x_of_y_min = self.buckets[j + 1].x_of_y_min;
                    self.buckets[j].slope_inverse = self.buckets[j + 1].slope_inverse;
                    self.buckets[j].color = self.buckets[j + 1].color;
                }
                self.counter_edge_bucket -= 1;
                i -= 1;
            }
            i += 1;
        }
    }
    
    fn insertion_sort(&mut self) {
        let mut temp = EdgeBucket::new();
        let mut j;
        for i in 1..self.counter_edge_bucket {
            temp.y_max = self.buckets[i].y_max;
            temp.x_of_y_min = self.buckets[i].x_of_y_min;
            temp.slope_inverse = self.buckets[i].slope_inverse;
            temp.color = self.buckets[i].color;
            j = i - 1;
            while temp.x_of_y_min < self.buckets[j].x_of_y_min {
                self.buckets[j + 1].y_max = self.buckets[j].y_max;
                self.buckets[j + 1].x_of_y_min = self.buckets[j].x_of_y_min;
                self.buckets[j + 1].slope_inverse = self.buckets[j].slope_inverse;
                self.buckets[j + 1].color = self.buckets[j].color;
                if j == 0 {
                    break;
                }
                else {
                    j = j - 1;
                }
            }

            self.buckets[j + 1].y_max = temp.y_max;
            self.buckets[j + 1].x_of_y_min = temp.x_of_y_min;
            self.buckets[j + 1].slope_inverse = temp.slope_inverse;
            self.buckets[j + 1].color = temp.color;
        }
    }

    fn update_x_by_slope_in(&mut self) {
        for i in 0..self.counter_edge_bucket {
            self.buckets[i].x_of_y_min = self.buckets[i].x_of_y_min + self.buckets[i].slope_inverse;
        }
    }
}

pub struct Renderer {
    buffer: Vec<u32>,
    window_height: usize,
    window_width: usize,
    //    polygons: Vec<Vec<(usize, usize)>>,
    edge_table: Vec<EdgeTable>,
    active_edge_table: EdgeTable,
    window: Window
}

impl Renderer {

    ///
    pub fn create_transform(translation: (f32, f32), scale: f32, rotation: f32) -> Array2<f32> {
        let translation_matrix = Array2::from_shape_vec((3, 3), vec![1., 0., translation.0,
                                                                     0., 1., translation.1,
                                                                     0., 0., 1.]).unwrap();
        let scale_matrix = Array2::from_shape_vec((3, 3), vec![scale, 0., 0.,
                                                               0., scale, 0.,
                                                               0., 0., 1.]).unwrap();
        let rotation_matrix = Array2::from_shape_vec((3, 3), vec![rotation.cos(), -rotation.sin(), 0.,
                                                                  rotation.sin(), rotation.cos(), 0.,
                                                                  0., 0., 1.]).unwrap();
        return translation_matrix.dot(&scale_matrix.dot(&rotation_matrix));
    }

    pub fn new(window_width: usize, window_height: usize) -> Renderer {

        let mut edge_table = vec![];
        for _ in 0..window_height {
            edge_table.push(EdgeTable::new());
        }
        Renderer {
            buffer: vec![0; window_height * window_width],            
            window_height,
            window_width,
            edge_table,
            active_edge_table: EdgeTable::new(),
            window: Window::new(
                "Test - ESC to exit",
                window_width,
                window_height,
                WindowOptions::default(),
            )
            .unwrap_or_else(|e| {
                panic!("{}", e);
            })
        }
    }

    fn reset(&mut self) {
        for e in &mut self.edge_table {
            e.reset();
        }
        self.active_edge_table.reset();
    }

    fn store_edge_in_table(&mut self, point1: (usize, usize), point2: (usize, usize), color: u32) {
        //Don't store horizontal edges

        let minv;
        let m;
        let scanline;
        let y_max_ts;
        let x_with_t_min_ts;

        if point1.0 == point2.0 {
            minv = 0.0f32;
        }
        else {
            if point1.1 == point2.1 {
                return;
            }    
            m = (point2.1 as f32 - point1.1 as f32) / (point2.0 as f32 - point1.0 as f32 );
            minv = 1.0f32 / m;        
        }

        if point1.1 > point2.1 {
            scanline = point2.1;
            y_max_ts = point1.1;
            x_with_t_min_ts = point2.0;
        }
        else {
            scanline = point1.1;
            y_max_ts = point2.1;
            x_with_t_min_ts = point1.0;
        }
        //println!("{}", scanline);
        self.edge_table[scanline].store_edge_tuple(y_max_ts, x_with_t_min_ts, minv, color);
    }

    //This is a really quicky and dirty clipping algo I am writing just to avoid some
    //out of bounds issues I am seeing
    fn clipping(&self, edge: (usize, usize)) -> (usize, usize) {
        let mut x = self.window_width;
        if edge.0 < self.window_width {
            x = edge.0;
        }
        let mut y = self.window_height;
        if edge.1 < self.window_height {
            y = edge.1;
        }        
        return (x, y);
    }
    //Drawing just adds the polgon to the edge table, committing a frame
    //is the final act that will call render and 
    pub fn draw_polygon(&mut self, points: &Vec<(usize, usize)>, transform: &Array2<f32>, color: u32) {
        let mut count = 0;
        let mut x1 = 0;
        let mut y1 = 0;
        let mut x2 = 0;
        let mut y2 = 0;

        for p in points {
            count += 1;
            if count > 2 {
                x1 = x2;
                y1 = y2;
                count = 2;                
            }
            if count == 1 {
                x1 = p.0;
                y1 = p.1;
            }
            else {
                x2 = p.0;
                y2 = p.1;
                let pretransform_position_1 = Array2::from_shape_vec((3, 1), vec![x1 as f32, y1 as f32, 1.0]).unwrap();
                let pretransform_position_2 = Array2::from_shape_vec((3, 1), vec![x2 as f32, y2 as f32, 1.0]).unwrap();
                let posttransofmr_position_1 = transform.dot(&pretransform_position_1);
                let posttransofmr_position_2 = transform.dot(&pretransform_position_2);

                //before we add it to the edge table lets ensure that it wont cause any problems in the edge table
                let clipped_start = self.clipping((posttransofmr_position_1[[0, 0]] as usize, posttransofmr_position_1[[1, 0]] as usize));
                let clipped_end = self.clipping((posttransofmr_position_2[[0, 0]] as usize, posttransofmr_position_2[[1, 0]] as usize));

                self.store_edge_in_table(
                    clipped_start,
                    clipped_end,
                    color,
                    );
            }
        }

        self.render_polygon();
    }

    pub fn draw_line(&mut self, y: usize, start_x: usize, end_x: usize, color: u32) {
        if start_x > end_x {
            return;
        }
        if y > self.window_height {
            return
        }

        for x in start_x..end_x {
            let index = x  + (y * self.window_width);
            if index < self.buffer.len() {
                self.buffer[index] = color;
            }
        }
    }

    pub fn color(r: u32, g: u32, b: u32) -> u32 {
        return r & g & b;
    }

    pub fn draw_circle(&mut self, radius: usize) {
        let radius = radius as f32;
        let mut circle = vec![];

        //I should find a way to precaluclate these points
        for i in 0..CIRCLE_LOD {
            let theta = (2.0 * std::f32::consts::PI) * (i as f32 / CIRCLE_LOD as f32);
            let point = (((theta.cos() * radius) + radius) as usize, ((theta.sin() * radius) + radius) as usize);
            circle.push(point);
        }    
        circle.push((circle[0].0, circle[0].1));
        //self.draw_polygon(&circle);
    }

    pub fn clear_screen(&mut self) {
        //Clear the screen
        for x in 0..self.window_width {
            for y in 0..self.window_height {
                let index = x  + (y * self.window_width);
                self.buffer[index] = std::u32::MAX - 1;//clear it white
            }
        }
    }

    pub fn render_polygon(&mut self) {
        for i in 0..self.window_height {
            for j in 0..self.edge_table[i].counter_edge_bucket {
                let used_bucket = &self.edge_table[i].buckets[j];
                self.active_edge_table.store_edge_tuple(used_bucket.y_max, used_bucket.x_of_y_min as usize, used_bucket.slope_inverse, used_bucket.color);
            }
            self.active_edge_table.remove_edge_by_y_max(i);
            self.active_edge_table.insertion_sort();
            let mut fill_flag;
            let mut j = 0;
            let mut coord_count = 0;
            let mut x1 = 0;
            let mut x2 = 0;
            let mut y_max_1 = 0;
            let mut y_max_2 = 0;

            while j < self.active_edge_table.counter_edge_bucket {

                if coord_count % 2 == 0 {
                    x1 = self.active_edge_table.buckets[j].x_of_y_min as usize;
                    y_max_1 = self.active_edge_table.buckets[j].y_max as usize;
                    if x1 == x2 {
                        if ( x1 == y_max_1 && x2 != y_max_2) || ( x1 != y_max_1 && x2 == y_max_2) {
                            x2 = x1;
                            y_max_2 = y_max_1;
                        }
                        else {
                            coord_count += 1;
                        }
                    }
                    else {
                        coord_count += 1;
                    }
                }
                else {
                    x2 = self.active_edge_table.buckets[j].x_of_y_min as usize; 
                    y_max_2 = self.active_edge_table.buckets[j].y_max; 
                
                    fill_flag = false;                    
                    if x1 == x2
                    {                   
                        if (x1 == y_max_1 && x2 != y_max_2) || (x1 != y_max_1 && x2 == y_max_2)
                        { 
                            x1 = x2; 
                            y_max_1 = y_max_2; 
                        } 
                        else
                        { 
                            coord_count += 1;
                            fill_flag = true; 
                        } 
                    } 
                    else
                    { 
                            coord_count += 1; 
                            fill_flag = true; 
                    }
                    if fill_flag {
                        /*
                        if x2 == self.window_width - 1 {
                            panic!("ACASDASD");
                        }
                        if x2 < x1 {
                            panic!("AABBCCDD");
                        }
                        */
                        if x2 < x1 {
                            for x in x2..x1 {
                                let index = x  + (i * self.window_width);
                                if index < self.buffer.len() {
                                    self.buffer[index] = self.active_edge_table.buckets[j].color;
                                }
                            }
                        }
                        else {
                            //Horrible hack to cover for a bug else where in the code... no idea what is going
                            //On, I think it has to do with slopes,but also, no idea
                            for x in x1..x2 {
                                let index = x  + (i * self.window_width);
                                if index < self.buffer.len() {
                                    self.buffer[index] = self.active_edge_table.buckets[j].color;
                                }
                            }
                        }
                    }        
                }
                j += 1;
            }
            self.active_edge_table.update_x_by_slope_in();
        }
        self.reset();
    }

    pub fn render(&mut self) {        
        let _ = self.window.update_with_buffer(&self.buffer.iter().rev().map(|x|*x).to_owned().collect::<Vec<_>>(), self.window_width, self.window_height).unwrap();
        //self.reset();
    }
}