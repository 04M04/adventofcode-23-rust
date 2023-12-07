// Rust
const INPUT: &str = include_str!("../input_data.txt");

fn main(){
    let mut result = 0;

    for line in INPUT.lines() {
        // let (_, line) = line.split_once(": ").unwrap();
        // let draw_parts = line.split("; ");

        
        // let mut max_red_count = 0;
        // let mut max_green_count = 0;
        // let mut max_blue_count = 0;
        // for draw_part in draw_parts {
        //     let color_parts = draw_part.split(", ");
        //     for color_part in color_parts {
        //         // println!("{}", color_part);
        //         let (color_count, color) = color_part.split_once(' ').unwrap();
        //         let color_count = color_count.parse::<i32>().unwrap();
        //         match color {
        //             "red" => {if color_count > max_red_count{max_red_count = color_count}}
        //             "green" => {if color_count > max_green_count{max_green_count = color_count}}
        //             "blue" => {if color_count > max_blue_count{max_blue_count = color_count}}
        //             _ => unreachable!("unknown: {color}")
        //         }
        //     }
        // }
        // result += max_red_count * max_green_count * max_blue_count;
    }
    dbg!(result);
}
