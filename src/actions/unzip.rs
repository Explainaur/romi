use std::{fs, io, path};
use serde::Serialize;
use crate::actions::decompress::{DCHandler, Decompress, Decompresser};

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, Clone, Serialize)]
pub struct ZipEntry {
  pub idx: usize,
  name: String,
  path: String,
  size: u64,
}

impl Decompress<ZipEntry> for Decompresser<ZipEntry> {
  fn decompress_all(&mut self, dc_handler: DCHandler) -> &mut Self {
    self.set_entries(unzip(dc_handler));
    self
  }

  fn get_file_list(&self, filename: String, password: Option<String>) -> Vec<ZipEntry> {
    get_files(filename, password)
  }
}

pub fn unzip(dc_handler: DCHandler) -> Vec<ZipEntry> {
  let DCHandler { filename, path, password } = dc_handler;
  let file = fs::File::open(&filename).unwrap();
  let reader = io::BufReader::new(file);
  let mut archive = zip::ZipArchive::new(reader).unwrap();

  let root_path = path::Path::new(&path);
  if !root_path.exists() {
    fs::create_dir_all(&root_path).unwrap()
  }

  let mut entries = vec![];

  for i in 0..archive.len() {

    // get files
    let mut file = match &password {
      Some(res) => {
        let res = archive.by_index_decrypt(i, res.as_bytes());
        match res.unwrap() {
          Ok(zip_file) => zip_file,
          Err(error) => panic!("Problem unzip the file {:?}", error),
        }
      }
      _ => archive.by_index(i).unwrap()
    };

    let outpath = match file.enclosed_name() {
      Some(p) => root_path.join(p),
      None => continue,
    };

    if (*file.name()).ends_with('/') {
      fs::create_dir_all(&outpath).unwrap();
    } else {
      if let Some(p) = outpath.parent() {
        if !p.exists() {
          fs::create_dir_all(&p).unwrap();
        }
      }
      let mut outfile = fs::File::create(&outpath).unwrap();
      io::copy(&mut file, &mut outfile).unwrap();


      entries.push(ZipEntry {
        idx: i,
        name: file.name().to_string(),
        path: outpath.to_str().unwrap().to_string(),
        size: file.size(),
      });
    }
  }
  sort_entries(&mut entries);
  entries
}

pub fn get_files(filename: String, password: Option<String>) -> Vec<ZipEntry> {
  let mut entries = vec![];

  let file = fs::File::open(&filename).unwrap();
  let reader = io::BufReader::new(file);
  let mut archive = zip::ZipArchive::new(reader).unwrap();

  for i in 0..archive.len() {
    let file = match &password {
      Some(res) => {
        let res = archive.by_index_decrypt(i, res.as_bytes());
        match res.unwrap() {
          Ok(zip_file) => zip_file,
          Err(error) => panic!("Problem unzip the file {:?}", error),
        }
      }
      _ => archive.by_index(i).unwrap()
    };

    let outpath = match file.enclosed_name() {
      Some(p) => p,
      None => continue,
    };

    entries.push(ZipEntry {
      idx: i,
      name: file.name().to_string(),
      path: outpath.to_str().unwrap().to_string(),
      size: file.size(),
    });
  }
  sort_entries(&mut entries);
  entries
}

pub fn sort_entries(entries: &mut Vec<ZipEntry>) {
  entries.sort_by(|l, r| l.name.cmp(&r.name));
  for i in 0..entries.len() {
    entries[i].idx = i
  }
}