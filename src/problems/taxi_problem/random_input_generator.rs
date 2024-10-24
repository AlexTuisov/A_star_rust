use clap::Parser;
use rand::Rng;
use serde_json::json;
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

#[derive(Parser)]
struct Args {
    #[arg(short, long, help = "Width of the map")]
    width: i32,

    #[arg(long, help = "Height of the map")]
    height: i32,

    #[arg(short, long, help = "Number of passengers")]
    passengers: usize,

    #[arg(short, long, help = "Number of impassable tiles")]
    impassable: usize,
}

fn main() {
    let args = Args::parse();

    match generate_input(args) {
        Ok(json_data) => {
            if let Err(e) = write_to_file(json_data) {
                eprintln!("Error writing file: {}", e);
            } else {
                println!("Input JSON generated successfully: generated_input.json");
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn generate_input(args: Args) -> Result<String, String> {
    if args.width * args.height <= (args.passengers * 2 + 1 + args.impassable) as i32 {
        return Err("Map size too small for the number of elements.".to_string());
    }

    let mut rng = rand::thread_rng();
    let mut positions: HashSet<(i32, i32)> = HashSet::new();

    let mut impassable_tiles = vec![];
    while impassable_tiles.len() < args.impassable {
        let pos = (rng.gen_range(0..args.width), rng.gen_range(0..args.height));
        if positions.insert(pos) {
            impassable_tiles.push(pos);
        }
    }

    let mut taxi_position;
    loop {
        taxi_position = (rng.gen_range(0..args.width), rng.gen_range(0..args.height));
        if !positions.contains(&taxi_position) {
            positions.insert(taxi_position);
            break;
        }
    }

    let mut passengers = vec![];
    for i in 1..=args.passengers {
        let mut passenger_position;
        let mut goal_position;
        loop {
            passenger_position = (rng.gen_range(0..args.width), rng.gen_range(0..args.height));
            if !positions.contains(&passenger_position) {
                positions.insert(passenger_position);
                break;
            }
        }
        loop {
            goal_position = (rng.gen_range(0..args.width), rng.gen_range(0..args.height));
            if !positions.contains(&goal_position) && goal_position != passenger_position {
                positions.insert(goal_position);
                break;
            }
        }
        passengers.push((format!("passenger{}", i), passenger_position));
        passengers.push((format!("goal{}", i), goal_position));
    }

    // Correct the JSON structure to wrap positions in serde_json::Value::Array
    let json_data = json!({
        "bounds": [args.width, args.height],
        "entities": {
            "taxi": [taxi_position.0, taxi_position.1],
            "passengers": passengers.iter().map(|(name, pos)| (name.clone(), serde_json::Value::Array(vec![pos.0.into(), pos.1.into()]))).collect::<serde_json::Map<_, _>>(),
            "impassable_tiles": impassable_tiles.iter().map(|pos| serde_json::Value::Array(vec![pos.0.into(), pos.1.into()])).collect::<Vec<_>>(),
        }
    });

    Ok(json_data.to_string())
}

fn write_to_file(json_data: String) -> std::io::Result<()> {
    let mut file = File::create("inputs/taxi_problem/generated_input.json")?;
    write!(file, "{}", json_data)?;
    Ok(())
}
