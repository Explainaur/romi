mod actions;

use crate::actions::decompress::{Decompresser, Decompress, unrar::archive::Entry};
fn main() {
  let file = String::from("/Users/dyf/code/solo/project/Romi/test/hellbound.cbr");
  let path = String::from("/Users/dyf/code/solo/project/Romi/test/1");
  let mut decompresser = Decompresser::<Entry>::new();
  let res = decompresser.decompress(file.clone(), path, None).get_file_list(file, None);
  println!("{:?}", res);
}
