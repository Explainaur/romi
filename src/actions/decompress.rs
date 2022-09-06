pub extern crate unrar;

pub struct DCHandler {
  pub filename: String,
  pub path: String,
  pub password: Option<String>,
}

pub struct Decompresser<T> {
  entries: Vec<T>,
}

impl<T> Decompresser<T> {
  pub fn new() -> Decompresser<T> {
    Decompresser { entries: vec![] }
  }
  pub fn entries(&self) -> &Vec<T> {
    &self.entries
  }

  pub fn set_entries(&mut self, entries: Vec<T>) -> &mut Decompresser<T> {
    self.entries = entries;
    self
  }
}


impl<T> Decompresser<T> where Decompresser<T>: Decompress<T> {
  pub fn decompress_all(&mut self, dc_handler: DCHandler) -> &mut Self {
    <Self as Decompress<T>>::decompress_all(self, dc_handler)
  }

  pub fn get_file_list(&self, filename: String, password: Option<String>) -> Vec<T> {
    <Self as Decompress<T>>::get_file_list(self, filename, password)
  }
}

pub trait Decompress<T> {
  fn decompress_all(&mut self, dc_handler: DCHandler) -> &mut Self;

  fn get_file_list(&self, filename: String, password: Option<String>) -> Vec<T>;
}