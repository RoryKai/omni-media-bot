pub mod api {
    use reqwest::Client;
    use std::error::Error;

    pub async fn make_request(base_url: &str, endpoint: &str) -> Result<String, Box<dyn Error>> {
        // Create a new HTTP client
        let client = Client::new();

        // Construct the full URL
        let url = format!("{}{}", base_url, endpoint);

        // Debugging: Print the URL
        println!("Making request to URL: {}", url);

        // Make the GET request
        let response = client
            .get(&url)
            .send()
            .await?;

        // Check if the request was successful
        if response.status().is_success() {
            // Parse the response body as a string
            let body = response.text().await?;
            Ok(body)
        } else {
            Err(format!("Request failed with status: {}", response.status()).into())
        }
    }
}
