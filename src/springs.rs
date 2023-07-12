#[derive(Debug)]

pub struct Spring {
    pub x: f32,
    pub v: f32,
    pub dampen: f32,
    pub k: f32,
    pub m: f32,
    pub len: f32
}

impl Spring {
    pub fn new(x0: f32, v0: f32, dampen: f32, k: f32, m: f32, len: f32) -> Self {
        Spring {
            x: x0,
            v: v0, 
            k,
            m,
            dampen,
            len,
        }
    }

    pub fn spring_force(&self, x: f32, v: f32) -> f32{
        (-self.k / self.m)* (x - self.len) - ((self.dampen / self.m) * v)
    }

    pub fn step_v(&mut self, dt: f32){
        // v' = spring_force(self.x, self.v)
        let k1 = self.spring_force(self.x ,self.v) * dt;
        let k2 = self.spring_force(self.x + k1/2.0, self.v*k1/2.0) * dt;
        let k3 = self.spring_force(self.x + k2/2.0, self.v + k2/2.0) * dt;
        let k4 = self.spring_force(self.x + k3 ,self.v + k3) * dt;
        self.v += (k1 + 2.0*k2 + 2.0*k3 + k4) / 6.0
    } 
    pub fn step_x(&mut self, dt: f32){
        // x' = velocity
        let k1 = self.v * dt;
        let k2 = (self.v + k1/2.0) * dt;
        let k3 = (self.v + k2/2.0) * dt;
        let k4 = (self.v + k1) * dt;
        self.x += (k1 + 2.0*k2 + 2.0*k3 + k4) / 6.0
    } 
}