extern crate rustrip;
extern crate regex;
extern crate rand;

use std::env::args;
use std::io::{stdin, Read};
use regex::Regex;
use rand::distributions::{IndependentSample, Range};

fn gen_pass(min: usize, max: usize) -> String {
  let mut buf = String::new();
  let mut rng = rand::weak_rng();
  let ascii_range: Range<u8> = Range::new(33, 127);
  let length = Range::new(min, max).ind_sample(&mut rng);

  for _ in 0..length {
    buf.push(ascii_range.ind_sample(&mut rng) as char);
  }

  buf
}

fn main() {
  let mut input: String = String::new();

  if args().count() < 2 {
    let stdin = stdin(); {
      stdin.lock().read_to_string(&mut input).unwrap();
    }
  }

  else {
    input = args().skip(1).nth(0).unwrap();
  }

  let regex = Regex::new(&input).expect("Invalid regular expression.");

  loop {
    let pass: &str = &gen_pass(4, 12); {
      let tripcode = rustrip::encode(pass).expect("Password failed to encode...");
      if regex.is_match(&tripcode) {
        println!("{} -> {}", pass, tripcode);
      }
    }
  }
}
