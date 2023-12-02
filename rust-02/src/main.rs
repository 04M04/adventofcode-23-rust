// Rust
const INPUT: &str = include_str!("../input_data.txt");

fn main(){
    let mut game_id = 0;
    let mut result = 0;

    let red_count = 12;
    let green_count = 13;
    let blue_count = 14;

    for line in INPUT.lines() {
        game_id += 1;
        let mut game_possible = true;
        let (_, line) = line.split_once(": ").unwrap();
        let draw_parts = line.split("; ");
        for draw_part in draw_parts {
            let color_parts = draw_part.split(", ");
            for color_part in color_parts {
                // println!("{}", color_part);
                let (color_count, color) = color_part.split_once(' ').unwrap();

                match color {
                    "red" => {if color_count.parse::<i32>().unwrap() > red_count{game_possible = false;}}
                    "green" => {if color_count.parse::<i32>().unwrap() > green_count{game_possible = false;}}
                    "blue" => {if color_count.parse::<i32>().unwrap() > blue_count{game_possible = false;}}
                    _ => unreachable!("unknown: {color}")
                }
            }
        }
        if game_possible {result += game_id}
    }
    dbg!(result);
}
