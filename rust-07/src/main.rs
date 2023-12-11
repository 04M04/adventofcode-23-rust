// Rust
const INPUT: &str = include_str!("../input_data.txt");

struct Person {
    cards: String,
    bit: u32,
    rank: u32,
    
}

fn main(){
    // let mut result = 0;

    let possible_cards = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];
    let possible_cards: [char; 12] = ["High card", "One pair", "Two pair", "Three of a kind", "Full house", "Four of a kind", "Five of a kind"];
    enum {
                
    }
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
