use super::reducer::Reducer;

#[derive(Default)]
pub struct LenReducer(u64);

impl Reducer for LenReducer {
  type Output = u64;
  type OutputExtra = ();

  fn next(&mut self, _s: &str) {
    self.0 += 1;
  }

  fn into_output(self, _extra: Self::OutputExtra) -> Self::Output {
    self.0
  }
}