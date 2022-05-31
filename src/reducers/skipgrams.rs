use std::collections::{HashMap, VecDeque};

use super::reducer::Reducer;

pub struct SkipgramsReducer {
  map: HashMap<String, u64>,
  buf: VecDeque<String>,
}

impl Default for SkipgramsReducer {
  fn default() -> Self {
    SkipgramsReducer {
      map: HashMap::default(),
      buf: VecDeque::with_capacity(3),
    }
  }
}

impl Reducer for SkipgramsReducer {
  type Output = HashMap<String, f64>;
  type OutputExtra = u64 /* len */;

  fn next(&mut self, s: &str) {
    if self.buf.len() == 2 {
      let front_gram = self.buf.pop_back().unwrap();
      let next_trigram = format!("{}{}", front_gram, s);
      *self.map.entry(next_trigram).or_insert(0) += 1;
    }
    self.buf.push_front(s.to_owned());
  }

  fn into_output(self, len: Self::OutputExtra) -> Self::Output {
    self.map.into_iter().map(|(k, v)| (k, (v as f64) / ((len - 2) as f64))).collect()
  }
}