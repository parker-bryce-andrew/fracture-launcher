use std::{
    env,
    fs::{File, Permissions},
    io::Write,
    os::unix::fs::PermissionsExt,
    process::Command,
};

use notify_rust::Notification;

const APP_PATH: &'static str = "/app/bin/slashshade";

// const BINARY: &[u8] = include_bytes!("../slashshade");

fn build_cmd() -> Command {
    let mut app = Command::new(&APP_PATH);
    // app.arg("--sandbox")
    // app
    //     // .arg(&APP_PATH)
    //     .args(env::args());

    app
}

fn main() {
    // {
    //     let mut f = File::create("./app").unwrap();
    //     f.write_all(BINARY).unwrap();
    //     f.set_permissions(Permissions::from_mode(755)).unwrap();
    //     f.flush().unwrap();
    // }
    'process: loop {
        let is_safe_mode = env::var("SAFE_MODE").is_ok();

        let mut app = build_cmd();

        println!("Launching: \r\n\r\n{:#?}\r\n\r\n", app);

        let err_msg: String;

        match app.status() {
            Ok(c) => match c.success() {
                true => {
                    break 'process;
                }
                false => {
                    err_msg = format!("{c:#?}");
                }
            },

            err @ Err(_) => {
                err_msg = format!("{err:#?}");
            }
        }

        if !is_safe_mode {
            let err = format!(
                "THERE WAS A PANIC. Starting again in safe mode. To always start in safe mode, set {} in the environment.{}{}\r\n",
                "\"SAFE_MODE=1\"",
                "\r\n\r\n",
                err_msg.replace("\n", "\r\n"),
            );

            let _ = Notification::new().summary("ERROR").body(&err).show();
            println!("{}", err);

            unsafe { env::set_var("SAFE_MODE", "1") };
        } else {
            break 'process;
        }
    }
}
