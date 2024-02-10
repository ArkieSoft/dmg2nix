use std::env;
use std::io;
use std::fs::File;
use std::process::Command;
//use std::fmt::Display;
use download_rs::async_download::Download;
use glob::glob;
use dmg::Attach;
use std::error::Error;
use std::io::Write;
//use std::ffi::OsString;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let dmgname = &args[1];

    let precommand: ();
    let command: ();
    
    println!("Name of package");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    name = name.trim().to_owned();

    let download = Download::new(dmgname,Some("target.dmg"),None);

    match download.download() {
    Ok(_) => println!("Downloaded"),
    Err(e) => println!("Failure"),

    }
    
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

    //println!("{:?}", _prehash);

  //  let _prehashstr = _prehash.stdout;

//    let hash = match std::str::from_utf8(&_prehash.stdout) {
//        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
//        Ok(_) => todo!(),
//    };
    println!("Hashing Complete");
  //  println!("{:?}", hash);
  //
    let content_str: String = format!(r#"{{ stdenv, undmg }}:
                 
stdenv.mkDerivation rec {{
    pname = "{0}";
    version = "{1}";
    src = pkgs.fetchurl {{
        recursiveHash = true;
        url = "{2}";
        hash = "{3:?}";
        name = "target.dmg"
    }};
                     
                     
    buildInputs = [pkgs.undmg];
                     
    unpackPhase = ''
        undmg $src
    '';
                     
    installPhase = ''
        ls
        mkdir -p #out/Applications/
        cp -r {0}.app $out/Applications/
    '';
}}"#, name, version, dmgname, _prehash);
    let content = content_str.as_bytes();
    let mut f = File::create("target.nix");
    f?.write_all(content);
    Ok(())

}

