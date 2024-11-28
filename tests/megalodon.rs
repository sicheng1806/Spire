//! Test Megalodon's interface

#[tokio::test]
async fn mastodon_request1() {
    let client = megalodon::generator(
        megalodon::SNS::Mastodon,
        String::from("https://fedibird.com"),
        None,
        None,
    )
    .expect("fail to generate!");
    let res = client.get_instance().await.expect("fail to get instance!");
    println!("{:?}", res.json());
}

#[tokio::test]
#[ignore = "no token"]
async fn mastodon_request2() {
    let client = megalodon::generator(
        megalodon::SNS::Mastodon,
        String::from("https://mastoland.shop"),
        Some(String::from("your access token")),
        None,
    )
    .unwrap();
    let res = client
        .verify_account_credentials()
        .await
        .expect("fail to verify!");
    println!("{:#?}", res.json());
}
