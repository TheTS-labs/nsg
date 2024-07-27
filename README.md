# Nsg

Nsg is a library for interacting with Portal. It provides read-only methods for basic search, view request, work schedule and brief request

## Usage

```rust
use nsg::Nsg;
use chrono::Utc;

#[tokio::main]
async fn main() {
    let nsg = Nsg::from_creds(
        "login".to_string(),
        "password_hash".to_string(),
        "https://net-stroy.itnet.lviv.ua".to_string(),
        "client".to_string(),
        "x.y".to_string(),
    ).await.unwrap();

    println!("Work schedule (orders) for today: {:#?}", nsg.work_schedule(Utc::now().date_naive()).await);
    println!("Brief request of 950974: {:#?}", nsg.brief_request(950974).await);
    println!("Full information about 950974: {:#?}", nsg.view_request(950974).await);
    println!("Search for 95097%: {:#?}", nsg.basic_search("95097%").await);
}
```

## Install

To use `nsg` lib install it via `cargo`:

```terminal
cargo add nsg
```
