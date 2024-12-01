fn part_second (path_to_file : &str) -> u32 {
    let input_string = fs::read_to_string(path_to_file).unwrap();

    let mut lists: HashMap<u32, u32> = HashMap::new();
    let mut left_list : Vec<u32> = vec![];
    let mut num_str = String::new();
    let mut is_left : bool = true;
    let mut sum : u32 = 0;
    
    for i in input_string.chars(){
        if i.is_numeric() {
            num_str.push(i);
        } else if i == '\n' && !is_left {
            let num: u32  = num_str.parse().unwrap();
            match lists.get_mut(& num) {
                Some(x) => {*x += 1;},
                None => {lists.insert(num, 1);},
            }
            num_str.clear();
            is_left = true;
        } else if i == ' ' && is_left {
            left_list.push(num_str.parse().unwrap());
            num_str.clear();
            is_left = false;
        }
    }

    for i in 0..left_list.len() {
        if let Some(x) = lists.get(&left_list[i]){
            sum = sum + left_list[i] * x;
        }
    }
    sum
}