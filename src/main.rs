use std::env;
use std::io;
use std::fs::File;
//use std::io::BufRead;
use std::process::Command;
//use std::fmt::Display;
use download_rs::async_download::Download;
use glob::glob;
use dmg::Attach;
//use std::error::Error;
use std::io::Write;
//use std::iter;
//use std::ffi::OsString;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let dmgname = &args[1];

    let stdin = io::stdin();
    let mut inputs = stdin.lines();

    println!("Name of package");
    let name = inputs.next().unwrap().unwrap().trim().to_owned();

    let download = Download::new(dmgname,Some("target.dmg"),None);

    match download.download() {
    Ok(_) => println!("Downloaded"),
    Err(_e) => println!("Failure"),

    }

    println!("Version");
    let version = inputs.next().unwrap().unwrap().trim().to_owned();

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

    let hash = match std::str::from_utf8(&_prehash.stdout) {
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        Ok(v) => v,
    };

    let hashsplit:Vec<&str> = hash.split("\n").collect();
    let hashres = hashsplit.iter().nth(0).unwrap();
    let hashf = hashres.trim_matches('"').to_string();     

    println!("Hashing Complete");
    println!("{}", hashf);
  //
    let content_str: String = format!(r#"{{ stdenv, undmg, pkgs }}:
                 
stdenv.mkDerivation rec {{
    pname = "{0}";
    version = "{1}";
    src = pkgs.fetchurl {{
        recursiveHash = true;
        url = "{2}";
        hash = "sha256-{3}";
        name = "target.dmg";
    }};
                     
                     
    buildInputs = [pkgs.undmg];
                     
    unpackPhase = ''
        undmg $src
    '';
                     
    installPhase = ''
        ls
        mkdir -p $out/Applications/
        cp -r {0}.app $out/Applications/
    '';
}}"#, name, version, dmgname, hashf);
    let content = content_str.as_bytes();
    let f = File::create("target.nix");
    let _ = f?.write_all(content);
    Ok(())

    //println!("Cleaning up");
}

