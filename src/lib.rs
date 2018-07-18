#![feature(plugin)]
extern crate actix;
extern crate actix_web;
extern crate config;
extern crate futures;
extern crate r2d2;
extern crate telegram_bot;
extern crate tokio_core;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate indoc;

pub mod bot;
pub mod db;
pub mod server;
pub mod settings;
