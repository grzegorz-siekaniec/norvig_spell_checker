extern crate crossbeam;
extern crate crossbeam_channel;
extern crate libc;
extern crate num_cpus;
extern crate regex;
extern crate threadpool;

use crossbeam_channel::unbounded;
use libc::c_char;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
use threadpool::ThreadPool;

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
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub struct SpellChecker {
    pub word_count: HashMap<String, usize>,
}

// do a microbenchmarking for that
fn word_split(word: &String) -> Vec<(String, String)> {
    let word_splits: Vec<(String, String)> = word
        .char_indices()
        .into_iter()
        .map(|(i, _)| (word[..i].to_string(), word[i..].to_string()))
        .collect();

    word_splits
}

fn update_word_count(word_count: &mut HashMap<String, usize>, vec: &[String]) {
    //let mut word_count: HashMap<String, usize> = HashMap::new();
    for item in vec.iter() {
        *word_count.entry(item.to_string()).or_insert(0) += 1;
    }
}

fn deletes(splits: &[(String, String)]) -> Vec<String> {
    let deletes: Vec<String> = splits
        .into_iter()
        .map(|(l, r)| {
            let l0 = r.len();
            let mut chars = r.chars();
            chars.next();
            let l1 = r.len();
            assert_eq!(l0, l1);
            [l.to_string(), chars.collect()].concat()
        })
        .collect();
    deletes
}

fn transposes(splits: &[(String, String)]) -> Vec<String> {
    // transposes = [L + R[1] + R[0] + R[2:] for L, R in splits if len(R)>1]

    let transposes: Vec<String> = splits
        .into_iter()
        .filter(|(_, r)| r.len() > 1)
        .map(|(l, r)| {
            let mut chars = r.chars().clone();
            let r0 = chars.next().unwrap().to_string();
            let r1 = chars.next().unwrap().to_string();
            let rest: String = chars.collect();

            [l.clone(), r1, r0, rest].join("")
        })
        .collect();
    transposes
}

fn replaces(splits: &[(String, String)]) -> Vec<String> {
    let letters = "abcdefghijklmnopqrstuvwxyz";
    // replaces   = [L + c + R[1:]           for L, R in splits if R for c in letters]
    let mut replaces: Vec<String> = Vec::new();
    for (l, r) in splits {
        for c in letters.chars() {
            replaces.push([l.to_string(), c.to_string(), r[1..].to_string()].concat())
        }
    }

    replaces
}

fn inserts(splits: &[(String, String)]) -> Vec<String> {
    // inserts    = [L + c + R               for L, R in splits for c in letters]
    let letters = "abcdefghijklmnopqrstuvwxyz";
    let mut inserts: Vec<String> = Vec::new();
    for (l, r) in splits {
        for c in letters.chars() {
            inserts.push([l.to_string(), c.to_string(), r.to_string()].concat())
        }
    }

    inserts
}

fn edits_distance_1(word: &String) -> Vec<String> {
    // "All edits that are one edit away from `word`."
    // letters    = 'abcdefghijklmnopqrstuvwxyz'
    // deletes    = [L + R[1:]               for L, R in splits if R]
    // transposes = [L + R[1] + R[0] + R[2:] for L, R in splits if len(R)>1]
    // replaces   = [L + c + R[1:]           for L, R in splits if R for c in letters]
    // inserts    = [L + c + R               for L, R in splits for c in letters]

    let word_splits = word_split(&word);

    // let mut edits_1: Vec<String> = vec![];
    // crossbeam::scope(|s| {
    //     let th_deletes = s.spawn(|_| deletes(&word_splits));
    //     let th_transposes = s.spawn(|_| transposes(&word_splits));
    //     let th_replaces = s.spawn(|_| replaces(&word_splits));
    //     let th_inserts = s.spawn(|_| inserts(&word_splits));
    //
    //     edits_1.extend(th_deletes.join().unwrap());
    //     edits_1.extend(th_transposes.join().unwrap());
    //     edits_1.extend(th_replaces.join().unwrap());
    //     edits_1.extend(th_inserts.join().unwrap());
    // }).unwrap();

    // left for benchmarking
    let mut edits_1: Vec<String> = [
        deletes(&word_splits),
        transposes(&word_splits),
        replaces(&word_splits),
        inserts(&word_splits),
    ]
    .concat();

    edits_1.sort();
    edits_1.dedup();

    // println!("All edits with distance 1");
    // for word in &edits_1 {
    //     println!("{}", word);
    // }

    edits_1
}

