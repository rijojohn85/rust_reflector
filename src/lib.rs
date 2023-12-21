use std::process::Command;

pub fn run() {
    let country = "IN";
    let url = format!("https://archlinux.org/mirrors/status/json/");
    let output = Command::new("curl")
        .arg("-s")
        .arg(&url)
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("Failed to fetch mirror list: {:?}", output.stderr);
        std::process::exit(1);
    }

    let mirrorlist = String::from_utf8_lossy(&output.stdout);

    println!("{:#?}", mirrorlist);
}
