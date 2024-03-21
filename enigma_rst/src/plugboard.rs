use lazy_static::lazy_static;

lazy_static! {
    static ref left_rotor: Vec<char> = {
        let mut temp_vec = Vec::new();
        for c in 'a'..='z' {
            temp_vec.push(c);
        }
        temp_vec
    };
}

lazy_static! {
    static ref right_rotor: Vec<char> = {
        let mut temp_vec = Vec::new();
        for c in 'a'..='z' {
            temp_vec.push(c);
        }
        temp_vec
    };
}

pub fn main() {
    let mut right_list: Vec<char> = Vec::new();
    for c in 'a'..='z' {
        right_list.push(c);
    }

    //let plug_pairs = vec![('a', 'f'), ('b', 'g')];

    //let left_list = init(&plug_pairs);

    //let plugged_char = forward(&right_list, &left_list, 0);

    //let unplugged_char = backward(&right_list, &left_list, 5);

    //println!("{}", unplugged_char);
}

pub fn init(pairs: Option<&Vec<(char, char)>>) -> (Vec<char>, Vec<char>) {
    let mut char_i_pos: usize = 0;
    let mut char_ii_pos: usize = 0;

    let mut right_list: Vec<char> = Vec::new();
    for c in 'a'..='z' {
        right_list.push(c);
    }

    let mut left_list: Vec<char> = Vec::new();
    for c in 'a'..='z' {
        left_list.push(c);
    }

    match pairs {
        Some(pairs_vec) => {

            for &(char_i, char_ii) in pairs_vec {
                // Find and switch characters in left_rotor.
        
                char_i_pos = get_index(char_i, &left_list);
                left_list.remove(char_i_pos);
        
                char_ii_pos = get_index(char_ii, &left_list);
                left_list.remove(char_ii_pos);
        
                left_list.insert(char_i_pos, char_ii);
        
                left_list.insert(char_ii_pos + 1usize, char_i);
        
                //println!("{:?}", &left_list);
            }
            return (left_list, right_list);

        },
        _ => {
            return (left_list, right_list);
        }
    }
    /*/
    for &(char_i, char_ii) in pairs {
        // Find and switch characters in left_rotor.

        char_i_pos = get_index(char_i, &left_list);
        left_list.remove(char_i_pos);

        char_ii_pos = get_index(char_ii, &left_list);
        left_list.remove(char_ii_pos);

        left_list.insert(char_i_pos, char_ii);

        left_list.insert(char_ii_pos + 1usize, char_i);

        //println!("{:?}", &left_list);
    }

    return (left_list, right_list);
    */
}

pub fn forward(right_list: &Vec<char>, left_list: &Vec<char>, c_index: usize) -> usize {
    let right_list_char = right_list[c_index];
    let left_list_index = get_index(right_list_char, &left_list);
    return left_list_index;
}

pub fn backward(right_list: &Vec<char>, left_list: &Vec<char>, c_index: usize) -> usize {
    let left_list_char = left_list[c_index];
    let right_list_index = get_index(left_list_char, &right_list);
    return right_list_index;
}

pub fn get_index(c: char, v: &Vec<char>) -> usize {
    //println!("Trying to get index for {:?} in {:?}", c, v);
    if let c_index = v.iter().position(|&x| x == c) {
        return c_index.unwrap();
    } else {
        return 30usize;
    }
}
