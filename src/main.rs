use std::env;
use std::io;
use std::fs::File;
use std::process::Command;
//use std::fmt::Display;
use download_rs::async_download::Download;
use glob::glob;
use dmg::Attach;
use std::error::Error;
//use std::ffi::OsString;
mod write;
use write::write;

fn main() {
    let args: Vec<String> = env::args().collect();

    let dmgname = &args[1];

    let precommand: ();
    let command: ();
    
    println!("Name of package");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");


    let download = Download::new(dmgname,Some("target.dmg"),None);

    match download.download() {
        Ok(_) => println!("Downloaded!"),
        Err(e) => println!("Failure"),
    };

    println!("Version");
    let mut version = String::new();

    io::stdin()
        .read_line(&mut version)
        .expect("Failed to read line");

    println!("Moumnting .dmg Image");


    let info = Attach::new("target.dmg").with().expect("could not attach");
    println!("Mounted at {:?}", info.mount_point);


    let path = format!("/Volumes/*/*.app");
    let binding = glob(&path).unwrap().next().unwrap().unwrap();
    let appname = binding.display().to_string();

    let appnamesplit:Vec<&str> = appname.split('/').collect();
    let last = appnamesplit.last().unwrap();
    println!("Found .app Directory");
    println!("{:?}", last);

    println!("Hashing");
    let _prehash = Command::new("nix")
        .arg("hash")
        .arg("file")
        .arg("--type")
        .arg("sha256")
        .arg("--base64")
        .arg("target.dmg")
        .output()
        .expect("Failed Command");

  //  let _prehashstr = _prehash.stdout;

    let hash = match std::str::from_utf8(&_prehash.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    println!("Hashing Complete");
    println!("{}", hash);

    let nixname = ("{}-darwin.nix", name.clone());

    write();


}

