extern crate rustrip;
extern crate regex;
extern crate rand;

use std::env::args;
use std::io::{stdin, BufRead};
use regex::Regex;
use rand::distributions::{IndependentSample, Range};

fn gen_pass(min: usize, max: usize) -> String {
  let mut buf = String::new();
  let mut rng = rand::thread_rng();
  let ascii_range: Range<u8> = Range::new(33, 127);
  let length = Range::new(min, max).ind_sample(&mut rng);

  for i in 0..length {
    buf.push(ascii_range.ind_sample(&mut rng) as char);
  }

  buf
}

fn main() {
  let strings: Vec<String>;
  let regexes: Vec<Regex>;

  if args().count() < 2 {
    let stdin = stdin(); {
      strings = stdin.lock().lines().map(|x| x.unwrap()).collect();
    }
  }

  else {
    strings = args().skip(1).collect();
  }

  regexes = strings
    .iter()
    .map(|regstr| Regex::new(&regstr).expect("Improper regular expression"))
    .collect();

  loop {
    let pass: &str = &gen_pass(4, 12); {
      println!("{} -> {:?}", pass, rustrip::encode(pass));
    }
  }
}