fn edits_distance_2(edits_1: &Vec<String>) -> Vec<String> {
    // return (e2 for e1 in edits1(word) for e2 in edits1(e1))

    let mut edits_2: Vec<String> = Vec::new();
    for e1 in edits_1 {
        edits_2.extend(edits_distance_1(&e1));
    }

    edits_2
}

impl SpellChecker {
    pub fn from_corpus_file_par(corpus_fn: &String) -> SpellChecker {
        info!("Using corpus file located at {:}", corpus_fn);

        let (snd, rcv) = unbounded();

        let path_to_read = Path::new(&corpus_fn);
        let pool = ThreadPool::new(num_cpus::get());
        info!("Using {} threads", num_cpus::get());

        // let file = File::open("foo.txt")?;
        // let reader = BufReader::new(file);
        //
        // for line in reader.lines() {
        //     println!("{}", line?);
        //}
        //let re = regex::Regex::new(r"(?P<word>\w+)").unwrap();
        let now = Instant::now();

        if let Ok(lines) = read_lines(path_to_read) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                let snd = snd.clone();
                pool.execute(move || {
                    if let Ok(ip) = line {
                        let words: Vec<String> = ip
                            .to_ascii_lowercase()
                            .split_ascii_whitespace()
                            .map(|word_ref| word_ref.to_owned())
                            .collect();

                        snd.send(words).unwrap();
                    }
                });
            }
        }
        drop(snd);
        pool.join();

        let mut word_count: HashMap<String, usize> = HashMap::new();
        for words in rcv.iter() {
            update_word_count(&mut word_count, &words);
        }

        let new_now = Instant::now();
        info!(
            "Spell checker initialized. It took {:?} to instantate (par) ",
            new_now.duration_since(now)
        );
        SpellChecker { word_count }
    }

    pub fn from_corpus_file(corpus_fn: &String) -> SpellChecker {
        let path_to_read = Path::new(&corpus_fn);

        let mut word_count: HashMap<String, usize> = HashMap::new();
        let re = regex::Regex::new(r"(?P<word>\w+)").unwrap();

        // let file = File::open("foo.txt")?;
        // let reader = BufReader::new(file);
        //
        // for line in reader.lines() {
        //     println!("{}", line?);
        //}
        let now = Instant::now();
        if let Ok(lines) = read_lines(path_to_read) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(ip) = line {
                    //println!("{}", ip);
                    let words: Vec<String> = re
                        .captures_iter(&ip)
                        .map(|cap| (&cap["word"]).to_ascii_lowercase())
                        .collect();

                    // use messaging here
                    update_word_count(&mut word_count, &words);
                }
            }
        }
        let new_now = Instant::now();
        info!("It took {:?} to instantate ", new_now.duration_since(now));
        SpellChecker { word_count }
    }

    pub fn from_hash_map(word_count: HashMap<String, usize>) -> SpellChecker {
        SpellChecker { word_count }
    }

    pub fn correction(&self, word: &String) -> String {
        // def P(word, N=sum(WORDS.values())):
        // "Probability of `word`."
        // return WORDS[word] / N
        //
        // def correction(word):
        // "Most probable spelling correction for word."
        // return max(candidates(word), key=P)

        let mut candidates = self.candidates(&word);
        candidates.sort_by_cached_key(|word| {
            if self.word_count.contains_key(word) {
                self.word_count[word]
            } else {
                0
            }
        });
        candidates.dedup();

        // let mut i = 0;
        // for word in (&candidates).into_iter().rev() {
        //     println!("{} {}", word, if self.word_count.contains_key(word) { self.word_count[word] } else { 0 } );
        //     i += 1;
        //     if i > 10 {break;}
        // }

        candidates.last().unwrap().to_string()
    }

    fn known(&self, words: &[String]) -> Vec<String> {
        let known_words = words
            .iter()
            .filter(|&word| self.word_count.contains_key(word))
            .map(|word| word.to_string())
            .collect();
        known_words
    }

    fn candidates(&self, word: &String) -> Vec<String> {
        let v0: Vec<String> = vec![word.to_string()];
        let v = self.known(&v0);
        if !v.is_empty() {
            return v;
        }

        let edits_1 = edits_distance_1(word);
        let known_edits_1 = self.known(&edits_1);
        if !known_edits_1.is_empty() {
            return known_edits_1;
        }

        let v = self.known(&edits_distance_2(&edits_1));
        //let v = self.known(&edits_distance_2(&word));
        if !v.is_empty() {
            return v;
        }

        v0
    }
}

