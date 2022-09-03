use super::decompress;
use unrar::Archive;
use unrar::archive::{Entry, OpenArchive};
use unrar::error::UnrarResult;
use crate::actions::decompress::{Decompress, Decompresser};

impl Decompress<unrar::archive::Entry> for Decompresser<Entry> {
  fn decompress(&mut self, filename: String, path: String, password: Option<String>) -> &Decompresser<Entry> {
    self.entries = unrar(filename, path, password);
    self
  }

  fn get_file_list(&self, filename: String, password: Option<String>) -> Vec<Entry> {
    get_file_list(filename, password)
  }
}

pub fn unrar(filename: String, path: String, password: Option<String>) -> Vec<Entry> {
  // Get the archive information and extract everything
  let archive = match password {
    Some(password) => Archive::with_password(filename, password),
    None => Archive::new(filename)
  }.extract_to(path);

  let mut archive: OpenArchive = match archive {
    Ok(res) => res,
    Err(error) => panic!("Problem unrar the file: {:?}", error),
  };

  let entries = archive.process();
  let entries = match entries {
    Ok(res) => res,
    Err(error) => panic!("Problem process archives: {:?}", error),
  };
  entries
}

pub fn get_file_list(filename: String, password: Option<String>) -> Vec<unrar::archive::Entry> {
  let list = match password {
    Some(password) => Archive::with_password(filename, password),
    None => Archive::new(filename)
  }.list();

  let mut list = if let Ok(mut list) = list {
    match list.process() {
      Ok(res) => res,
      Err(error) => panic!("Problem unrar the file: {:?}", error),
    }
  } else {
    panic!("Problem unrar the file: {:?}", list)
  };
  list
}