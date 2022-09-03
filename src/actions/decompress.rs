pub extern crate unrar;

pub struct Decompresser<T> {
  pub entries: Vec<T>,
}

impl<T> Decompresser<T> {
  pub fn new() -> Decompresser<T> {
    Decompresser { entries: vec![] }
  }
}

pub trait Decompress<T> {
  fn decompress(&mut self, filename: String, path: String, password: Option<String>) -> &Decompresser<T>;

  fn get_file_list(&self, filename: String, password: Option<String>) -> Vec<T>;
}

