use std::fs;
pub fn part_second(path_to_file : &str) {
    let input = fs::read_to_string(path_to_file).unwrap();
    let mut mul_count : u32 = 0;
    let input : Vec<char> = input.chars().collect();

    let mut dont = false;
    for mut i in 0..(input.len()-6) {
        if (input[i], input[i+1], input[i+2], input[i+3], input[i+4], input[i+5], input[i+6])
        == ('d', 'o', 'n', '\'', 't', '(', ')') {
            dont = true;
        } else if (input[i+2], input[i+3], input[i+4], input[i+5]) == ('d', 'o', '(', ')') {
            dont = false;
        }

        if (input[i+3], input[i+4], input[i+5], input[i+6]) == ('m', 'u', 'l', '(') && !dont {
            i += 7;
            let (mut x, mut y) : (u32, u32) = (0,0);
            let mut pass : u8 = 0;
            let mut num_str : String = String::new();
            while input[i] != ')' || input[i]!=',' || !input[i].is_numeric() {
                if input[i].is_numeric() {
                    num_str.push(input[i]);
                } else if input[i] == ',' && pass == 0 && input[i-1].is_numeric() {
                    x = num_str.parse::<u32>().unwrap();
                    num_str.clear();
                    pass = 1;
                } else if input[i] == ')' && pass == 1 {
                    y = num_str.parse::<u32>().unwrap();
                    num_str.clear();
                    pass = 2;
                } else {
                    break;
                }
                i +=1;
            }
            if pass == 2 {
                mul_count += x * y;
            }
        }
    }
    println!("Mul_count : {mul_count}");
}