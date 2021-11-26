#[path="common.rs"]
mod common;

fn is_vowel(c: char) -> bool {
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

/*
It does not contain the strings ab, cd, pq, or xy,
even if they are part of one of the other requirements.
 */
fn is_bad_seq(p: char, c: char) -> bool {
    (p == 'a' && c == 'b') ||
    (p == 'c' && c == 'd') ||
    (p == 'p' && c == 'q') ||
    (p == 'x' && c == 'y')
}

fn string_is_nice(line: &str) -> bool {
    let mut vowel_count = 0;
    let mut doubled_letter = false;
    let mut bad_seq = false;

    let mut prev_char = '0';
    for char in line.to_ascii_lowercase().chars() {
        if is_vowel(char) {
            vowel_count += 1;
        }
        doubled_letter |= char == prev_char;
        bad_seq |= is_bad_seq(prev_char, char);

        prev_char = char;
    }

    vowel_count >= 3 && doubled_letter && !bad_seq
}

pub fn part5() {
    // assert test cases
    assert!(string_is_nice("ugknbfddgicrmopn"));
    assert!(string_is_nice("aaa"));
    assert!(!string_is_nice("jchzalrnumimnmhp"));
    assert!(!string_is_nice("haegwjzuvuyypxyu"));
    assert!(!string_is_nice("dvszwmarrgswjxmb"));

    // run on input
    let mut count = 0;
    for result in common::common::read_file_linewise("input5.txt", &string_is_nice) {
        if result {
            count += 1;
        }
    }

    println!("Nice strings: {}", count);
}