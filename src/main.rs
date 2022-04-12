#![feature(decl_macro)]
use std::collections::HashMap;
use std::{fmt::format,thread,process,time};
use std::sync::mpsc::{self,Receiver,Sender};
use std::{io,fs,path,hash};

#[macro_use] extern crate rocket;

use colored::*;

pub mod utils;
use utils::{executer,embed};

pub mod site;


fn main() {
    println!("3x100 Project starting...");


    let weh = site::utils::messageboxtemplate::MessageBox::new("weh".into(),2);

    println!("Testing MessageBox");
    println!("{:#?}",weh);
    println!("HTML: {}",weh.to_html());


    thread::sleep(time::Duration::from_secs(3));
    site::launch();    
}


fn invalid_arguments() {
    println!("Please define a right Dashboard HTML page path.\n>Example: (Your Executable) -> {:?} {} <- Your Dashboard Page Path\n\nWarning: {}",std::env::current_exe().unwrap(),"/var/www/basedboard/".blue(),
    "The Backend will run without it but its not recomendended to do so please quit the programm now".on_bright_magenta());

    println!("Waiting 5 seconds. Its Recomended to set a path to the page");
    thread::sleep(time::Duration::from_secs(5));
    return;
}
