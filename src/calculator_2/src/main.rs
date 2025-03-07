mod functions;
use crate::functions::*;

pub fn sigma(dl: [f64; 3], r: [f64; 3], ri: [f64; 3], sigma: &mut f64) {
    let mut top: [f64; 3];
    subtract_vector(&r, &ri, &mut top);
    let mut bot_vec: [f64; 3];
    let mut bot: f64 = 0.0;
    subtract_vector(&r, &ri, &mut bot_vec);
    abs_vector(&bot_vec, &mut bot);
    bot = bot.powi(3);
    divide_vector(&dl, bot, sigma)
}

fn main() {
    let mut sigma_val: f64 = 0.0;
    sigma([2.0, 2.5, 3.0], [2.0, 2.5, 3.0], [2.0, 2.5, 3.0], &mut sigma_val);
    println!("{}", sigma_val)
}