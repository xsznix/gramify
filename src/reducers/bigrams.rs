use std::collections::{HashMap, VecDeque};

use super::reducer::Reducer;

pub struct BigramsReducer {
  map: HashMap<String, u64>,
  buf: VecDeque<String>,
}

impl Default for BigramsReducer {
  fn default() -> Self {
    BigramsReducer {
      map: HashMap::default(),
      buf: VecDeque::with_capacity(2),
    }
  }
}

impl Reducer for BigramsReducer {
  type Output = HashMap<String, f64>;
  type OutputExtra = u64 /* len */;

  fn next(&mut self, s: &str) {
    match self.buf.pop_back() {
      Some(last) => {
        let next_bigram = format!("{}{}", last, s);
        *self.map.entry(next_bigram).or_insert(0) += 1;
      }
      None => {}
    }
    self.buf.push_front(s.to_owned());
  }

  fn into_output(self, len: Self::OutputExtra) -> Self::Output {
    self.map.into_iter().map(|(k, v)| (k, (v as f64) / ((len - 1) as f64))).collect()
  }
}