pub mod executer {
    pub mod basedprocess {
        use super::BasedProcessInfo;
        use std::sync::mpsc;
        use std::{process, thread};

        pub struct BasedProcess {
            pub out: String,
            pub err: String,
        }

        impl BasedProcess {
            pub fn new(info: BasedProcessInfo) -> BasedProcess {
                let (tx, rx) = mpsc::channel();

                thread::spawn(move || {
                    let mut process = process::Command::new("sh")
                        .arg("-c")
                        .arg(info.command)
                        .args(info.args)
                        .output()
                        .expect("Failed to execute");
                    println!("Process Created");

                    tx.send(process).expect("Failed sending");
                });
                let output = rx.recv().expect("Error Reading Output from BasedProcess");
                let (_out, _err) = (
                    String::from_utf8(output.stdout).unwrap_or("nothing".into()),
                    String::from_utf8(output.stderr).unwrap_or("no err".into()),
                );

                BasedProcess {
                    out: _out,
                    err: _err,
                }
            }
        }

        impl std::fmt::Display for BasedProcess {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "stdout:{} \n\nstderr:{} \n", self.out, self.err)
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
                }

                false => {}
            };

            BasedProcessInfo {
                command: command,
                args: args,
            }
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
                }

                false => {}
            };

            BasedProcessInfo {
                command: command,
                args: args,
            }
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

pub mod security {
    use rocket::request::{FromRequest, Outcome};

    pub mod totp {
        use rand::{self, Rng};
        use std::sync::mpsc;
        use std::{
            iter::Inspect,
            time::{self, Duration, Instant},
        };

        #[derive(Debug)]
        pub struct _TOTP {
            pub value: i32,
            /// Use time.elapsed not a new value
            pub lifetime: Instant,
        }

        pub fn totp_process(wait_time_sec: i32) -> _TOTP{
            let time = time::Instant::now();
            let mut code: Vec<i32> = Vec::new();

            while time.elapsed().as_secs() <= wait_time_sec.try_into().unwrap() {}
            for i in 0..5 {
                code.push(rand::thread_rng().gen_range(0..9));
            }


            let mut code_i32: i32 = format!("{}{}{}{}{}", code[0], code[1], code[2], code[3], code[4]).parse().unwrap();
            
            if code_i32 < 10000 {
                code_i32 = format!("{}{}{}{}{}", code[0], code[1], code[2], code[3], 0).parse().unwrap();
            }

            _TOTP {lifetime: time::Instant::now(), value: code_i32}
        }
    }

    pub struct Auth {
        pub username: String,
        code: i32,
    }

    #[derive(Debug)]
    enum AuthError {
        UsernameInvalid,
        TotpInvalid,
    }

    /*impl <'a,'r>FromRequest<'a,'r> for Auth {
        type Error = AuthError;

        fn from_request(request: &'a rocket::Request<'r>) -> rocket::request::Outcome<Self, Self::Error> {
            let totp_key: Vec<&str> = request.headers().get("totp").collect();

            return Outcome::Success();
        }
    }*/
}
