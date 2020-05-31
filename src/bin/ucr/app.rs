use ucr::{
    error::*
};
use clap::ArgMatches;

pub struct App {
    pub matches: ArgMatches<'static>
}

impl App {
    pub fn new() {}
}