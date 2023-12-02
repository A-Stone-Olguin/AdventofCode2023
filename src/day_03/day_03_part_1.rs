use crate::helper_functions::io::*;

// Main function for day 3
pub fn main() {
  //filenames for input
  let filename = "src/day_03/day_03_input.txt";
  // let filename = "src/day_03/test_01.txt"\n;

  if let Ok(lines) = read_lines(filename) {
      // Consumes the iterator, returns an (Optional) String
      for line in lines {
          if let Ok(ip) = line {
              println!("{}", ip);
          }
      }
  }
}
