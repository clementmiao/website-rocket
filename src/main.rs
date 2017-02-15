#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::io;
use std::path::{Path, PathBuf};

use rocket::config::{Config, Environment};
use rocket::response::NamedFile;

use std::process::Command;
use std::net::SocketAddrV4;
use std::net::Ipv4Addr;
use std::str;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    let host_port = 8080;

    let hostname_cmd = Command::new("hostname").arg("-I").output();

    let host_addr: SocketAddrV4 = match hostname_cmd {
        Ok(res) => {                
            let addr = str::from_utf8(res.stdout.as_slice())
                .map_err(|err| err.to_string())
                .and_then(|ip_str| ip_str.trim()
                    .parse::<Ipv4Addr>()
                    .map_err(|err| err.to_string()))
                .map(|ip| SocketAddrV4::new(ip, host_port));

            match addr {
                Ok(addr) => addr,
                Err(_) => {
                    let ip = Ipv4Addr::new(127, 0, 0, 1);
                    SocketAddrV4::new(ip, host_port)
                }
            }
        },
        Err(_) => {
            let ip = Ipv4Addr::new(127, 0, 0, 1);
            SocketAddrV4::new(ip, host_port)
        }
    };

    let config = Config::build(Environment::Development)
        .address(format!("{}", host_addr.ip()))
        .port(host_addr.port())
        .finalize()
        .unwrap();

    let app = rocket::custom(config, false);

    app.mount("/", routes![index, files]).launch();
}