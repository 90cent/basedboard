pub mod messageboxtemplate {

use std::{time,thread};
use std::hash::{Hash,Hasher};
use std::collections::hash_map::DefaultHasher;


use rand::{thread_rng, Rng};
use log::*;

    #[derive(Debug)]
    #[derive(Clone)]
    pub struct MessageBox {
        pub content: String,
        pub id: u32,
        pub instant: std::time::Instant,
        pub state: BoxState
    }

    #[derive(Debug)]
    #[derive(Clone)]
    pub enum BoxState {
        NEW = 0,
        OLD = 1,
        NEW_SIGNED = 2,
        OLD_SIGNED = 3
    }

    impl MessageBox {
        pub fn new(_content: String,_id: u32) -> MessageBox {
            info!("Creating new MessageBox");


            MessageBox {
                content: _content,
                id: _id,
                instant: std::time::Instant::now(),
                state: BoxState::NEW
            }
        }

        pub fn to_json(&self) -> String {
            todo!()
        }

        pub fn to_html(&self) -> String {
            let element_start = String::from("<li><div>");
            let element_end = String::from("</div></li>");

            let element_infobar = format!("<div class=\"infobar\"><h3 id=\"status\">{:#?}</h2><small id=\"id\">{}</small><button id=\"sign-button\">SIGN</button></div>",&self.state,&self.id);

            format!("{}{}<p id=\"content\">{}</p>{}",element_start,element_infobar,&self.content,element_end)
        }
    }
}



pub mod chatverse {
    pub fn init(_message_box: super::messageboxtemplate::MessageBox) {
        
    }
}