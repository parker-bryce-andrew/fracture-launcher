use ashpd::desktop::notification::{NotificationProxy, Priority};
use std::{env, process::Command};

const APP_PATH: &'static str = "/app/bin/fracture";
const FLATPAK_ID: &str = "systems.fracture.launcher";

pub async fn send_ash_notifcation(title: &str, msg: &str) -> ashpd::Result<()> {
    let proxy = NotificationProxy::new().await?;

    proxy
        .add_notification(
            &FLATPAK_ID,
            ashpd::desktop::notification::Notification::new(&title)
                .body(msg)
                .priority(Priority::Urgent),
        )
        .await?;

    Ok(())
}

pub fn send_notifcation(title: &str, msg: &str) -> ashpd::Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build();

    if let Ok(rt) = rt {
        rt.block_on(async { send_ash_notifcation(&title, &msg).await })
    } else {
        Err(ashpd::Error::IO(rt.unwrap_err()))
    }
}

fn build_cmd() -> Command {
    let app = Command::new(&APP_PATH);

    app
}

fn main() {
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

            let _ = send_notifcation("ERROR", &err);
            println!("{}", err);

            unsafe { env::set_var("SAFE_MODE", "1") };
        } else {
            break 'process;
        }
    }
}
