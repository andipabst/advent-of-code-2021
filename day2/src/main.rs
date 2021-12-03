fn main() {
    let commands: Vec<(&str, i32)> = include_str!("../input.txt")
        .lines()
        .map(|i| i.split_whitespace().collect::<Vec<_>>())
        .map(|cmd| { (cmd[0], cmd[1].parse().unwrap_or(0)) })
        .collect();

    let mut simple_coordinates = (0, 0);
    let mut aim = 0;
    let mut aimed_coordinates = (0, 0);

    for command in commands {
        match command {
            ("forward", amount) => {
                simple_coordinates = (simple_coordinates.0 + amount, simple_coordinates.1);
                aimed_coordinates = (aimed_coordinates.0 + amount, aimed_coordinates.1 + (aim * amount))
            }
            ("up", amount) => {
                // depth -> subtract on going up
                simple_coordinates = (simple_coordinates.0, simple_coordinates.1 - amount);
                aim -= amount;
            }
            ("down", amount) => {
                // depth -> add on going down
                simple_coordinates = (simple_coordinates.0, simple_coordinates.1 + amount);
                aim += amount;
            }
            (_, _) => {}
        }
    }

    println!("part one: {:?} -> {}", simple_coordinates, simple_coordinates.0 * simple_coordinates.1);
    println!("part two: {:?} -> {}", aimed_coordinates, aimed_coordinates.0 * aimed_coordinates.1);
}
