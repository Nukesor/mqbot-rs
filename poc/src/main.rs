extern crate actix_web;
extern crate futures;
extern crate tokio;
extern crate tokio_io;
extern crate tokio_process;

use actix_web::{fs, server, App};

use std::io;
use std::process::{Command, Stdio};

use futures::{Future, Stream};
use tokio_process::{Child, CommandExt};

fn print_lines(mut cat: Child) -> Box<Future<Item = (), Error = ()> + Send + 'static> {
    let stdout = cat.stdout().take().unwrap();
    let reader = io::BufReader::new(stdout);
    let lines = tokio_io::io::lines(reader);
    let cycle = lines.for_each(|l| {
        println!("Line: {}", l);
        Ok(())
    });

    let future = cycle.join(cat).map(|_| ()).map_err(|e| panic!("{}", e));

    Box::new(future)
}

fn main() {
    let mut cmd = Command::new("youtube-dl");
    cmd.arg("--newline");
    cmd.arg("https://www.youtube.com/watch?v=l7EsakLakQE");
    cmd.stdout(Stdio::piped());

    let future = print_lines(cmd.spawn_async().expect("failed to spawn command"));
    tokio::run(future);

    let address = "0.0.0.0:8080";
    println!("Running webserver on {}", address);
    server::new(|| App::new().handler("/", fs::StaticFiles::new(".").show_files_listing()))
        .bind(address)
        .unwrap()
        .run();
}
