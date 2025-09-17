use ns_r::potential::{ChaoticPotential};
use ns_r::solver::{find_phi_end, find_phi_exit};

fn main() {
    println!("--- Inflationary Parameters Calculation ---");

    //　ポテンシャルモデルと計算精度の設定
    // TODO: potentialとprecisionの意味が然前違うのに、こんな位置に書いて違和感
    let potential = ChaoticPotential{ m: 1.0, power: 2.0 };
    let precision = 1e-4;

    // find_phi_end を呼び出し、inflation endでの場の値を返す
    let phi_end = match find_phi_end(&potential, (20.0, 1.0), precision) {
        Ok(phi) => {
            println!("Inflation ends at phi_end = {}", phi);
            phi
        },
        Err(e) => {
            println!("[Error] Could not find phi_end: {}", e);
            return; // phi_endが見つからなければ終了
        }
    };

    // find_phi_exit を呼び出し、e-fold=60となる点を逆算する
    let n_target = 60.0;
    let search_range = (phi_end + 0.1, 30.0); 
    match find_phi_exit(&potential, phi_end, n_target, search_range, precision) {
        Ok(phi_exit) => {
            println!("Field value that the horizon exit at N={} corresponds to phi_exit = {}", n_target, phi_exit);
        },
        Err(e) => {
            println!("[Error] Could not find phi_exit: {}", e);
            return; // phi_exitが見つからなければ終了
        }
    }
}