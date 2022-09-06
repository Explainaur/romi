pub mod decompress;
pub mod controller;
pub mod unrar;
pub mod unzip;

use crate::actions::decompress::{DCHandler, Decompress};
use crate::actions::{unzip::ZipEntry, controller::Controller, unrar::RarEntry};

enum ACTION {
  ZIP,
  RAR,
}

fn choose_action(file: &str) -> Result<ACTION, String> {
  if file.ends_with(".cbz") {
    return Ok(ACTION::ZIP);
  } else if file.ends_with(".cbr") {
    return Ok(ACTION::RAR);
  } else {
    return Err(format!("Unknown file format: {:?}", file));
  }
}

struct RomiController {
  file: String,
  action: ACTION,
  zip_con: Option<Controller<ZipEntry>>,
  rar_con: Option<Controller<RarEntry>>,
}

impl RomiController {
  pub fn new(file: &String) -> RomiController {
    match choose_action(&file) {
      Ok(ACTION::ZIP) => RomiController {
        file: file.clone(),
        action: ACTION::ZIP,
        zip_con: Some(Controller::<ZipEntry>::new(Some(file.clone()))),
        rar_con: None,
      },
      Ok(ACTION::RAR) => RomiController {
        file: file.clone(),
        action: ACTION::RAR,
        zip_con: None,
        rar_con: Some(Controller::<RarEntry>::new(Some(file.clone()))),
      },
      Err(error) => panic!("{}", error),
    }
  }

  pub fn parse(&mut self, password: Option<String>) -> String {
    let dc_handler = match self.action {
      ACTION::ZIP => {
        let zip_con = self.zip_con.as_mut().unwrap();
        DCHandler {
          filename: self.file.clone(),
          path: String::from(zip_con.get_workspace().to_str().unwrap()),
          password,
        }
      }
      ACTION::RAR => {
        let rar_con = self.rar_con.as_mut().unwrap();
        DCHandler {
          filename: self.file.clone(),
          path: String::from(rar_con.get_workspace().to_str().unwrap()),
          password,
        }
      }
    };

    match self.action {
      ACTION::ZIP => {
        let zip_con = self.zip_con.as_mut().unwrap();
        let entries = zip_con.decompress_all(dc_handler).entries();
        serde_json::to_string(&entries).unwrap()
      }
      ACTION::RAR => {
        let rar_con = self.rar_con.as_mut().unwrap();
        let entries = rar_con.decompress_all(dc_handler).entries();
        serde_json::to_string(&entries).unwrap()
      }
    }
  }
}

#[cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]
#[tauri::command]
pub fn parse() -> String {
  let file = String::from("/Users/dyf/code/solo/project/romi/test/hellbound.cbr");

  let mut romi_controller = RomiController::new(&file);

  let res = romi_controller.parse(None);
  println!("{:?}", res);
  res
}