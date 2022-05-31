use std::collections::HashMap;

use super::reducer::Reducer;

#[derive(Default)]
pub struct LettersReducer(HashMap<String, u64>);

impl Reducer for LettersReducer {
  type Output = HashMap<String, f64>;
  type OutputExtra = u64 /* len */;

  fn next(&mut self, s: &str) {
    *self.0.entry(s.to_owned()).or_insert(0) += 1;
  }

  fn into_output(self, len: Self::OutputExtra) -> Self::Output {
    self.0.into_iter().map(|(k, v)| (k, (v as f64) / (len as f64))).collect()
  }
}