extern crate base64;
use std::collections::HashMap;
pub fn rm_first_last_char(string: &str) -> String{
    let mut chars = string.chars();
    chars.next();
    chars.next_back();
    return chars.as_str().to_string();
}

pub 
#[derive(Debug)]
pub struct Pkce_Auth {
    token: String,
    expire_time: String,
}
impl Pkce_Auth {

    async fn request_authentication(client_id: String, client_secret: String) -> Option<serde_json::Value> {
        let client = reqwest::Client::new();
        let authstring = "Basic ".to_owned() + &(base64::encode([client_id, client_secret].join(":")));
   
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", reqwest::header::HeaderValue::from_static("application/x-www-form-urlencoded"));
        headers.insert("Authorization", reqwest::header::HeaderValue::from_str(&authstring).unwrap());
    

    let res = client
        .post("https://accounts.spotify.com/api/token")
        .headers(headers)
        .form(&HashMap::from([("grant_type", "client_credentials")]))
        .send()
        .await
        .unwrap()
        .text()
        .await;

    match res {
        Ok(response) => {return Some(serde_json::from_str(&response).unwrap())}
        Err(_code) => {return None}
    }
}


    pub async fn new(client_id:String, client_secret:String) -> Self {
        let json = request_authentication(client_id, client_secret).await.unwrap();
        //intentionally panics if client_id string fails
        return Pkce_Auth {
            token: rm_first_last_char(&json["access_token"].to_string()),
            expire_time: json["expires_in"].to_string(),
        }
    }
}
