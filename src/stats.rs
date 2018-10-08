use std::collections::HashMap;

use esx::Esx;
use esx::record::Record;
use esx::util::name_to_string;

pub fn stats(esx: &Esx) {
  let mut map = HashMap::new();
  for record in &esx.records {
    *map.entry(record.name()).or_insert(0u32) += 1;
  }
  let mut vec = map
    .into_iter()
    .map(|(name, count)| (count, name))
    .collect::<Vec<_>>();
  vec.sort_unstable();
  for (count, name) in vec.into_iter().rev() {
    println!("{} {:6}", name_to_string(name), count);
  }

  let mut unknown = 0u32;
  for record in &esx.records {
    if let Record::Unknown(_) = record {
      unknown += 1;
    }
  }
  println!("Unknown: {}", unknown);
}
