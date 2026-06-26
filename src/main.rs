mod cli_display;
mod core;
mod inputs;

fn main() {
    let mut state = inputs::start();
    let speed = state.1;

    cli_display::display_frame(state.0.clone(), 0);

    // display other states
    for generation in 1.. {
        let points = core::algorithm(state.0.clone(), state.1);
        state = (points.clone(), speed);
        cli_display::display_frame(points, generation);
        if state.0.is_empty() {
            println!();
            println!("All cells died, simulation ended.");
            println!();
            break;
        }
    }
}
