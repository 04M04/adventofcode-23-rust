// Rust
const INPUT: &str = include_str!("../input_data.txt");

fn main(){
    // let mut result = 0;

    let mut time_values = Vec::new();

    for line in INPUT.lines() {
        let (_, line) = line.split_once(":").unwrap();
        let splitted_elems: std::str::Split<'_, &str> = line.split(" ");

        for elem in splitted_elems{
            elem = elem.parse().expect("Not a valid number");
            if(elem.is_digit(10)){
                time_values.push(elem.to_digit(10).unwrap());
                dbg!(time_values);
            }
            // times.push(elem.parse().unwrap());
            // dbg!(elem);
        }
    }
    // dbg!(result);
}