#[no_mangle]
pub extern "C" fn spell_checker_from_corpus_file(zip: *const c_char) -> *mut SpellChecker {
    let corpus_file = unsafe {
        assert!(!zip.is_null());
        CStr::from_ptr(zip).to_str()
    };

    let corpus_file: String = {
        if corpus_file.is_ok() {
            corpus_file.unwrap().to_string()
        } else {
            panic!("");
        }
    };

    Box::into_raw(Box::new(SpellChecker::from_corpus_file(&corpus_file)))
}

#[no_mangle]
pub extern "C" fn spell_checker_free(ptr: *mut SpellChecker) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        println!("Free Spell Checker in rust");
        Box::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn spell_checker_correction(
    ptr: *const SpellChecker,
    word: *const c_char,
) -> *mut c_char {
    let spell_checker = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    let word = unsafe {
        //assert!(!spell_checker.is_null());
        CStr::from_ptr(word)
    };
    let word_str = word.to_str().unwrap().to_string();
    let correction = spell_checker.correction(&word_str);
    unsafe { CString::from_vec_unchecked(correction.as_bytes().to_vec()).into_raw() }
}

#[cfg(test)]
mod tests {
    use crate::spell_checker::{deletes, transposes, word_split, SpellChecker};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_known() {
        let word_count: HashMap<String, usize> = {
            let mut word_count: HashMap<&str, usize> = HashMap::new();
            word_count.insert("one", 1);
            word_count.insert("two", 1);
            word_count.insert("the", 1);

            word_count
                .iter()
                .map(|(word, count)| (word.to_string(), *count))
                .collect()
        };

        let spell_checker = SpellChecker::from_hash_map(word_count);

        let candidates: Vec<String> = vec![
            String::from("one"),
            String::from("the"),
            String::from("eth"),
            String::from("oen"),
        ];
        let known_words = spell_checker.known(&candidates);

        let expected: HashSet<String> = vec![String::from("the"), String::from("one")]
            .into_iter()
            .collect();

        let actual: HashSet<String> = known_words.into_iter().collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_splits() {
        let word = String::from("the");
        let word_splits = word_split(&word);
        assert_eq!(word_splits.len(), 3);

        let expected: Vec<_> = vec![("", "the"), ("t", "he"), ("th", "e")];

        let expected: HashSet<_> = expected
            .into_iter()
            .map(|split| (split.0.to_string(), split.1.to_string()))
            .collect();

        let actual: HashSet<_> = word_splits.into_iter().collect();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deletes() {
        let split: Vec<_> = vec![("", "the"), ("t", "he")];

        let split: Vec<(_, _)> = split
            .into_iter()
            .map(|(l, r)| (l.to_string(), r.to_string()))
            .collect();

        let mut act_deletes = deletes(&split);
        act_deletes.sort();

        assert_eq!(2, act_deletes.len());

        let mut exp_deletes: Vec<String> = vec!["he", "te"]
            .into_iter()
            .map(|word| word.to_string())
            .collect();
        exp_deletes.sort();

        assert_eq!(act_deletes, exp_deletes);
    }

    #[test]
    fn test_transposes() {
        let split: Vec<_> = vec![("", "the"), ("t", "he")];

        let split: Vec<(_, _)> = split
            .into_iter()
            .map(|(l, r)| (l.to_string(), r.to_string()))
            .collect();

        let mut act_transposes = transposes(&split);
        act_transposes.sort();

        assert_eq!(2, act_transposes.len());

        let mut exp_transposes: Vec<String> = vec!["hte", "teh"]
            .into_iter()
            .map(|word| word.to_string())
            .collect();
        exp_transposes.sort();

        assert_eq!(act_transposes, exp_transposes);
    }

    #[test]
    fn test_transposes_unicode() {
        let split: Vec<_> = vec![("", "Здр"), ("З", "др")];

        let split: Vec<(_, _)> = split
            .into_iter()
            .map(|(l, r)| (l.to_string(), r.to_string()))
            .collect();

        let mut act_transposes = transposes(&split);
        act_transposes.sort();

        assert_eq!(2, act_transposes.len());

        let mut exp_transposes: Vec<String> = vec!["дЗр", "Зрд"]
            .into_iter()
            .map(|word| word.to_string())
            .collect();
        exp_transposes.sort();

        assert_eq!(act_transposes, exp_transposes);
    }

    #[test]
    fn test_transposes_polish() {
        let split: Vec<_> = vec![("", "żąę"), ("ż", "ąę")];

        let split: Vec<(_, _)> = split
            .into_iter()
            .map(|(l, r)| (l.to_string(), r.to_string()))
            .collect();

        let mut act_transposes = transposes(&split);
        act_transposes.sort();

        assert_eq!(2, act_transposes.len());

        let mut exp_transposes: Vec<String> = vec!["ążę", "żęą"]
            .into_iter()
            .map(|word| word.to_string())
            .collect();
        exp_transposes.sort();

        assert_eq!(act_transposes, exp_transposes);
    }

    #[test]
    fn test_chain_of_transpose_and_delete() {
        let word = String::from("peotryy");

        let mut candidates: Vec<String> = Vec::new();

        let word_splits = word_split(&word);
        let deletes = deletes(&word_splits);

        let peotry = String::from("peotry");
        assert!(deletes.contains(&peotry));

        for word in &deletes {
            let word_splits = word_split(&word);
            let deletes = transposes(&word_splits);
            candidates.extend(deletes.into_iter());
        }

        candidates.sort();
        candidates.dedup();

        let poetry = String::from("poetry");
        assert!(candidates.contains(&poetry));
    }

    #[test]
    fn test_correction() {
        let file: String =
            String::from("/home/gsiekaniec/devel/rust_projects/norvig_spell_checker/data/big.txt");
        let sc = SpellChecker::from_corpus_file(&file);

        let misspelling_correct: Vec<(_, _)> = vec![
            ("speling", "spelling"),              // insert
            ("korrectud", "corrected"),           // replace 2
            ("bycycle", "bicycle"),               // replace
            ("inconvient", "inconvenient"),       // insert 2
            ("arrainged", "arranged"),            // delete
            ("peotry", "poetry"),                 // transpose
            ("peotryy", "poetry"),                // transpose + delete
            ("word", "word"),                     // known
            ("quintessential", "quintessential"), // unknown
        ];

        let misspelling_correct: Vec<(_, _)> = misspelling_correct
            .into_iter()
            .map(|(misspelling, correction)| (misspelling.to_string(), correction.to_string()))
            .collect();

        for (misspelling, correction) in misspelling_correct {
            assert_eq!(sc.correction(&misspelling), correction);
        }
    }
}
