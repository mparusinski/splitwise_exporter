#[tokio::main]
async fn main() {
    let client = splitwise::client::Client::default();
    let user = client.users().get_current_user().await.unwrap();
    println!("Current user {:#?}", user)
}
