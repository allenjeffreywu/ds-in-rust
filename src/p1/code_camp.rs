use rand::Rng;
use std::collections::HashMap;
use std::fmt::Debug;

/// Determine the Hamming Distance between two arrays of ints.
/// Neither a or b are altered
/// Returns the Hamming Distance between the two arrays of ints.
pub fn hamming_distance(a: &[i32], b: &[i32]) -> i32 {
    assert_eq!(a.len(), b.len(), "a and b don't have the same length");
    let mut differences = 0;
    for (i, e) in a.iter().enumerate() {
        if e != &b[i] {
            differences += 1;
        }
    }
    return differences;
}

/// Determine if one array of ints is a permutation of another.
/// Neither a or b are altered
pub fn is_permutation(a: &[i32], b: &[i32]) -> bool {
    // never said that a and b can have the same array length
    if a.len() != b.len() {
        return false;
    }

    // This is not the leetcode is_permutation. We are dealing with ints here
    let mut map_a = HashMap::<i32, i32>::new();
    let mut map_b = HashMap::<i32, i32>::new();

    for (i, e) in a.iter().enumerate() {
        *map_a.entry(*e).or_insert(0) += 1;
        *map_b.entry(b[i]).or_insert(0) += 1;
    }
    println!("{map_a:?}");
    println!("{map_b:?}");
    return map_a.eq(&map_b);
}

/// Determine the index of the String that has the largest number of vowels
/// string_array is not altered as a result of this method
/// Returns the index of the String with the most vowels
pub fn most_vowels(string_array: &[&str]) -> usize {
    let mut most_vowels_idx = 0;
    let mut most_vowels_count = 0;
    for (i, e) in string_array.iter().enumerate() {
        let mut vowel_count = 0;
        let char_vec: Vec<char> = e.to_lowercase().chars().collect();
        for c in char_vec {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => vowel_count += 1,
                _ => (),
            }
        }
        if vowel_count > most_vowels_count {
            most_vowels_count = vowel_count;
            most_vowels_idx = i;
        }
    }
    return most_vowels_idx;
}

/// Perform an experiment simulating the birthday problem
/// Pick random birthdays for the given number of people
/// Return the number of pairs of people that share the same birthday
pub fn shared_birthdays(num_people: i32, num_days_in_year: i32) -> i32 {
    // 0 indexing days
    assert!(num_people > 0);
    assert!(num_days_in_year > 0);

    let mut birthdays = HashMap::<i32, i32>::new();

    for _ in 0..num_people {
        let birthday = rand::thread_rng().gen_range(0..num_days_in_year);
        *birthdays.entry(birthday).or_insert(0) += 1;
    }

    let mut counter = 0;

    for (_, value) in birthdays {
        if value > 1 {
            // use gauss formula here?
            counter += (value * (value - 1)) / 2;
        }
    }

    return counter;
}

/// Determine if the chess board represented by board is a safe setup
/// board must be a square matrix
/// 'q' in the board represents a queen, and '.' represents an empty square
/// board is not altered as a result of this method
/// This is the queen chess problem
pub fn queens_are_safe(board: &[Vec<char>]) -> bool {
    assert!(is_square(board));
    let board = board.as_ref();
    // unknown amount of Queens
    // we should add to a vector
    // let queens: Vec<(i32, i32)> = Vec::new();

    for i in 0..board.len() {
        println!("{:?}", board[i]);
    }
    return true;
}

pub fn is_square(board: &[Vec<char>]) -> bool {
    let board = board.as_ref();
    assert!(board.len() != 0);
    let length = board.len();
    for i in 0..board.len() {
        if board[i].len() != length {
            return false;
        }
    }
    return true;
}

