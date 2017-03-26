extern crate mpd;

use self::mpd::{Query, Client};
use std::net::TcpStream;

use std::process::Command;

pub fn uninit() {
}

pub fn init() {
	// start mpd
	//let status = Command::new("/usr/bin/mpd /root/.local/share/cantata/mpd/mpd-rs.conf").status().unwrap_or_else(|e| {
    //	panic!("failed to execute process: {}", e)
	//});

let output = Command::new("/usr/bin/mpd")
                     .arg("/root/.local/share/cantata/mpd/mpd-rs.conf")
                     .output()
                     .expect("failed to execute process");

println!("status: {}", output.status);
println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    // println!("process exited with: {}", status);

	// connect to mpd
	let mut conn = Client::connect("127.0.0.1:9999").unwrap();
	conn.volume(100).unwrap();
	let res = conn.rescan().unwrap();
	let pls = conn.playlists();
	println!("PlayLists: {:?}", pls);
	//conn.load("My Lounge Playlist", ..).unwrap();
	//conn.play().unwrap();
	let songs = conn.songs(..).unwrap();
	println!("{:?}", songs);

	println!("Status: {:?}", conn.status());
}
