// n階微分をどう表現するか課題
pub trait Potential {
    fn value(&self, phi: f64) -> f64;
    fn prime(&self, phi: f64) -> f64;
    fn double_prime(&self, phi: f64) -> f64;
}

pub struct ChaoticPotential {
    pub m: f64,
    pub power: f64,
}

impl Potential for ChaoticPotential {
    fn value(&self, phi: f64) -> f64 {
        0.5 * self.m.powi(2) * phi.powi(2)
    }

    fn prime(&self, phi: f64) -> f64 {
        self.m * phi
    }

    fn double_prime(&self, _phi: f64) -> f64 {
        self.m
    }
}
