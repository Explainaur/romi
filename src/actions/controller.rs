use std::fs;
use std::os::unix::fs::PermissionsExt;
use home;
use crate::actions::decompress;
use crate::actions::decompress::{Decompress, Decompresser};


pub struct Controller<T> {
  filename: String,
  workspace: std::path::PathBuf,
  decompresser: Decompresser<T>,
}

impl<T> Controller<T> {
  pub fn new(filename: Option<String>) -> Controller<T> {
    let dir = home::home_dir().unwrap().join(".romi/");
    if !dir.is_dir() {
      fs::create_dir_all(&dir).unwrap();
    }
    let metadata = dir.metadata().unwrap();
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o755);

    Controller::<T> {
      filename: match filename {
        Some(file) => file,
        None => String::new(),
      },
      workspace: dir.to_path_buf(),
      decompresser: Decompresser::<T>::new(),
    }
  }

  pub fn get_workspace(&self) -> &std::path::PathBuf {
    &self.workspace
  }

  pub fn get_filename(&self) -> &String {
    &self.filename
  }

  pub fn entries(&self) -> &Vec<T> {
    self.decompresser.entries()
  }
}

impl<T> Decompress<T> for Controller<T>
  where Decompresser<T>: Decompress<T> {
  fn decompress_all(&mut self, dc_handler: decompress::DCHandler) -> &mut Self {
    let decompresser = &mut self.decompresser;
    <Decompresser<T> as Decompress<T>>::decompress_all(decompresser, dc_handler);
    self
  }


  fn get_file_list(&self, filename: String, password: Option<String>) -> Vec<T> {
    self.decompresser.get_file_list(filename, password)
  }
}
