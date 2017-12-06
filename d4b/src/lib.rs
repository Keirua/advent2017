#![allow(dead_code)]
trait CountPW {
    fn count_valid(pws:&str) -> i32;
}

pub struct CountRegular {} 

impl CountPW for CountRegular {
    fn count_valid(pws:&str) -> i32 {
        count_valid_pws_callback(pws, is_pw_valid)
    }
}

pub struct CountAnagrams {} 

impl CountPW for CountAnagrams {
    fn count_valid(pws:&str) -> i32 {
        count_valid_pws_callback(pws, is_pw_valid_anagrams)
    }
}

// Todo: do this better
fn sort_word(w:&str) -> String {
    let mut chars: Vec<char> = w.chars().collect();
    chars.sort();

    let mut string = String::new();
    for c in chars {
        string.push(c);
    }

    string
}

fn all_different(words:Vec<&str>) -> bool {
    for i in 0..words.len() {
        for j in 0..words.len() {
            if i != j && words[i] == words[j] {
                return false;
            }
        }

    }
    
    true
}

fn is_pw_valid_anagrams(pw:&str) -> bool{
    let words:Vec<&str> = pw.split(" ").collect::<Vec<&str>>();
    let mut sorted_words:Vec<String> = Vec::new();
    for w in words {
        let sorted_word = sort_word(w);
        sorted_words.push(sorted_word);
    }

    // todo: refactor and use all_different
    for i in 0..sorted_words.len() {
        for j in 0..sorted_words.len() {
            if i != j && sorted_words[i] == sorted_words[j] {
                return false;
            }
        }

    }
    
    true
}

fn is_pw_valid(pw:&str) -> bool {
    let words:Vec<&str>= pw.split(" ").collect();
    all_different(words)
}

fn count_valid_pws_iterative(pws:&str) -> i32 {
    let lines:Vec<&str>= pws.split("\n").collect();
    let mut count = 0;
    for l in lines {
        if is_pw_valid(l){
            count += 1;
        }
    }
    count
}

fn count_valid_pws(pws:&str) -> i32 {
    pws.split("\n")
        .fold(0, |sum, l| sum + is_pw_valid(l) as i32)
}

type ValidPasswordCallback = fn (&str) -> bool;

fn count_valid_pws_callback(pws:&str, callback: ValidPasswordCallback) -> i32 {
    pws.split("\n")
        .fold(0, |sum, l| sum + callback(l) as i32 )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_pw_valid_works() {
        assert_eq!(true, is_pw_valid("aa bb cc dd ee"));
        assert_eq!(false, is_pw_valid("aa bb cc dd aa"));
        assert_eq!(true, is_pw_valid("aa bb cc dd aaa"));
    }

    #[test]
    fn is_pw_valid_anagrams_works() {
        assert_eq!(true, is_pw_valid_anagrams("abcde fghij"));
        assert_eq!(false, is_pw_valid_anagrams("abcde xyz ecdab"));
        assert_eq!(true, is_pw_valid_anagrams("a ab abc abd abf abj"));
        assert_eq!(true, is_pw_valid_anagrams("iiii oiii ooii oooi oooo"));
        assert_eq!(false, is_pw_valid_anagrams("oiii ioii iioi iiio"));
    }

    #[test]
    fn count_valid_pw_works() {
        let pws = "aa bb cc dd ee
aa bb cc dd aa
aa bb cc dd aaa";

        assert_eq!(2, count_valid_pws(pws));
        assert_eq!(2, count_valid_pws_callback(pws, is_pw_valid));
    }
}



    /*aa bb cc dd ee is valid.
    aa bb cc dd aa is not valid - the word aa appears more than once.
    aa bb cc dd aaa is valid - aa and aaa count as different words.*/
