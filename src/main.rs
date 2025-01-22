use factorio_helpers::calculators::*;
use factorio_helpers::consts::*;
use factorio_helpers::terminal_funcs::*;

fn main() {
    clear_terminal();

    let number_of_labs = get_input(MSG_LABS, NUMBER_OF_LABS);
    let research_speed_bonus = get_input(MSG_LAB_SPEED, RESEARCH_LEVEL);
    let module_speed_bonus = get_input(MSG_MODULE_SPEED, MODULE_SPEED_BONUS);
    let research_cycle_time = get_input(MSG_CYCLE_TIME, RESEARCH_CYCLE_TIME);

    let (effective_research_time, adjusted_cycle_time, packs_per_second) = calculate_research_stats(
        number_of_labs,
        research_speed_bonus,
        module_speed_bonus,
        research_cycle_time,
    );

    clear_terminal();
    println!("Effective Lab Research Speed: {}", effective_research_time);
    println!("Adjusted Cycle Time: {} seconds", adjusted_cycle_time);
    println!("Packs Per Second: {}", packs_per_second);
}

fn get_input(prompt: &str, default: f64) -> f64 {
    use std::io::{self, Write};
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(default)
}
