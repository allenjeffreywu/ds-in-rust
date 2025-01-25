mod p1;
use p1::code_camp::{hamming_distance, is_permutation};

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
}
