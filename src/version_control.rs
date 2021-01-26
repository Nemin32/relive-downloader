extern crate curl;
extern crate serde;
extern crate serde_json;

use curl::easy::Easy;
use std::io::{Write, stdout};

fn get_current_version() -> String {
  return String::from("1.1");
}

pub fn get_remote_version(tx: glib::Sender<String>) -> () {
  let mut curl: Easy = Easy::new();
  curl.url("https://ci.appveyor.com/api/projects/paulsapps/alive-reversing/history?recordsNumber=5&branch=master").unwrap();

  curl.progress(true).unwrap();

  let mut curl = curl.transfer();

  curl.progress_function(|dltotal, dlnow, _, _| {
    write!(stdout(), "{} / {}\n", dlnow, dltotal).unwrap();

    tx.send(format!("{} / {}\n", dlnow, dltotal));

    true
  }).unwrap();

  curl.write_function(|data| {
    //stdout().write_all(data).unwrap();

    let json: serde_json::Value = serde_json::from_slice(data).unwrap();
    let builds = json.get("builds").unwrap().as_array().unwrap();

    for elem in builds {
      if let Some(id) = elem.get("buildId") {
        println!("{}", id);
      } else {
        println!("No id");
      }
    }

    /*for elem in val.get("builds").unwrap().as_array().unwrap() {
      println!("{}", elem.as_object().unwrap().get("buildId").unwrap().as_str().unwrap());
    }*/

    Ok(data.len())
  }).unwrap();
  curl.perform().unwrap();
}

pub fn download_newest(tx: glib::Sender<(String, f64)>) {
  let mut curl: Easy = Easy::new();
  curl.url("https://ci.appveyor.com/api/projects/paulsapps/alive-reversing/artifacts/build/RELIVE_Binaries_Lite_Debug_x86.zip?branch=master&job=Platform%3A%20x86&pr=false").unwrap();

  curl.follow_location(true).unwrap();
  curl.progress(true).unwrap();

  let mut curl = curl.transfer();

  curl.progress_function(|dltotal, dlnow, _, _| {
    write!(stdout(), "{} / {}\n", dlnow, dltotal).unwrap();


    let frac = if dltotal == 0.0 {0.0} else {dlnow / dltotal};
    tx.send((format!("{} / {}\n", dlnow, dltotal), frac)).unwrap();

    true
  }).unwrap();

  curl.write_function(|data| {
    //stdout().write_all(data).unwrap();

    let mut file = std::fs::File::create("relive.zip").unwrap();
    file.write_all(data).unwrap();

    /*for elem in val.get("builds").unwrap().as_array().unwrap() {
      println!("{}", elem.as_object().unwrap().get("buildId").unwrap().as_str().unwrap());
    }*/

    Ok(data.len())
  }).unwrap();
  curl.perform().unwrap();
}