pub fn only_contains<Matrix: AsRef<[Row]>, Row: AsRef<[char]> + Debug>() -> bool {
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hamming_distance_1() {
        let a = [1, 2, 3, 4, 5, 4, 3, 2, 1];
        let b = [1, 2, 10, 4, 5, 4, 3, -10, 1];
        assert_eq!(hamming_distance(&a, &b), 2);
    }

    #[test]
    fn hamming_distance_2() {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let b = [-6, -6, -6, -6, -6, -6, -6, -6, -6, -6];
        assert_eq!(hamming_distance(&a, &b), 10);
    }

    #[test]
    fn hamming_distance_3() {
        let a = [0; 50000];
        let b = [0; 50000];
        assert_eq!(hamming_distance(&a, &b), 0);
    }

    #[test]
    fn is_permutation_1() {
        let a = [1, 2, 3];
        let b = [2, 1, 3];
        assert_eq!(is_permutation(&a, &b), true);
    }

    #[test]
    fn is_permutation_2() {
        let a = [1, 2, 3];
        let b = [2, 1, 3, 3];
        assert_eq!(is_permutation(&a, &b), false);
    }

    #[test]
    fn is_permutation_3() {
        let a = [i32::MAX, i32::MIN, 0, 100000, i32::MAX / 2];
        let b = [100000, i32::MAX, i32::MAX / 2, 0, i32::MIN];
        assert_eq!(is_permutation(&a, &b), true);
    }

    #[test]
    fn is_permutation_4() {
        let a = [
            -180997325,
            1320698025,
            888999820,
            -829322186,
            -1307183500,
            1361152474,
            -1392440054,
            -214873900,
            -1855376901,
            -1960300168,
            -1953730082,
            425360258,
            1058753183,
            -677178196,
            1984530148,
            -1942949307,
            -1635374961,
            343505368,
            95408596,
            858251297,
            -1364562317,
            -582163733,
            -1883628785,
            -1285527161,
            -997097776,
            675098870,
            1137740700,
            -855636981,
            889189296,
            1637018879,
            -349690004,
            -1168073383,
            -1612354431,
            -2088449515,
            -1121376283,
            2124922217,
            -815737283,
            -1660242780,
            1619131037,
            1081153522,
            1073648075,
            -956169462,
            -274405274,
            -2029240037,
            1380457636,
            -16016534,
            1992992906,
            -325813896,
            487792570,
            751182527,
            846488663,
            1076151604,
            1182271636,
            1972603187,
            -334762275,
            1222230665,
            -46755651,
            1178240944,
            1189688565,
            796259192,
            -1747921057,
            1168761527,
            -17815162,
            -795578698,
            -670306006,
            -231963023,
            -479546877,
            -677303323,
            -753986951,
            -2017800189,
            1626756919,
            -225616125,
            -431441993,
            470194214,
            1553317444,
            -760637657,
            1909682175,
            -1868246283,
            -462279192,
            527864937,
            -1333121534,
            512809225,
            1088005122,
            1205405986,
            2123776813,
            762490306,
            1841971028,
            -64243115,
            524249355,
            -707602713,
            857997706,
            -2089897108,
            -1402438425,
            -1661232783,
            1806052731,
            1988722982,
            1135202741,
            -2064601181,
            -1855076946,
            1367451599,
        ];
        let b = [
            -17815162,
            762490306,
            524249355,
            -2064601181,
            -670306006,
            -1285527161,
            2123776813,
            1619131037,
            -2029240037,
            -2017800189,
            527864937,
            1073648075,
            -1942949307,
            857997706,
            796259192,
            -431441993,
            1909682175,
            -956169462,
            751182527,
            -1168073383,
            -462279192,
            -760637657,
            -334762275,
            -582163733,
            -1635374961,
            -1612354431,
            1168761527,
            -1953730082,
            -479546877,
            -2088449515,
            -677303323,
            -1660242780,
            -677178196,
            -64243115,
            1361152474,
            1076151604,
            675098870,
            -815737283,
            -180997325,
            1135202741,
            1222230665,
            -1121376283,
            1189688565,
            512809225,
            -349690004,
            1553317444,
            1972603187,
            -1307183500,
            1182271636,
            -46755651,
            -1883628785,
            -1661232783,
            -829322186,
            1984530148,
            1626756919,
            889189296,
            -855636981,
            1992992906,
            1137740700,
            -1960300168,
            1806052731,
            343505368,
            1367451599,
            -1402438425,
            1178240944,
            -231963023,
            -795578698,
            -2089897108,
            95408596,
            1205405986,
            -1855376901,
            470194214,
            -997097776,
            -1364562317,
            888999820,
            -325813896,
            1637018879,
            -1855076946,
            1081153522,
            1320698025,
            -16016534,
            -274405274,
            487792570,
            -1392440054,
            1841971028,
            -1747921057,
            1058753183,
            1380457636,
            858251297,
            -214873900,
            -225616125,
            425360258,
            1088005122,
            -707602713,
            2124922217,
            -1868246283,
            -1333121534,
            1988722982,
            846488663,
            -753986951,
        ];
        assert_eq!(is_permutation(&a, &b), true);
    }

    #[test]
    fn is_permutation_5() {
        let a = [0; 1000];
        let b = [0; 1000];
        assert_eq!(is_permutation(&a, &b), true);
    }

    #[test]
    fn is_permutation_6() {
        let a = [0; 1000];
        let b = [0; 1001];
        assert_eq!(is_permutation(&a, &b), false);
    }

    #[test]
    fn most_vowels_1() {
        let array_of_strings = ["aaaaaaa", "aieou"];
        assert_eq!(most_vowels(&array_of_strings), 0);
    }

    #[test]
    fn most_vowels_2() {
        let array_of_strings = [
            "Staying",
            "",
            "moo",
            "SEqUOIA NAtIOnAl FOrEst",
            "!!!!>>+_)(*&^%$#@!>)))???\\///\n\n/n",
        ];
        assert_eq!(most_vowels(&array_of_strings), 3);
    }

    #[test]
    fn most_vowels_3() {
        let array_of_strings = [
            "100,000,000",
            "XXX",
            "",
            "!!!!>>+_)(*&^%$#@!>)))???\\///\n\n/n",
        ];
        assert_eq!(most_vowels(&array_of_strings), 0);
    }

    #[test]
    fn most_vowels_4() {
        let array_of_strings = [""];
        assert_eq!(most_vowels(&array_of_strings), 0);
    }

    #[test]
    fn shared_birthdays_1() {
        let a = shared_birthdays(1, 365);
        assert_eq!(a, 0);
    }

    #[test]
    fn shared_birthdays_2() {
        let a = shared_birthdays(366, 365);
        assert!(a > 0);
    }

    #[test]
    fn shared_birthdays_3() {
        let a = shared_birthdays(100, 1);
        assert_eq!(a, 4950);
    }

    #[test]
    fn shared_birthdays_4() {
        let a = shared_birthdays(100000, 100000);
        assert!(a > 0);
    }
}
