fn main() {
 let mut foo = Box::new(1);
 let mut bar = &mut foo;
 println!("{:?}", **bar);
}
