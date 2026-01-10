use apodex::chrono::NaiveDate;
use apodex::client::reqwest::ReqwestClient;
use apodex::client::ApodClient;

#[tokio::main]
async fn main() {
    let client = ReqwestClient::new();

    let date = NaiveDate::from_ymd_opt(1996, 1, 1).unwrap();
    let page = client.fetch_page(date).await.unwrap();
    println!("{}", page.unwrap());
}
