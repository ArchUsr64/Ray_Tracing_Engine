
pub fn reset_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn draw_grayscale(val: f32) {
    print_ansi_string(val, " ".to_string());
}

pub fn print_ansi_string(val: f32, text: String) {
    let mut temp_val = val;
    temp_val = 232.0 + temp_val * 23.0;
    print!("\x1b[48;5;{}m{}", temp_val as u16, text);
}

use super::traits::ToF64;
pub fn print_progress<T: ToF64>(task: &str, progress_fraction: T) {
    println!("{} : {:.1}%", task, (progress_fraction.to_f64() * 100f64));
}
