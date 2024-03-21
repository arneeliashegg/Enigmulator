use std::io;

use reflector::reflect;

mod keyboard;
mod plugboard;
mod reflector;
mod rotor;

fn main() {
    println!("");
    println!("     <------------------------------------------------------------>");
    println!(" <----                                                            ---->");
    println!("<-- Digital Enigma (I) Emulator - Written in rust by Arne Hegghammer -->");
    println!(" <----                    18. March 2024                          ---->");
    println!("     <------------------------------------------------------------>");
    println!("");

    //let ((pb_left, pb_right), (r3_left, r3_right, r3_notch), (r2_left, r2_right, r2_notch), (r1_left, r1_right, r1_notch), (ref_left, ref_right)) = init(None, 1, 1, 1, 'a');

    let (
        (mut pb_left, mut pb_right),
        (mut r3_left, mut r3_right, mut r3_notch),
        (mut r2_left, mut r2_right, mut r2_notch),
        (mut r1_left, mut r1_right, mut r1_notch),
        (mut ref_left, mut ref_right),
    ) = init(Some(&vec![('a', 'b')]), 1, 1, 1, 'a');
    let mut enciphered: char = '_';

    loop {
        let mut input = String::new();
        let mut output = String::new();

        println!("|");
        eprint!("| Input Char => ");

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                //println!("{}", input);
                let input = input.trim();
                for c in input.chars() {
                    (
                        enciphered, pb_left, pb_right, r3_left, r3_right, r2_left, r2_right,
                        r1_left, r1_right, ref_left, ref_right,
                    ) = encipher(
                        c, pb_left, pb_right, r3_left, r3_right, r3_notch, r2_left, r2_right,
                        r2_notch, r1_left, r1_right, r1_notch, ref_left, ref_right,
                    );
                    output.push(enciphered);
                }
                println!("|-------------------------|");
                println!("| Enciphered Output: {}", output);
                println!("|-------------------------|")
            }
            Err(error) => {
                eprintln!("Error handling input: {}", error);
                break;
            }
        }
    }

    let c: char = 'b';
    let (
        (pb_left, pb_right),
        (r3_left, r3_right, r3_notch),
        (r2_left, r2_right, r2_notch),
        (r1_left, r1_right, r1_notch),
        (ref_left, ref_right),
    ) = init(Some(&vec![('a', 'b')]), 1, 1, 1, 'a');
    let (
        enciphered,
        pb_left,
        pb_right,
        r3_left,
        r3_right,
        r2_left,
        r2_right,
        r1_left,
        r1_right,
        ref_left,
        ref_right,
    ) = encipher(
        c, pb_left, pb_right, r3_left, r3_right, r3_notch, r2_left, r2_right, r2_notch, r1_left,
        r1_right, r1_notch, ref_left, ref_right,
    );
    println!("ORIGINAL: {:?} - ENCIPHERED: {:?}", c, enciphered);
    let (
        enciphered,
        pb_left,
        pb_right,
        r3_left,
        r3_right,
        r2_left,
        r2_right,
        r1_left,
        r1_right,
        ref_left,
        ref_right,
    ) = encipher(
        c, pb_left, pb_right, r3_left, r3_right, r3_notch, r2_left, r2_right, r2_notch, r1_left,
        r1_right, r1_notch, ref_left, ref_right,
    );
    println!("ORIGINAL: {:?} - ENCIPHERED: {:?}", c, enciphered);
    let (
        enciphered,
        pb_left,
        pb_right,
        r3_left,
        r3_right,
        r2_left,
        r2_right,
        r1_left,
        r1_right,
        ref_left,
        ref_right,
    ) = encipher(
        c, pb_left, pb_right, r3_left, r3_right, r3_notch, r2_left, r2_right, r2_notch, r1_left,
        r1_right, r1_notch, ref_left, ref_right,
    );
    println!("ORIGINAL: {:?} - ENCIPHERED: {:?}", c, enciphered);
}

fn init(
    plug_args: Option<&Vec<(char, char)>>,
    rotor_iii_setup: usize,
    rotor_ii_setup: usize,
    rotor_i_setup: usize,
    reflector_setup: char,
) -> (
    (Vec<char>, Vec<char>),
    (Vec<char>, Vec<char>, char),
    (Vec<char>, Vec<char>, char),
    (Vec<char>, Vec<char>, char),
    (Vec<char>, Vec<char>),
) {
    let (pb_left, pb_right) = plugboard::init(plug_args);
    let (r3_left, r3_right, r3_notch) = rotor::init(rotor_iii_setup);
    let (r2_left, r2_right, r2_notch) = rotor::init(rotor_ii_setup);
    let (r1_left, r1_right, r1_notch) = rotor::init(rotor_i_setup);
    let (ref_left, ref_right) = reflector::init(reflector_setup);

    return (
        (pb_left, pb_right),
        (r3_left, r3_right, r3_notch),
        (r2_left, r2_right, r2_notch),
        (r1_left, r1_right, r1_notch),
        (ref_left, ref_right),
    );
}

