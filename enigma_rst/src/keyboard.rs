use lazy_static::lazy_static;

// CREATE THE ALPHABET VARIABLE
lazy_static! {
    static ref ALPHABET: Vec<char> = {
        let mut temp_vec = Vec::new();
        for c in 'a'..='z' {
            temp_vec.push(c);
        }
        temp_vec
    };
}

pub fn forward(c: char) -> usize {
    let index = ALPHABET.iter().position(|&x| x == c);
    return index.unwrap().try_into().unwrap();
}

pub fn backward(i: usize) -> char {
    return ALPHABET[i];
}
