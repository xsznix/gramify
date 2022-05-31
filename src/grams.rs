use std::collections::HashMap;
use std::str;

use super::reducers::{
  bigrams::BigramsReducer,
  len::LenReducer,
  letters::LettersReducer,
  reducer::Reducer,
  skipgrams::SkipgramsReducer,
  trigrams::TrigramsReducer,
};

use regex::Regex;
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Grams {
  length: u64,
  letters: HashMap<String, f64>,
  bigrams: HashMap<String, f64>,
  skipgrams: HashMap<String, f64>,
  trigrams: HashMap<String, f64>,
}

impl Grams {
  pub fn apply_letter_threshold(&mut self, threshold: f64) {
    self.letters.retain(|_, freq| *freq > threshold / 1_000_000f64);
  }

  pub fn apply_letter_pattern(&mut self, pattern: Regex) {
    self.letters.retain(|s, _| pattern.is_match(s))
  }

  pub fn apply_bigram_threshold(&mut self, threshold: f64) {
    self.bigrams.retain(|_, freq| *freq > threshold / 1_000_000f64);
  }

  pub fn apply_bigram_pattern(&mut self, pattern: Regex) {
    self.bigrams.retain(|s, _| pattern.is_match(s))
  }

  pub fn apply_skipgram_threshold(&mut self, threshold: f64) {
    self.skipgrams.retain(|_, freq| *freq > threshold / 1_000_000f64);
  }

  pub fn apply_skipgram_pattern(&mut self, pattern: Regex) {
    self.skipgrams.retain(|s, _| pattern.is_match(s))
  }

  pub fn apply_trigram_threshold(&mut self, threshold: f64) {
    self.trigrams.retain(|_, freq| *freq > threshold / 1_000_000f64);
  }

  pub fn apply_trigram_pattern(&mut self, pattern: Regex) {
    self.trigrams.retain(|s, _| pattern.is_match(s))
  }
}

pub fn make_grams(corpus: &[u8]) -> Grams {
  let corpus_str = unsafe { str::from_utf8_unchecked(corpus) };
  let mut len_reducer = LenReducer::default();
  let mut letters_reducer = LettersReducer::default();
  let mut bigrams_reducer = BigramsReducer::default();
  let mut skipgrams_reducer = SkipgramsReducer::default();
  let mut trigrams_reducer = TrigramsReducer::default();
  for (i, grapheme) in UnicodeSegmentation::grapheme_indices(corpus_str, true) {
    if i % 1_000_000 == 0 {
      eprint!("Processing: {}\r", i);
    }
    len_reducer.next(grapheme);
    letters_reducer.next(grapheme);
    bigrams_reducer.next(grapheme);
    skipgrams_reducer.next(grapheme);
    trigrams_reducer.next(grapheme);
  }
  let len = len_reducer.into_output(());
  Grams {
    length: len,
    letters: letters_reducer.into_output(len),
    bigrams: bigrams_reducer.into_output(len),
    skipgrams: skipgrams_reducer.into_output(len),
    trigrams: trigrams_reducer.into_output(len),
  }
}