fn set_rotor_key(
    r3_left: Vec<char>,
    r3_right: Vec<char>,
    r2_left: Vec<char>,
    r2_right: Vec<char>,
    r1_left: Vec<char>,
    r1_right: Vec<char>,
    keys_str: &str,
) -> (
    (Vec<char>, Vec<char>),
    (Vec<char>, Vec<char>),
    (Vec<char>, Vec<char>),
) {
    let mut keys_list: Vec<char> = Vec::new();
    for c in keys_str.chars() {
        keys_list.push(c);
    }

    let (r3_left, r3_right) = rotor::rotate_to_char(r3_left, r3_right, keys_list[2]);
    let (r2_left, r2_right) = rotor::rotate_to_char(r2_left, r2_right, keys_list[1]);
    let (r1_left, r1_right) = rotor::rotate_to_char(r1_left, r1_right, keys_list[0]);

    return (
        (r3_left, r3_right),
        (r2_left, r2_right),
        (r1_left, r1_right),
    );
}

fn encipher(
    letter: char,
    pb_left: Vec<char>,
    pb_right: Vec<char>,
    mut r3_left: Vec<char>,
    mut r3_right: Vec<char>,
    r3_notch: char,
    mut r2_left: Vec<char>,
    mut r2_right: Vec<char>,
    r2_notch: char,
    mut r1_left: Vec<char>,
    mut r1_right: Vec<char>,
    r1_notch: char,
    ref_left: Vec<char>,
    ref_right: Vec<char>,
) -> (
    char,
    Vec<char>,
    Vec<char>,
    Vec<char>,
    Vec<char>,
    Vec<char>,
    Vec<char>,
    Vec<char>,
    Vec<char>,
    Vec<char>,
    Vec<char>,
) {
    let mut r1_left_new: Vec<char> = r1_left.clone();
    let mut r1_right_new: Vec<char> = r1_right.clone();
    let mut r2_left_new: Vec<char> = r2_left.clone();
    let mut r2_right_new: Vec<char> = r2_right.clone();
    let mut r3_left_new: Vec<char> = r3_left.clone();
    let mut r3_right_new: Vec<char> = r3_right.clone();

    if r2_left[0] == r2_notch && r3_left[0] == r3_notch {
        (r1_left_new, r1_right_new) = rotor::rotate(r1_left.clone(), r1_right.clone());
        (r2_left_new, r2_right_new) = rotor::rotate(r2_left.clone(), r2_right.clone());
        (r3_left_new, r3_right_new) = rotor::rotate(r3_left.clone(), r3_right.clone());
    } else if r3_left[0] == r3_notch {
        (r2_left_new, r2_right_new) = rotor::rotate(r2_left.clone(), r2_right.clone());
        (r3_left_new, r3_right_new) = rotor::rotate(r3_left.clone(), r3_right.clone());
    } else {
        (r3_left_new, r3_right_new) = rotor::rotate(r3_left.clone(), r3_right.clone());
    }

    let mut sig = keyboard::forward(letter);
    sig = plugboard::forward(&pb_right, &pb_left, sig);
    sig = rotor::forward(&r3_left_new, &r3_right_new, sig);
    sig = rotor::forward(&r2_left_new, &r2_right_new, sig);
    sig = rotor::forward(&r1_left_new, &r1_right_new, sig);
    sig = reflect(&ref_left, &ref_right, sig);
    sig = rotor::backward(&r1_left_new, &r1_right_new, sig);
    sig = rotor::backward(&r2_left_new, &r2_right_new, sig);
    sig = rotor::backward(&r3_left_new, &r3_right_new, sig);
    sig = plugboard::backward(&pb_right, &pb_left, sig);
    let letter = keyboard::backward(sig);
    return (
        letter,
        pb_left,
        pb_right,
        r3_left_new,
        r3_right_new,
        r2_left_new,
        r2_right_new,
        r1_left_new,
        r1_right_new,
        ref_left,
        ref_right,
    );
}
