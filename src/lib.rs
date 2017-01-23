use std::ffi::{CString, CStr};
use std::os::raw::c_char;

extern {
  fn crypt(key: *const c_char, salt: *const c_char) -> *const c_char;
}

pub fn rust_crypt(key: &str, salt: &str) -> Option<String> {
  let c_key = CString::new(key).unwrap();
  let c_salt = CString::new(salt).unwrap();
  unsafe {
    let data_ptr = crypt(c_key.as_ptr(), c_salt.as_ptr());

    if data_ptr.is_null() {
      return None
    }

    else {
      let buffer = CStr::from_ptr(data_ptr).to_str();
      buffer.ok().and_then(|hash| Some(hash.into()))
    }
  }
}

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
  rust_crypt(source, &salt).and_then(|trip| {
    Some( (&trip[3..]).into() )
  })
}

#[cfg(test)]
#[test]
fn known_good_tripcode() {
  assert!(encode("tester").unwrap() == "k2VpQ2TTb.")
}
