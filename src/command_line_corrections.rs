use ascii_table::{Align, AsciiTable, Column};
use norvig_spell_checker::spell_checker::SpellChecker;
use rayon::prelude::*;
use std::time::Instant;

pub fn provide_words_corrections(spell_checker: &SpellChecker, words: Vec<&str>) {
    info!("Words {:?}", words);
    let now = Instant::now();

    // let hm_par = sc.word_count.clone();
    //
    // let sc = norvig_spell_checker::spell_checker::SpellChecker::from_corpus_file(&corpus_file);
    // let hm_seq = sc.word_count.clone();

    // assert_eq!(hm_par, hm_seq);

    let word_correction: Vec<Vec<_>> = words
        .par_iter()
        .map(|word| {
            let word = word.to_string();
            let correction = spell_checker.correction(&word);
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CorrectionRequest {
    pub words: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Correction {
    word: String,
    correction: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CorrectionResponse {
    corrections: Vec<Correction>,
}
// merge with above
pub fn provide_words_corrections_req(
    spell_checker: &SpellChecker,
    words: Vec<String>,
) -> CorrectionResponse {
    info!("Words {:?}", words);
    let now = Instant::now();

    let corrections: Vec<Correction> = words
        .into_par_iter()
        .map(|word| {
            let correction = spell_checker.correction(&word);
            Correction { word, correction }
        })
        .collect();

    let new_now = Instant::now();
    info!(
        "It took {:?} to find corrections for words",
        new_now.duration_since(now)
    );
    CorrectionResponse { corrections }
}
