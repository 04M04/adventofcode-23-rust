// Rust
const INPUT: &str = include_str!("../test_input_data.txt");
// const INPUT: &str = include_str!("../input_data.txt");

fn main(){
    let mut result = 0;
    let mut variable = 0;
    for line in INPUT.lines() {
        let (_, line) = line.split_once(": ").unwrap();
        

        let (numbers_part_1, numbers_part_2) = line.split_once(" | ").unwrap();
        dbg!(numbers_part_1);
        dbg!(numbers_part_2);
        let parts1 = numbers_part_1.split(" ");
        let parts2 = numbers_part_2.split(" ");
        
        for part1 in parts1{
            for part2 in parts2.clone() {
                if part1 == part2 && part1 != ""{
                    result += 1;
                    variable += 1;
                    dbg!(variable);
                }
                
            }
            
            if variable > 0{
                result += variable;
                variable = 0;
            }
            
            // for part2 in parts2{
            //     if part1 == part2{
            //         
            //     }
            // }
            
            
        }
    }
    dbg!(result);
}
