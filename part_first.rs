use std::fs;
pub fn part_first(path_to_file : &str) {
    let stringed = fs::read_to_string(path_to_file).unwrap();
    
    let oreo : Vec<char> = stringed.chars().collect();
    // println!("{oreo:?}");
    let mut mul_count : u32 = 0;
    for mut i in 0..(oreo.len()-3) {
        if (oreo[i], oreo[i+1], oreo[i+2], oreo[i+3]) == ('m', 'u', 'l', '(') {
            i += 4;
            let (mut x, mut y) : (u32, u32) = (0,0);
            let mut pass : u8 = 0;
            let mut num_str : String = String::new();
            while oreo[i] != ')' || oreo[i]!=',' || !oreo[i].is_numeric() {
                if oreo[i].is_numeric() {
                    num_str.push(oreo[i]);
                } else if oreo[i] == ',' && pass == 0 && oreo[i-1].is_numeric() {
                    x = num_str.parse::<u32>().unwrap();
                    num_str.clear();
                    pass = 1;
                } else if oreo[i] == ')' && pass == 1 {
                    y = num_str.parse::<u32>().unwrap();
                    num_str.clear();
                    pass = 2;  
                } else {
                    break;
                }
                i +=1;
            }
            if pass == 2 {
                mul_count += x*y;
            }
        }
    }
    println!("Sum of all multiplications : {mul_count}");
}