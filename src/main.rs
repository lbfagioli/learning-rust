mod day1;
mod day2;
mod day3;

fn main() {
  // Day 1
  day1::morning::run();
  day1::afternoon::run();

  // Day 2
  day2::morning::run();
  let _ = day2::afternoon::run();

  // Day 3
  day3::morning::run();
  day3::afternoon::run();
}