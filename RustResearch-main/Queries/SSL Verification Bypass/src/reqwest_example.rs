pub mod reqwest_example {
    use reqwest;
    pub async fn reqwest_go_1() {
        let client = reqwest::Client::builder()
        .danger_accept_invalid_hostnames(true) // RESULT
        .build()
        .unwrap();

        let s = client.get("https://wrong.host.badssl.com/")
            .send().await.expect("failed");

        // print response
        println!("{:?}", s.text().await.expect("failed 2"));
    }
    pub async fn reqwest_go_2() {
        let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true) // RESULT
        .build()
        .unwrap();
        let k : String = "aaaa";
        let s = client.get(k)
            .send().await.expect("failed");

        // print response
        println!("{:?}", s.text().await.expect("failed 2"));
    }
}