#![feature(decl_macro)]
use std::collections::HashMap;
use std::{fmt::format,thread,process,time};
use std::sync::mpsc::{self,Receiver,Sender};
use std::{io,fs,path};

#[macro_use] extern crate rocket;
use rocket::{Rocket,response,request::FromRequest};
use rocket::http;
use rocket_contrib::templates::Template;
use colored::*;

pub mod utils;
use utils::{executer,embed,security};


#[get("/execute/<command>")]
fn execute(command: String) -> io::Result<String> {
    let process_info = executer::BasedProcessInfo::new(command);
    let process = executer::basedprocess::BasedProcess::new(process_info);

    
    let output = format!("{}",&process);
    println!("{output}");
    Ok(output)
}


#[get("/")]
fn index() -> Template {
    let mut context:HashMap<String,String> = HashMap::new();
    Template::render("index", &context)
}

fn main() {
    println!("BasedBoard Dashboard -- Based");

   /* let arguments: Vec<String> = std::env::args().collect();

        if !is_debug {
            if arguments.to_owned().len() > 1 {
                let path = path::Path::new(&arguments[1]);
    
                if !path.exists() {
                    invalid_arguments();
                    return;
                }          
                
                //std::env::set_var("PAGE_PATH", path);
            }
            else {
                invalid_arguments();
            }
        }
    */


    //embed::init();

    
    let r = thread::spawn(|| {
        let r = rocket::ignite()
        .mount("/",rocket_contrib::serve::StaticFiles::from("page"))
        .mount("/api",rocket::routes![execute,index])
        .attach(Template::fairing())
        .launch();
        r
    });


    let mut totp_vec: HashMap<i32,security::totp::_TOTP> = HashMap::new();
    let mut first_time = true;

    loop {
        let totp = if first_time {
            first_time = false;
            security::totp::totp_process(0)
        }
        else {
            security::totp::totp_process(25)
        };

        println!("New OTP:\nCode:{:}", totp.value);

        let mut old_key = 0;
        for (key,totp) in &totp_vec {
            let is_key_old = if totp.lifetime.elapsed().as_secs() > 30 {
                println!("Key: {} is to old bruh",key);
                (true,key)
            }
            else
            {
                (false,key)
            };

            if is_key_old.0 {
                old_key = *is_key_old.1;
            } 
        };

        if old_key > 0 {
            totp_vec.remove(&old_key);
            println!("Key: {} was removed from chain",&old_key);
        }

        totp_vec.insert(totp.value, totp);
        thread::sleep(time::Duration::from_millis(10));
    }
}


fn invalid_arguments() {
    println!("Please define a right Dashboard HTML page path.\n>Example: (Your Executable) -> {:?} {} <- Your Dashboard Page Path\n\nWarning: {}",std::env::current_exe().unwrap(),"/var/www/basedboard/".blue(),
    "The Backend will run without it but its not recomendended to do so please quit the programm now".on_bright_magenta());

    println!("Waiting 5 seconds. Its Recomended to set a path to the page");
    thread::sleep(time::Duration::from_secs(5));
    return;
}
