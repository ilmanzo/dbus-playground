use chrono::{DateTime, Local};
use std::{error::Error, future::pending};
use zbus::{dbus_interface, ConnectionBuilder};

struct MyService {
    call_count: u64,
    call_timestamp: Option<DateTime<Local>>,
}

#[dbus_interface(name = "org.zbus.MyService")]
impl MyService {
    // Can be `async` as well.
    fn reply(&mut self) -> String {
        let msg = match self.call_count {
            0 => "This is the first time you call me!".into(),
            _ => format!(
                "I have been called {} times, last at {}",
                self.call_count,
                self.call_timestamp
                    .expect("unable to get local time")
                    .to_rfc3339()
            ),
        };
        self.call_count += 1;
        self.call_timestamp = Some(Local::now());
        msg
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let svc = MyService {
        call_count: 0,
        call_timestamp: None,
    };
    let _conn = ConnectionBuilder::session()?
        .name("org.zbus.MyService")?
        .serve_at("/org/zbus/MyService", svc)?
        .build()
        .await?;

    // Do other things or go to wait forever
    pending::<()>().await;

    Ok(())
}
