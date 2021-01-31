use ascii_table::{Align, AsciiTable, Column};
use norvig_spell_checker::spell_checker::SpellChecker;
use rayon::prelude::*;
use std::time::Instant;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CorrectionRequest {
    pub words: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Correction {
    pub word: String,
    pub correction: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CorrectionResponse {
    pub corrections: Vec<Correction>,
}
// merge with above
pub fn find_words_corrections(
    spell_checker: &SpellChecker,
    words: Vec<String>,
) -> CorrectionResponse {
    info!("Words to correct: {:?}", words);
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

    print_correction(&corrections);
    CorrectionResponse { corrections }
}

pub fn print_correction(words_corrections: &Vec<Correction>) {
    let word_and_correction_vec: Vec<Vec<String>> = words_corrections
        .iter()
        .map(|correction| vec![correction.word.clone(), correction.correction.clone()])
        .collect();

    print_correction_inner(&word_and_correction_vec);
}

fn print_correction_inner(word_correction: &Vec<Vec<String>>) {
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
