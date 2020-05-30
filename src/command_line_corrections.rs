use ascii_table::{Align, AsciiTable, Column};
use rayon::prelude::*;
use std::time::Instant;

pub fn provide_words_corrections(corpus_file: String, words: Vec<&str>) {
    info!("Using corpus file located at {:}", corpus_file);
    info!("Words {:?}", words);
    let now = Instant::now();
    let sc = norvig_spell_checker::spell_checker::SpellChecker::from_corpus_file_par(&corpus_file);
    // let hm_par = sc.word_count.clone();
    //
    // let sc = norvig_spell_checker::spell_checker::SpellChecker::from_corpus_file(&corpus_file);
    // let hm_seq = sc.word_count.clone();

    // assert_eq!(hm_par, hm_seq);

    let word_correction: Vec<Vec<_>> = words
        .par_iter()
        .map(|word| {
            let word = word.to_string();
            let correction = sc.correction(&word);
            vec![word, correction]
        })
        .collect();

    print_correction(&word_correction);

    let new_now = Instant::now();
    info!(
        "It took {:?} to find corrections for words",
        new_now.duration_since(now)
    );
}

fn print_correction(word_correction: &Vec<Vec<String>>) {
    let mut ascii_table = AsciiTable::default();
    let mut word_column = Column::default();
    word_column.header = "Word".into();
    word_column.align = Align::Left;
    ascii_table.columns.insert(0, word_column);

    let mut suggestion_column = Column::default();
    suggestion_column.header = "Correction".into();
    suggestion_column.align = Align::Left;
    ascii_table.columns.insert(1, suggestion_column);

    ascii_table.print(word_correction);
}
