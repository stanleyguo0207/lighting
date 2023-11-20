const REQUEST_USER_AGENT : &'static str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = reqwest::Client::new();

  let request = client
    .post("https://kyfw.12306.cn/passport/web/create-qr64")
    .header(
      reqwest::header::USER_AGENT,
      reqwest::header::HeaderValue::from_static(REQUEST_USER_AGENT),
    )
    .form(&[("appid", "otn")]);

  let response = request.send().await?;
  let response_json: serde_json::Value = response.json().await?;
  println!("{:#?}", response_json);

  Ok(())
}
