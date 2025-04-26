use sm_algorithm::sm_3::hash;

fn main() {
  let message = "Hello".as_bytes();
  let size = 40u64;

  let result = hash(message, size).unwrap();

  println!("{:?}", result);
}
