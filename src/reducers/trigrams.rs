use std::collections::{HashMap, VecDeque};

use super::reducer::Reducer;

pub struct TrigramsReducer {
  map: HashMap<String, u64>,
  buf: VecDeque<String>,
}

impl Default for TrigramsReducer {
  fn default() -> Self {
    TrigramsReducer {
      map: HashMap::default(),
      buf: VecDeque::with_capacity(3),
    }
  }
}

impl Reducer for TrigramsReducer {
  type Output = HashMap<String, f64>;
  type OutputExtra = u64 /* len */;

  fn next(&mut self, s: &str) {
    if self.buf.len() == 2 {
      let gram1 = self.buf.pop_back().unwrap();
      let gram2 = self.buf.back().unwrap();
      let next_trigram = format!("{}{}{}", gram1, gram2, s);
      *self.map.entry(next_trigram).or_insert(0) += 1;
    }
    self.buf.push_front(s.to_owned());
  }

  fn into_output(self, len: Self::OutputExtra) -> Self::Output {
    self.map.into_iter().map(|(k, v)| (k, (v as f64) / ((len - 2) as f64))).collect()
  }
}