// Rust
const INPUT: &str = include_str!("../input_data.txt");

fn main(){
    // let mut result = 0;

    // let mut positions = "";
    let mut positions: Vec<[&str; 3]> = Vec::new();
    let mut result = 0;

    let (directions, input_data) = INPUT
        .split_once("\r\n\r\n").unwrap();

    dbg!(directions);
    dbg!(input_data);
    for line in input_data.lines() {
        let elements: std::str::Split<'_, &str> = line.split("\r\n");
        for elem in elements{
            let (pos, data) = elem
                .split_once(" = (")
                .unwrap();
            let (left, right) = data
                .split_once(", ")
                .unwrap();
            let(right,_) = right.split_once(')').unwrap();
            positions
                .push([pos, left, right]);
            //TODO: Leo fragen wie man das optimieren kann!
        }
    }
    let mut last_path = "AAA";
    while last_path != "ZZZ"{
        for next_dir in directions.chars(){
            let mut found_corner: [&str; 3] = ["","",""];
            for elem in &positions {
                if elem[0] == last_path{
                    found_corner = *elem;
                }
            }; 
            // dbg!(found_corner);
            if next_dir == 'L'{
                last_path = found_corner[1];
            }
            else{
                last_path = found_corner[2];
            }
            result += 1;
            if last_path == "ZZZ"{
                break;
            }
        }
    }
        
    

    dbg!(result);
}
