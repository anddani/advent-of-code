// From https://www.christopherbiscardi.com/advent-of-code-2020-in-rust-day-3-nested-iterators
fn process_slope(input: &str, (x, y): (usize, usize)) -> usize {
  input
      .lines()
      .step_by(y)
      .enumerate()
      .filter_map(|(step, row)| match row.chars().nth((step * x) % row.len()) {
          Some('.') => None,
          s => s,
      })
      .count()
}