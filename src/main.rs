mod p1;

use p1::code_camp::{
    get_value_of_most_valuable_plot, hamming_distance, is_permutation, most_vowels,
    queens_are_safe, shared_birthdays,
};

fn main() {
    println!("Hello, world!");
    let a = vec![1, 2, 3, 4, 5];
    let b = vec![6, 7, 8, 9, 10];
    let distance = hamming_distance(&a, &b);
    println!("Hamming Distance is: {distance}");
    let a = vec![5, 5, 5];
    let b = vec![5, 5, 5];
    let is_perm = is_permutation(&a, &b);
    if is_perm {
        println!("{:?} is a permutation of {:?}", &a, &b);
    } else {
        println!("{:?} is not a permutation of {:?}", &a, &b);
    }
    let string_arr = ["hello", "allen", "AEIOU"];
    let most_vowel_idx = most_vowels(&string_arr);
    println!("In array {string_arr:?}, String at index {most_vowel_idx} has the most vowels");
    let a = shared_birthdays(3, 365);
    println!("{a} pair(s) of people shared a birthday");

    let board = [
        ['.', '.', '.', 'q'],
        ['.', '.', '.', '.'],
        ['.', '.', '.', '.'],
        ['q', '.', '.', '.'],
    ];
    let board: Vec<_> = board.iter().map(|&e| e.to_vec()).collect();
    println!("Queens are safe: {}", queens_are_safe(&board));

    let city = [
        [0, -2, -7, 0, -1],
        [9, 2, -6, 2, 0],
        [-4, 1, -4, 1, 0],
        [-1, 8, 0, -2, 1],
        [-10, 1, 1, -5, -6],
        [-15, -1, 1, 5, 4],
    ];
    let city: Vec<_> = city.iter().map(|&e| e.to_vec()).collect();
    println!(
        "Value of the most valuable plot in the city is: {}",
        get_value_of_most_valuable_plot(&city)
    );
}
