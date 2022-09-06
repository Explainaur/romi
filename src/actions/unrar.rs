use unrar::Archive;
use serde::Serialize;
pub use unrar::archive::{Entry, OpenArchive, EntryFlags};
use crate::actions::decompress::{DCHandler, Decompress, Decompresser};

#[derive(Serialize)]
pub struct RarEntry {
  pub idx: usize,
  pub path: String,
  pub name: String,
  pub size: u32,
}

impl RarEntry {
  pub fn new(idx: usize, path: String, entry: &Entry) -> RarEntry {
    RarEntry {
      idx,
      path,
      name: entry.filename.clone(),
      size: entry.unpacked_size,
    }
  }
}

// TODO: add a macro to refine these process
impl Decompress<RarEntry> for Decompresser<RarEntry> {
  fn decompress_all(&mut self, dc_handler: DCHandler) -> &mut Self {
    self.set_entries(unrar(dc_handler));
    self
  }

  fn get_file_list(&self, filename: String, password: Option<String>) -> Vec<RarEntry> {
    get_file_list(filename, password)
  }
}

pub fn unrar(dc_handler: DCHandler) -> Vec<RarEntry> {
  let DCHandler { filename, path, password } = dc_handler;

  // Get the archive information and extract everything
  let archive = match password {
    Some(password) => Archive::with_password(filename, password),
    None => Archive::new(filename)
  }.extract_to(path.clone());

  let mut archive: OpenArchive = match archive {
    Ok(res) => res,
    Err(error) => panic!("Problem unrar the file: {:?}", error),
  };

  let entries = archive.process();
  let entries = match entries {
    Ok(res) => res,
    Err(error) => panic!("Problem process archives: {:?}", error),
  };

  let mut res: Vec<RarEntry> = vec![];
  for i in 0..entries.len() {
    res.push(RarEntry::new(i, path.clone() + &entries[i].filename, &entries[i]));
  }
  sort_entries(&mut res);
  res
}

pub fn get_file_list(filename: String, password: Option<String>) -> Vec<RarEntry> {
  let list = match password {
    Some(password) => Archive::with_password(filename, password),
    None => Archive::new(filename)
  }.list();

  let list = if let Ok(mut list) = list {
    match list.process() {
      Ok(res) => res,
      Err(error) => panic!("Problem unrar the file: {:?}", error),
    }
  } else {
    panic!("Problem unrar the file: {:?}", list)
  };

  let mut res: Vec<RarEntry> = vec![];
  for i in 0..list.len() {
    res.push(RarEntry::new(i, String::new(), &list[i]));
  }
  sort_entries(&mut res);
  res
}

pub fn sort_entries(entries: &mut Vec<RarEntry>) {
  entries.sort_by(|l, r| l.name.cmp(&r.name));
  for i in 0..entries.len() {
    entries[i].idx = i
  }
}