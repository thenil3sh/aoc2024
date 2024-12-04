use std::fs;

fn part_1(path_to_input : &str) {
    let oreo: String = fs::read_to_string(path_to_input).unwrap();
    let mut order : i8 = 0;
    let mut safe_count : u16 = 0;
    let mut num_str: String = String::new();
    let mut vec : Vec<i8> = Vec::new(); 
    for i in oreo.chars() {
        if i.is_numeric() {
            num_str.push(i);
        } else if i == ' ' {
            vec.push(num_str.parse().unwrap());
            num_str.clear();
        } else if i == '\n' {
            vec.push( num_str.parse().unwrap());
            let mut safe: bool = true;
            for i in 1..vec.len() {
                if matches!(vec[i] - vec[i-1], 1..4) {
                    if i == 1 {
                        order = 1;
                    } else if order == -1 {
                        safe = false;
                        break;
                    }
                } else if matches!(vec[i] - vec[i-1], -3..0 ) {
                    if i == 1 {
                        order = -1;
                    } else if order == 1 {
                        safe = false;
                        break;
                    }
                } else {
                    safe = false;
                }
            }
            if safe {
                safe_count += 1;
            }
            vec.clear();
            num_str.clear()
        }
    }
    println!("Safe count {safe_count}");
}
