pub fn main(rotor_conf: usize) -> usize {
    let (mut left_rotor, mut right_rotor, turnover_notch) = init(1);

    let x = forward(&left_rotor, &right_rotor, 0);
    return x;
}

pub fn init(right_list_config: usize) -> (Vec<char>, Vec<char>, char) {
    let mut left_list: Vec<char> = Vec::new();
    for c in 'a'..='z' {
        left_list.push(c);
    }

    let (right_list, turnover_notch) = rotor_config(right_list_config);

    //println!("config: {} - {:?}", right_list_config, right_list);

    return (left_list, right_list, turnover_notch);
}

pub fn rotor_config(config_num: usize) -> (Vec<char>, char) {
    let mut temp_list: Vec<char> = Vec::new();

    match config_num {
        0 => {
            for c in "ekmflgdqvzntowyhxuspaibrcj".chars() {
                temp_list.push(c);
            }
            return (temp_list, 'q');
        }
        1 => {
            for c in "ajdksiruxblhwtmcqgznpyfvoe".chars() {
                temp_list.push(c);
            }
            return (temp_list, 'e');
        }
        2 => {
            for c in "bdfhjlcprtxvznyeiwgakmusqo".chars() {
                temp_list.push(c);
            }
            return (temp_list, 'v');
        }
        3 => {
            for c in "esovpzjayquirhxlnftgkdcmwb".chars() {
                temp_list.push(c);
            }
            return (temp_list, 'j');
        }
        4 => {
            for c in "vzbrgityupsdnhlxawmjqofeck".chars() {
                temp_list.push(c);
            }
            return (temp_list, 'z');
        }
        _ => {
            panic!("Wrong Rotor Config!");
        }
    }
}

pub fn forward(left_list: &Vec<char>, right_list: &Vec<char>, c_index: usize) -> usize {
    //println!("rotor forwarding this: {:?}", right_list);
    let right_list_char = right_list[c_index];
    let left_list_index = get_index(right_list_char, &left_list);
    return left_list_index;
}

pub fn backward(left_list: &Vec<char>, right_list: &Vec<char>, c_index: usize) -> usize {
    let left_list_char = left_list[c_index];
    let right_list_index = get_index(left_list_char, &right_list);
    return right_list_index;
}

pub fn get_index(c: char, v: &Vec<char>) -> usize {
    if let c_index = v.iter().position(|&x| x == c) {
        return c_index.unwrap();
    } else {
        return 30usize;
    }
}

pub fn rotate(mut left_list: Vec<char>, mut right_list: Vec<char>) -> (Vec<char>, Vec<char>) {
    let mut moved_char = left_list[0];
    left_list.remove(0);
    left_list.push(moved_char);

    moved_char = right_list[0];
    right_list.remove(0);
    right_list.push(moved_char);

    return (left_list, right_list);
}

pub fn rotate_to_char(
    mut left_list: Vec<char>,
    mut right_list: Vec<char>,
    c: char,
) -> (Vec<char>, Vec<char>) {
    loop {
        if left_list[0] != c {
            (left_list, right_list) = rotate(left_list, right_list);
        } else if left_list[0] == c {
            return (left_list, right_list);
        }
    }
}
