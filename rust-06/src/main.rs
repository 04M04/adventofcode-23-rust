// Rust
const INPUT: &str = include_str!("../input_data.txt");

fn main(){
    // let mut result = 0;

    let mut times_and_destinations: Vec<i32> = Vec::new();

    for line in INPUT.lines() {
        let (_, line) = line.split_once(":").unwrap();
        let splitted_elems: std::str::Split<'_, &str> = line.split(" ");

        for elem in splitted_elems{
            if elem != ""{
                times_and_destinations.push(elem.parse::<i32>().unwrap());
                dbg!(elem);
            }

            // times.push(elem.parse().unwrap());
        }
        for elem in times_and_destinations{
            dbg!(elem);
        }
    }
    // dbg!(result);
}
