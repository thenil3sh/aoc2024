fn part_first (path_to_file : &str) -> i32 {
    let input_string = fs::read_to_string(path_to_file).unwrap();
    // let oreo = oreo.as_str();
    let mut vec1 : Vec<i32> = Vec::new();
    let mut vec2 : Vec<i32> = Vec::new();
    let mut num_str = String::new();
    let mut is_left : bool  = true;
    for i in input_string.chars(){
        if i.is_numeric() {
            num_str.push(i);
        } else if i == '\n' && !is_left {
            vec2.push(num_str.parse().unwrap());
            num_str.clear();
            is_left = true;
        } else if i == ' ' && is_left {
            vec1.push(num_str.parse().unwrap());
            num_str.clear();
            is_left = false;
        } else {
        }
    }
    vec1.sort();
    vec2.sort();
    let mut sum  = 0;
    for i in 0..vec1.len() {
        let difference: i32 = vec1[i] - vec2[i];
        sum += if difference.is_negative() { - difference } else { difference };
    }
    sum
}