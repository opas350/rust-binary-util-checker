use std::process::Command;
use git2::{Repository, ProxyOptions, FetchOptions};
use std::ops::Deref;
use config::Config;
use std::collections::HashMap;
use std::process;
use crate::config_parser::Configuration;
use crate::app::App;

mod config_parser;
mod app;
fn main() {
    let result = run();

    match result {
        Err(error) => {
            let stderr = std::io::stderr();
            // default_error_handler(&error, &mut stderr.lock());
        }
        Ok(false) => {
            process::exit(1);
        }
        Ok(true) => {
            process::exit(0);
        }
    }
}

fn run () -> Result<bool> {

    let app = App::new()?;
    //let path = Path::new("D:/Repos");
    let settings = config_parser::make();
    let s: Configuration = settings.try_into().unwrap();
    println!("{:?}", s);
    // settings.try_into::<HashMap<String, String>>().unwrap());
    // settings.
    let output = if cfg!(target_os = "windows") {
    } else if cfg!(target_os = "macos") {
        let repo = match Repository::open("/Users/DOU1831/Repo/bat") {
            Ok(repo) => test(repo),
            Err(e) => panic!("failed to clone: {}", e)
        };
    } else {
        Err(false);
        unimplemented!()
    };

}

// fn get_config() -> Config {
//     let mut settings = config::Config::default();
//     settings
//         .merge(config::File::with_name("Config"))
//         .unwrap()
//         .to_owned()
// }

fn test(repo: Repository) {
    let is_bare = &repo.is_bare();
    let status_new = &repo.state();
    println!("{:?}", is_bare);
    println!("{:?}", status_new);

    // let n_repo = git2::Repository::discover("bat").unwrap();
    let n_repo = fetch_origin_master(repo).unwrap();
    println!("{:?}", n_repo);
}

fn fetch_origin_master(repo: git2::Repository) -> Result<(), git2::Error> {
    let mut proxy = ProxyOptions::new();
    proxy.url("http://127.0.0.1:3128/");
    let mut fo = FetchOptions::new();
    fo.proxy_options(proxy);
    // ProxyOptions::new().auto();
    repo.find_remote("origin")?.fetch(&["master"], Option::Some(&mut fo), None)
}
