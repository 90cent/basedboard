pub mod executer {
    pub mod basedprocess {
        use super::BasedProcessInfo;
        use std::{thread,process};
        use std::sync::mpsc;

        pub struct BasedProcess {
            pub out: String,
            pub err: String
        }
        
        impl BasedProcess {
            pub fn new(info: BasedProcessInfo) -> BasedProcess {
                let (tx,rx) = mpsc::channel();

                thread::spawn(move || {
                    let mut process = process::Command::new("sh")
                        .arg("-c")
                        .arg(info.command)
                        .args(info.args)
                        .output().expect("Failed to execute");
                    println!("Process Created");

                    tx.send(process).expect("Failed sending");
                });
                let output = rx.recv().expect("Error Reading Output from BasedProcess");
                let (_out,_err) = (String::from_utf8(output.stdout).unwrap_or("nothing".into()),
                String::from_utf8(output.stderr).unwrap_or("no err".into()));

                BasedProcess {out: _out, err: _err}
            }
        }    

        impl std::fmt::Display for BasedProcess {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f,"stdout:{} \n\nstderr:{} \n",self.out,self.err)
            }
        }
    }

    pub struct BasedProcessInfo {
        pub command: String,
        pub args: Vec<String>,
    }

    impl BasedProcessInfo {
        pub fn new(raw_command: String) -> BasedProcessInfo {
            let mut command = raw_command;
            let mut args = Vec::new();

            match command.contains('+') {
                true => {
                    for arg in command.split('+') {
                        if arg != command {
                            args.push(String::from(arg));
                        }
                    }
        
                    command.replace_range(command.find('+').unwrap().., "");
                },

                false => {},
            };

            BasedProcessInfo{ command: command, args: args }
        }

        fn from_crypted(crypt_command: String) -> BasedProcessInfo {
            let mut command = crypt_command;
            let mut args = Vec::new();

            match command.contains('+') {
                true => {
                    for arg in command.split('+') {
                        if arg != command {
                            args.push(String::from(arg));
                        }
                    }
        
                    command.replace_range(command.find('+').unwrap().., "");
                },

                false => {},
            };

            BasedProcessInfo{ command: command, args: args }
        }
    }
}


pub mod embed {
    use include_dir::*;

    static page: Dir = include_dir!("page/");


    pub fn init() {
        page.extract("page/").unwrap();
    }
}