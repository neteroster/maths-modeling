use std::{fs::File, io::Write};
use rand::Rng;

use rand::{self, random};
fn sim_ann
    (
    target_f: fn(arg: f64) -> f64,
    init_sol: f64,
    sol_min: f64,
    sol_max: f64,
    init_arg: f64,
    arg_min: f64,
    brk_count: usize,
    new_sol_genf: fn(curr_sol: f64, init_arg: f64, arg: f64, r: f64) -> f64,
    decline_t: f64
    ) -> f64 {
    
    let mut f = File::options().write(true).create(true).open("C:/Users/neter/Desktop/r.txt").unwrap(); // DEBUG
    let mut fdb = File::options().write(true).create(true).open("C:/Users/neter/Desktop/rdb.txt").unwrap(); // DEBUG

    let mut solution = init_sol;
    let mut arg = init_arg;
    let mut best_sol = sol_max;
    while arg >= arg_min {
        for _ in 0..brk_count {
            f.write_fmt(format_args!("------INNBEG------\n")).unwrap(); // DEBUG: tag

            let new_sol = new_sol_genf(solution, init_arg, arg, 5.0);
            let dE = target_f(new_sol) - target_f(solution);

            f.write_fmt(format_args!("sol: {}; new_sol: {}; deltaE: {}\n", solution, new_sol, dE)).unwrap(); // DEBUG: dump var
            if new_sol < sol_min || new_sol > sol_max { continue; }
            if dE <= 0.0 {
                solution = new_sol;
                f.write_fmt(format_args!("dE <= 0, renew solution.\n")).unwrap(); // DEBUG: dump program struc.
            } else {
                f.write_fmt(format_args!("dE > 0.\n")).unwrap(); // DEBUG: dump program struc.
                let poss = random::<f64>();
                f.write_fmt(format_args!("poss: {}\n", poss)).unwrap(); // DEBUG: dump var.
                f.write_fmt(format_args!("arg: {}; -dE / arg: {}; exp: {}\n", arg, (-dE / arg), (-dE / arg).exp())).unwrap(); // DEBUG: dump var.
                if (-dE / arg).exp() <= poss {
                    f.write_fmt(format_args!("(-dE / arg).exp() <= poss\n")).unwrap(); // DEBUG: dump strc.
                    solution = new_sol;
                }
            }

            if target_f(solution) < best_sol { best_sol = solution; }
            f.write_fmt(format_args!("------INNEND------\n")).unwrap(); // DEBUG: tag
            //fdb.write_fmt(format_args!("{} ", solution)).unwrap(); // DEBUG: dump solution
        }
        fdb.write_fmt(format_args!("{} ", solution)).unwrap(); // DEBUG: dump solution
        arg *= decline_t;
        f.write_fmt(format_args!("----------------OTTEND----------------\n")).unwrap();
    }

    best_sol
}

fn tf(x: f64) -> f64 {
    0.01 * (x * x * x * x) - 0.7 * (x * x) + x
}

fn new_sol(curr_sol: f64, init_arg: f64, arg: f64, r: f64) -> f64 {
    let lr_range = r * (arg / init_arg); // this is important
    rand::thread_rng().gen_range(curr_sol - lr_range..= curr_sol + lr_range)
}

fn main() {
    println!("{} ", sim_ann(tf, 5.0, -10.0, 10.0, 10.0, 0.01, 100, new_sol, 0.92));
}
