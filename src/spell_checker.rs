extern crate regex;

use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

// def words(text): return re.findall(r'\w+', text.lower())
//
// WORDS = Counter(words(open('big.txt').read()))
//
// def P(word, N=sum(WORDS.values())):
// "Probability of `word`."
// return WORDS[word] / N
//
// def correction(word):
// "Most probable spelling correction for word."
// return max(candidates(word), key=P)
//
// def candidates(word):
// "Generate possible spelling corrections for word."
// return (known([word]) or known(edits1(word)) or known(edits2(word)) or [word])
//
// def known(words):
// "The subset of `words` that appear in the dictionary of WORDS."
// return set(w for w in words if w in WORDS)
//
// def edits1(word):
// "All edits that are one edit away from `word`."
// letters    = 'abcdefghijklmnopqrstuvwxyz'
// splits     = [(word[:i], word[i:])    for i in range(len(word) + 1)]
// deletes    = [L + R[1:]               for L, R in splits if R]
// transposes = [L + R[1] + R[0] + R[2:] for L, R in splits if len(R)>1]
// replaces   = [L + c + R[1:]           for L, R in splits if R for c in letters]
// inserts    = [L + c + R               for L, R in splits for c in letters]
// return set(deletes + transposes + replaces + inserts)
//
// def edits2(word):
// "All edits that are two edits away from `word`."
// return (e2 for e1 in edits1(word) for e2 in edits1(e1))

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn update_word_count(word_count: &mut HashMap<String, usize> , vec: &[String]) {
    //let mut word_count: HashMap<String, usize> = HashMap::new();
    for item in vec.iter() {
        *word_count
            .entry(item.to_string())
            .or_insert(0) += 1;

    }
}

pub fn f2() {
}

pub struct SpellChecker {
    word_count: HashMap<String, usize>
}

impl SpellChecker {
    pub fn initialize(corpus_fn: String) -> SpellChecker {
        let path_to_read = Path::new(&corpus_fn);
        println!("{:?}", path_to_read.to_str());

        let mut word_count: HashMap<String, usize> = HashMap::new();
        let re = regex::Regex::new(r"(?P<word>\w+)").unwrap();
        if let Ok(lines) = read_lines(path_to_read) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    //println!("{}", ip);
                    let words: Vec<String> = re
                        .captures_iter(&ip)
                        .map(|cap| { (&cap["word"]).to_ascii_lowercase() })
                        .collect();

                    update_word_count(&mut word_count, &words);
                }
            }
        }

        SpellChecker{
            word_count
        }
    }

    pub fn correction(&self, word: String) -> String {
        String::from("empty")
    }

    // fn candidates(&self, word: String) -> Vec<String> {
    //
    // }
    //
    // fn known(&self, words: Vec<String>) -> Vec<String> {
    //
    // }
    //
    // fn edits_distance_1(word: String) -> Vec<String> {
    //
    // }
    //
    // fn edits_distance_2(word: String) -> Vec<String> {
    //
    // }
}

pub fn count() {

}