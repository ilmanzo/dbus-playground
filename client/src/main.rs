use zbus::{dbus_proxy, Connection, Result};

#[dbus_proxy(
    interface = "org.zbus.MyService",
    default_service = "org.zbus.MyService",
    default_path = "/org/zbus/MyService"
)]
trait MyService {
    async fn call_me(&self, name: &str) -> Result<String>;
}

#[tokio::main]
async fn main() -> Result<()> {
    let connection = Connection::session().await?;

    // `dbus_proxy` macro creates `MyServiceProxy` based on `Notifications` trait.
    let proxy = MyServiceProxy::new(&connection).await?;
    let reply = proxy.call_me("Andrea").await?;
    println!("{reply}");

    Ok(())
}
