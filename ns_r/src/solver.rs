use crate::potential::Potential;
use crate::cosmology::{epsilon};


// ε(φ) = 1 となる φ_endを見つける
pub fn find_phi_end(potential: &impl Potential, search_range: (f64, f64), precision: f64) -> Result<f64, &'static str> {

    // 根が存在するかチェック
    let f = epsilon(potential, search_range.0) - 1.0;
    let g = epsilon(potential, search_range.1) - 1.0;
    if f * g >= 0.0 {
        return Err("Root could not be specified. Change phi1 and/or phi2.");
    }

    let mut phi_a = search_range.0;
    let mut phi_b = search_range.1;
    let mut phi_c = (phi_a + phi_b) / 2.0;          // 中心
    let mut fc = epsilon(potential, phi_c) - 1.0;    // その時の関数の値f(c) = ε(φ_c) - 1
    while (phi_b - phi_a).abs() > precision {
        if fc.abs() < precision {
            return Ok(phi_c);
        }
        if f * fc < 0.0 {
            phi_b = phi_c;
        } else {
            phi_a = phi_c;
        }
        phi_c = (phi_a + phi_b) / 2.0;              // 場の値の更新
        fc = epsilon(potential, phi_c) - 1.0;       // 関数の値の更新
    }
    Ok(phi_c)
}

// e-foldの積分計算を行う
/// e-foldの積分計算を行い、対応するφの値を返す
pub fn find_phi_exit(
    potential: &impl Potential, 
    phi_end: f64, 
    n_target: f64, // e-foldの目標値
    search_range: (f64, f64),
    precision: f64
) -> Result<f64, &'static str> {

    //　クロージャが使える
    let integrand = |phi: f64| potential.value(phi) / potential.prime(phi);
    let efold_num = |phi: f64| simpson(|p| integrand(p), phi_end, phi, precision);
    let find_root = |phi: f64| efold_num(phi) - n_target;

    let mut phi_a = search_range.0;
    let mut phi_b = search_range.1;

    // 根が存在するかチェック
    let fa = find_root(phi_a);
    let fb = find_root(phi_b);
    if fa * fb >= 0.0 {
        return Err("Root for phi_exit not in search range. Change the range.");
    }

    let mut phi_c = (phi_a + phi_b) / 2.0;
    let mut fc = find_root(phi_c);

    while (phi_b - phi_a).abs() > precision {
        if fc.abs() < precision {
            return Ok(phi_c);
        }
        if fa * fc < 0.0 {
            phi_b = phi_c;
        } else {
            phi_a = phi_c;
        }
        phi_c = (phi_a + phi_b) / 2.0;
        fc = find_root(phi_c);
    }
    Ok(phi_c)
}

pub fn simpson<F>(f: F, a: f64, b: f64, precision: f64) -> f64 
    where F: Fn(f64) -> f64 {

    let mut n = 2; // 初期分割数
    let mut last_result = simpson_internal(&f, a, b, n);

    for _ in 0..20 { // 念のため最大20回程度の反復に制限
        n *= 2; // 分割数を2倍にする
        let current_result = simpson_internal(&f, a, b, n);

        // 前回の結果との相対誤差を計算
        let error = (current_result - last_result).abs() / last_result.abs();

        if error < precision {
            return current_result; // 精度を満たしたらループを抜ける
        }

        last_result = current_result;
    }

    last_result // 収束しなかった場合は最後の結果を返す
}

/// シンプソン法の内部計算（固定分割数）
fn simpson_internal<F>(f: &F, a: f64, b: f64, n: usize) -> f64 
    where F: Fn(f64) -> f64 {
    let n = if n % 2 == 1 { n + 1} else { n };  // nを偶数
    let h = (b - a) / n as f64;                 // 微小幅

    let mut sum1 = 0.0;
    for i in (1..n).step_by(2) {
        sum1 += f(a + i as f64 * h);
    }

    let mut sum2 = 0.0;
    for i in (2..n).step_by(2) {
        sum2 += f(a + i as f64 * h);
    }

    h / 3.0 * (f(a) + f(b) + 4.0 * sum1 + 2.0 * sum2)
}





