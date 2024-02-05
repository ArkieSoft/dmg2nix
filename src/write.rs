use std::fs::File;
use std::io::Write;
use super::main;

pub fn write() -> std::io::Result<()> {

    println!("New File");
    let nixname = super::main::nixname;
    let dmgname = main.dmgname;
    let hash = main.hash;
    let version = main.version;

    println!("{}", nixname);
    println!("{}", dmgname);
    println!("{}", hash);
    println!("{}", version);

    let mut f = File::create("foo.txt")?;
    f.write_all(&1234_u32.to_be_bytes())?;
    Ok(())
}
