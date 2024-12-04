use std::fs;

fn part_2(path_to_input : &str) {
    let oreo: String = fs::read_to_string(path_to_input).unwrap();
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
            num_str.clear();
            for k in 0..vec.len() {
                let mut tem_vec : Vec<i8> = Vec::new();
                for j in 0..vec.len() {
                    if k != j {
                        tem_vec.push(vec[j]);
                    }
                }
                let mut order : bool = true;
                let mut safe = true;
                for j in 1..tem_vec.len() {
                    if matches!(tem_vec[j] - tem_vec[j-1], 1..4) {
                        if j == 1 {
                            order = true;
                        } else if !order {
                            safe = false;
                        }
                    } else if matches!(tem_vec[j] - tem_vec[j-1], -3..0) {
                        if j == 1 {
                            order = false;
                        } else if order {
                            safe = false;
                        }
                    } else {
                        safe = false;
                    }
                }
                if safe {
                    safe_count = safe_count + 1;
                    break;
                }

            }
            vec.clear();
        }
    }
    println!("Safe reports : {safe_count}");
}

