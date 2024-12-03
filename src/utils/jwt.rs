use core::panic;
use std::{env, time::{SystemTime, UNIX_EPOCH}};


use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, errors::Error};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,         // Optional. Audience
    exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize,          // Optional. Issued at (as UTC timestamp)
    iss: String,         // Optional. Issuer
    nbf: usize,          // Optional. Not Before (as UTC timestamp)
    sub: String,         // Optional. Subject (whom token refers to)
}

enum Audience{
    User,
    Admin
}

impl Audience {
    pub fn to_string(self) -> String {
        match self {
            Audience::User => "User".to_string(),
            Audience::Admin => "Admin".to_string(),
        }
    }
}


impl Claims{
    pub fn new(audience: Audience, exp_as_min: usize, user_id: String) -> Claims{
       
        let now: usize = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => duration.as_secs() as usize,
            Err(_) => 0,
        };
        let exp = now + (exp_as_min * 3600);
        let iss = "system".to_string();
        Claims { aud:  audience.to_string(),
         exp: now,
          iat: exp,
           iss: iss,
            nbf: now,
             sub: user_id 
        }
    }
}


pub struct JwtUtil;
impl JwtUtil{
    pub fn issue_new(user_id: String, audience: Audience)->Result<String, jsonwebtoken::errors::Error> {
        let exp_as_min = 60;
        let claims = Claims::new(audience, exp_as_min, user_id);
        let token_secret = env::var("JWTSECRET").unwrap_or_else(|_| {
            panic!("Environment variable 'JWTSECRET' not found in .env");
        });
        let encoding_key = &EncodingKey::from_secret(token_secret.as_ref());
       
        let token = match encode(&Header::default(), &claims, encoding_key) {
            Ok(encoded_token) => encoded_token, // If successful, assign the token
            Err(e) => {
                eprintln!("Error encoding token: {:?}", e); // Print the error to stderr
                return Err(e); // Return the error wrapped in a Box
            }
        }; 
        Ok(token)
    }
} 