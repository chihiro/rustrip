extern crate pwhash;

// hash_with(salt: &str, pass: &str) -> Result<String>
use pwhash::unix_crypt::hash_with as crypt;

/**
  **This function doesn't convert to Shift-JIS.**
 
  Performs the operation needed to produce the salt
  supplied to crypt to produce the outputs used
  on the most common imageboards.
 
  Some implementations of the tripcode algorithm
  use conversion to SJIS as the first step in salting.
*/
pub fn saltify(source: &str) -> String {
  let mut buffer = String::from(source);
  buffer.push_str("H..");
  buffer.chars().skip(1).take(2).map(|rune| {
    match rune {
      ':'  => 'A',
      ';'  => 'B',
      '<'  => 'C',
      '='  => 'D',
      '>'  => 'E',
      '?'  => 'F',
      '@'  => 'G',
      '['  => 'a',
      '\\' => 'b',
      ']'  => 'c',
      '^'  => 'd',
      '_'  => 'e',
      '`'  => 'f',
      
      '.' ... 'z' => {
        rune
      },
 
      _    => '.'
    }
  }).collect::<String>()
}

/// Produces a tripcode compatible with most popular imageboards.
pub fn encode(source: &str) -> Option<String> {
  let salt = saltify(source);
  crypt(&salt, source).ok().and_then(|trip| {
    Some( (&trip[3..]).into() )
  })
}

#[cfg(test)]
#[test]
fn known_good_tripcode() {
  assert!(encode("tester").unwrap() == "k2VpQ2TTb.")
}
