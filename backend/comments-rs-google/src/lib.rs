use std::future::Future;

use comments_rs_core_backend::{error::Error, traits::{SignupProvider, SignupResult}};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Value;

struct GoogleSignupProvider {
    pub_keys: Vec<(String, String)>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Token {
    email: String,
    picture: String,
}

impl SignupProvider for GoogleSignupProvider {
    fn name(&self) -> &'static str {
        "google"
    }

    fn signup(&self, email: &str) -> SignupResult {
        todo!()
    }

    fn confirm(&self, hash: &str, user_name: &str) -> SignupResult {
        todo!()
    }
}

impl GoogleSignupProvider {
    async fn new(pub_keys: Vec<(String, String)>) -> Self {
        Self { pub_keys }
    }

    async fn verify(&self, token: &str) -> Result<Token, Error> {
        let mut results = self
            .pub_keys
            .iter()
            .map(|key| DecodingKey::from_rsa_components(&key.0, &key.1))
            .map(|key| decode::<Token>(token, &key, &Validation::new(Algorithm::RS256)));

        let result = results.clone().find(|token| token.is_ok());

        if result.is_none() {
            Err(Error::SignatureError(
                results
                    .map(|r| {
                        if let Err(e) = r {
                            format!("{}", e)
                        } else {
                            panic!("No valid token found yet no error either")
                        }
                    })
                    .collect(),
            ))
        } else {
            Ok(result.unwrap().unwrap().claims)
        }
    }
}

async fn retrieve_keys() -> Result<Vec<(String, String)>, Error> {
    let json = reqwest::get("https://www.googleapis.com/oauth2/v3/certs")
        .await
        .expect("Could not retrieve google public keys")
        .text()
        .await
        .expect("Could not retrieve google public keys");

    let json_value: Value = serde_json::from_str(json.as_str())
        .expect("Json Web Keys could not be retrieved from json");

    let pub_keys = vec![
        (
            json_value["keys"][0]["n"].as_str().unwrap().to_string(),
            json_value["keys"][0]["e"].as_str().unwrap().to_string(),
        ),
        (
            json_value["keys"][1]["n"].as_str().unwrap().to_string(),
            json_value["keys"][1]["e"].as_str().unwrap().to_string(),
        ),
    ];

    Ok(pub_keys)
}

fn map_err(error: reqwest::Error) -> Error {
    todo!()
}

#[cfg(test)]
mod tests {

    use comments_rs_core_backend::error::Error;

    use crate::{retrieve_keys, GoogleSignupProvider};

    #[tokio::test]
    async fn test_retrieve_keys() {
        let pub_keys = retrieve_keys().await.unwrap();

        assert_eq!(pub_keys.len(), 2);
    }

    #[tokio::test]
    async fn test_validate_signature() {
        // TODO 
        // let pub_keys = retrieve_keys().await.unwrap();
        // let provider = GoogleSignupProvider {
        //     pub_keys: pub_keys.clone(),
        // };

        // let result = provider.verify(token).await;

        // assert_eq!(
        //     result,
        //     Err(Error::SignatureError(vec![
        //         "InvalidSignature".to_string(),
        //         "ExpiredSignature".to_string()
        //     ]))
        // );
    }
}
