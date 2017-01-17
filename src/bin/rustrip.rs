extern crate rustrip;
extern crate regex;

use std::env::args;
use std::io::{stdin, BufRead};
use regex::Regex;

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

  println!("{:?}", regexes);
}
