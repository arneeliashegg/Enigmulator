
pub fn init(ref_config: char) -> (Vec<char>, Vec<char>) {
    let mut left_list: Vec<char> = Vec::new();
    for c in 'a'..='z' {
        left_list.push(c);
    }

    let mut right_list: Vec<char> = Vec::new();
    right_list = config_right_list(ref_config);

    return (left_list, right_list);
}

pub fn config_right_list(ref_config: char) -> Vec<char> {
    let mut temp_list: Vec<char> = Vec::new();

    match ref_config {
        'a' => {
            for c in "ejmzalyxvbwfcrquontspikhgd".chars() {
                temp_list.push(c);
            }
            return temp_list;
        }
        'b' => {
            for c in "yruhqsldpxngokmiebfzcwvjat".chars() {
                temp_list.push(c);
            }
            return temp_list;
        }
        'c' => {
            for c in "fvpjiaoyedrzxwgctkuqsbnmhl".chars() {
                temp_list.push(c);
            }
            return temp_list;
        }
        _ => {
            panic!("Wrong reflector config!");
        }
    }
}

pub fn reflect(left_list: &Vec<char>, right_list: &Vec<char>, c_index: usize) -> usize {
    let right_char = right_list[c_index];
    let left_index = get_index(right_char, left_list);
    return left_index;
}

fn get_index(c: char, v: &Vec<char>) -> usize {
    //println!("c: {:?}, v: {:?}", c, v);
    if let c_index = v.iter().position(|&x| x == c) {
        return c_index.unwrap();
    } else {
        panic!("{:?} is not in da list!! reflectr", c);
    }
}
