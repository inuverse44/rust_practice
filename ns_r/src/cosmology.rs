/*
 * use crateはモジュールAからモジュールBを参照するときに使われる
 * プロジェクトルートからの絶対パスで別モジュールを指す
 * パスの記述が壊れにくい
 * ベストプラクティス
 */
use crate::potential::Potential;

const M_P: f64 = 1.0; // TODO: 物理定数は別ファイルで管理する

pub fn hubble_parameter(potential: &impl Potential, phi: f64, dot_phi: f64) -> f64 {
        ((0.5 * dot_phi.powi(2) + potential.value(phi)) / (3.0 * M_P)).sqrt()
}

pub fn epsilon(potential: &impl Potential, phi: f64) -> f64 {
    0.5 * M_P.powi(2) * (potential.prime(phi) / potential.value(phi)).powi(2)
}

pub fn eta(potential: &impl Potential, phi: f64) -> f64 {
    M_P.powi(2) * potential.double_prime(phi) / potential.value(phi)
}

pub fn spectral_index(epsilon: f64, eta: f64) -> f64 {
    1.0 - 6.0 * epsilon + 2.0 * eta
}

pub fn tensor_to_scalar_ratio(epsilon: f64) -> f64 {
    16.0 * epsilon
}