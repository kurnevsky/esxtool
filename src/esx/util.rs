pub fn name_to_string(name: [u8; 4]) -> String {
  name.iter().map(|&c| { c as char }).collect()
}
