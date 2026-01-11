use apodex::client::reqwest::ReqwestClient;
use apodex::client::ApodClient;
use apodex::date::ApodDate;

#[tokio::main]
async fn main() {
    let client = ReqwestClient::default();

    let date = ApodDate::today();
    let page = client.fetch_page(date).await.unwrap();
    println!("{}", page.unwrap());
}
