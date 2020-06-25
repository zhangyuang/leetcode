fn main() {
  assert_eq!(100i32.wrapping_div_euclid(10), 10);
  assert_eq!((-128i32).wrapping_div_euclid(1), -128);
  let sum: i64 = 100;
  assert_eq!(sum.wrapping_rem(10), 0);
}
