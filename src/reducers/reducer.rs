pub trait Reducer {
  type Output;
  type OutputExtra;
  fn next(&mut self, s: &str);
  fn into_output(self, extra: Self::OutputExtra) -> Self::Output;
}