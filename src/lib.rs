#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate config;
extern crate futures;
extern crate rocket;
extern crate tokio_core;
extern crate telegram_bot;
#[macro_use] extern crate diesel;
#[macro_use] extern crate indoc;
#[macro_use] extern crate rocket_contrib;


pub mod db;
pub mod bot;
pub mod server;
pub mod settings;
