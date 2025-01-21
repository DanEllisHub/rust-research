pub mod reqwest_example {
    use reqwest;
    pub async fn reqwest_go_1() {
        // vulnerable - 1 result
        let url = "http://eoch7sk7og4ik7f.m.pipedream.net";
        let secret = "key=$uper$ecret";
        let urlAndParams = format!("{}?{}", url, secret);
    
        let body = reqwest::get(urlAndParams)
            .await
            .unwrap()
            .text()
            .await
            .unwrap(); // RESULT - the first param of get() - `urlAndParams` is output that contains secret key
    }

    pub async fn reqwest_go_2() {
        // vulnerable - 2 results
        let client = reqwest::Client::new();
        let my_secret = "$uper$ecret";
        let body_secret = format!("key={}", my_secret);
        let res = client.get("http://eoch7sk7og4ik7f.m.pipedream.net")
            .header("Some-Custom-Header", my_secret) // RESULT #1 - Header output with secret
            .body(body_secret) // RESULT #2 - Body output with secret
            .send()
            .await
            .unwrap();
    }

    pub async fn reqwest_go_3() {
        // safe - sanitized by `https://` protocol
        let client = reqwest::Client::new();
        let my_secret = "$uper$ecret";
        let body_secret = format!("key={}", my_secret);
        let res = client.get("https://eoch7sk7og4ik7f.m.pipedream.net")
            .header("Some-Custom-Header", my_secret) // SAFE
            .body(body_secret) // SAFE
            .send()
            .await
            .unwrap();
    }

    pub async fn reqwest_go_4() {
        // safe - sanitized by `https://` protocol
        let url = "https://eoch7sk7og4ik7f.m.pipedream.net";
        let secret = "key=$uper$ecret";
        let urlAndParams = format!("{}?{}", url, secret);
    
        let body = reqwest::get(urlAndParams)
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
    }
}