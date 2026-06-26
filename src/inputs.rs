use std::collections::HashSet;
use std::io::{self, Write, stdin};

pub fn start() -> (HashSet<(i32, i32)>, u64) {
    println!(
        "🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀"
    );
    println!(
        "🦀                                                                                          🦀"
    );
    println!(
        "🦀                               Welcome to Ferris's Game of Life!                          🦀"
    );
    println!(
        "🦀                                                                                          🦀"
    );
    println!(
        "🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀"
    );
    println!();
    println!("Input cells by using X and Y coordinates in the Cartesian coordinate system.");
    println!("The point (0,0) will be the center of the display.");
    println!("The dispay grid is limited so any coordinate too big will not be rendered.");
    println!();

    let mut points = input_coordinates();

    loop {
        println!("Would you like to initialize another cell? Y/n");
        println!("(3+ cells is recomended)");
        let mut response = String::new();
        stdin()
            .read_line(&mut response)
            .expect("Failed to read input.");
        if response.trim().to_lowercase() == "y" {
            let more = input_coordinates();
            points.extend(more);
        } else if response.trim().to_lowercase() == "n" {
            break;
        } else {
            println!();
            println!("Please enter either 'y' or 'n'.")
        }
    }

    let execution_speed: u64;
    // speed
    println!();
    loop {
        println!(
            "Please enter the speed of the simulation (time between each generation) in milliseconds (ms)."
        );
        println!("Put 0 for the maximum speed (limited by cpu speed).");
        let mut speed = String::new();
        stdin()
            .read_line(&mut speed)
            .expect("Failed to read input.");
        let speed_checked = match speed.trim().parse::<u64>() {
            Ok(num) => num,
            Err(_) => {
                println!();
                println!("Type in a valid positive integer.");
                continue;
            }
        };

        execution_speed = speed_checked;
        break;
    }

    (points, execution_speed)
}

fn input_coordinates() -> HashSet<(i32, i32)> {
    let mut points: HashSet<(i32, i32)> = HashSet::new();

    let n: i32;
    loop {
        println!(
            "How many points would you like to add? You will have the opportunity to add more later: "
        );
        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        println!();
        n = match input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter an integer value (3, 87, 12, etc...)");
                continue;
            }
        };
        break;
    }

    let mut counter = 1; // counter used to ensure the point count is accurate even if someone fails to input an integer.

    loop {
        // x-coordinate
        print!(
            "Type in the x-value of point {} (must be an integer): ",
            counter
        );
        let _ = io::stdout().flush();
        let mut x = String::new();
        stdin().read_line(&mut x).expect("Failed to read input.");
        let x_value: i32 = match x.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                println!("Type in a valid integer. Please try again.");
                continue;
            }
        };

        // y-coordinate
        print!(
            "Type in the y-value of point {} (must be an integer): ",
            counter
        );
        let _ = io::stdout().flush();
        let mut y = String::new();
        stdin().read_line(&mut y).expect("Failed to read input.");
        let y_value: i32 = match y.trim().parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                println!("Type in a valid integer. Please try again.");
                continue;
            }
        };

        points.insert((x_value, y_value));
        if counter == n {
            break;
        }
        counter += 1;
        println!();
    }
    println!();
    points
}
