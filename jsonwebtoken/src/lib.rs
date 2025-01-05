wit_bindgen::generate!({ generate_all });

use data_encoding::{BASE64URL_NOPAD, BASE64_NOPAD};
use exports::bettyblocks::runtime_cloud::jsonwebtoken::Guest;
use exports::bettyblocks::runtime_cloud::jsonwebtoken::Jwt;
// use exports::bettyblocks::runtime_cloud::jsonwebtoken::JWT;
use hmac::{Hmac, Mac};
use sha2::Sha256;

struct JsonWebToken;

fn verify_hs256_signature(
    secret: &str,
    header: &str,
    payload: &str,
    signature: &str,
) -> Result<bool, String> {
    // // Create HMAC-SHA256 instance
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .map_err(|e| format!("Invalid key: {:?}", e))?;

    // Concatenate header and payload
    let message = format!("{}.{}", header, payload);

    // Input the message into the HMAC instance
    mac.update(message.as_bytes());

    // Get the resulting HMAC code
    let result = mac.finalize();
    let code_bytes = result.into_bytes();

    // Base64 URL encode the HMAC code
    let expected_signature = format!("{}", BASE64URL_NOPAD.encode(&code_bytes));

    if expected_signature == signature {
        return Ok(true);
    } else {
        return Err("Signature is invalid".to_string());
    }
}

impl Guest for JsonWebToken {
    fn decode(encoded: String, secret: Option<String>) -> Result<Jwt, String> {
        let parts: Vec<&str> = encoded.split('.').collect();

        if parts.len() != 3 {
            return Err("Invalid JWT format".to_string());
        }

        let header = match BASE64_NOPAD.decode(parts[0].as_bytes()) {
            Ok(decoded) => decoded,
            Err(e) => return Err(format!("Failed to decode header: {:?}", e)),
        };
        let payload = match BASE64_NOPAD.decode(parts[1].as_bytes()) {
            Ok(decoded) => decoded,
            Err(e) => return Err(format!("Failed to decode payload: {:?}", e)),
        };

        let signature = parts[2]; // Signature is not base64 decoded

        let header_str = String::from_utf8(header).expect("Invalid UTF-8 sequence in header");
        let payload_str = String::from_utf8(payload).expect("Invalid UTF-8 sequence in payload");

        //use the secret if provided, otherwise use an empty string
        let _secret = secret.unwrap_or("".to_string());

        let verified = verify_hs256_signature(&_secret, &header_str, &payload_str, signature);

        Ok(Jwt {
            header: header_str,
            payload: payload_str,
            signature: signature.to_string(),
            verified: verified.is_ok(),
        })
    }
}

export!(JsonWebToken);

mod tests {
    use super::*;

    #[test]
    fn test_jwt_decoding_success_with_verified_signature() {
        let encoded = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let result =
            JsonWebToken::decode(encoded.to_string(), Some("your-256-bit-secret".to_string()));
        assert!(result.is_ok());
    }

    #[test]
    fn test_jwt_decoding_success_with_verified_hs512_signature() {
        let encoded = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let result =
            JsonWebToken::decode(encoded.to_string(), Some("your-512-bit-secret".to_string()));
        assert_eq!(result.unwrap().verified, true);
    }

    #[test]
    fn test_jwt_decoding_failure_with_unverified_signature_and_empty_string() {
        let encoded = "";
        let result = JsonWebToken::decode(encoded.to_string(), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_jwt_decoding_failure_with_unverified_signature_and_three_part_jwt() {
        let encoded = "a.b.c";
        let result = JsonWebToken::decode(encoded.to_string(), None);
        assert!(result.is_err());
    }

    #[test]
    fn test_signature_verification() {
        let secret = "your-256-bit-secret";
        let header = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
        let payload = "eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ";
        let signature = "SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let result = verify_hs256_signature(secret, header, payload, signature);
        assert!(result.is_ok());
    }

    #[test]
    fn test_signature_fails_verification() {
        let secret = "your-256-bit-secret";
        //removed first 'e' char from header
        let header = "yJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
        let payload = "eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ";
        let signature = "SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let result = verify_hs256_signature(secret, header, payload, signature);
        assert!(result.is_err());
    }

    #[test]
    fn test_a_bb_jwt_token_without_secret() {
        let token = "eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJhcHBfdXVpZCI6IjY5M2IyMmU5ODNmYjQ2YWZhNGViMzUzZDgyZWNlNGJiIiwiYXVkIjoiSm9rZW4iLCJhdXRoX3Byb2ZpbGUiOiI1YmY5ZWJhMzQ2MzY0OTVkODBlZDVhNzkwY2EzOTA3NyIsImNhc190b2tlbiI6ImQ2NTI1ODU5NjRlY2ZkNTliZDczOGJiMzNmNWE0MjFjZTg1YzQ5M2UiLCJleHAiOjE3MzYwODA1NDIsImlhdCI6MTczNjA3MzM0MiwiaXNzIjoiSm9rZW4iLCJqdGkiOiIzMGJzYjBkb2lidHUycHJ0NTAwMDcwZDMiLCJsb2NhbGUiOm51bGwsIm5iZiI6MTczNjA3MzM0Miwicm9sZXMiOlsxXSwidXNlcl9pZCI6MX0.qnlhrIcCbVSf5szKBJSnjsVLz_b8Cem-Bfwe6u-_921UYS9qRGJGpsZ9Sr7aAGz1NwC78eXT0GTuAz4fL28k_A";
        let result = JsonWebToken::decode(token.to_string(), None);
        let jwt = result.unwrap();
        assert_eq!(jwt.header, "{\"alg\":\"HS512\",\"typ\":\"JWT\"}");
        assert_eq!(jwt.payload, "{\"app_uuid\":\"693b22e983fb46afa4eb353d82ece4bb\",\"aud\":\"Joken\",\"auth_profile\":\"5bf9eba34636495d80ed5a790ca39077\",\"cas_token\":\"d652585964ecfd59bd738bb33f5a421ce85c493e\",\"exp\":1736080542,\"iat\":1736073342,\"iss\":\"Joken\",\"jti\":\"30bsb0doibtu2prt500070d3\",\"locale\":null,\"nbf\":1736073342,\"roles\":[1],\"user_id\":1}");
        assert_eq!(jwt.signature, "qnlhrIcCbVSf5szKBJSnjsVLz_b8Cem-Bfwe6u-_921UYS9qRGJGpsZ9Sr7aAGz1NwC78eXT0GTuAz4fL28k_A");
        assert_eq!(jwt.verified, false);
    }

    #[test]
    fn test_a_bb_fusion_auth_example_token_without_secret() {
        let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJleHAiOjE0ODUxNDA5ODQsImlhdCI6MTQ4NTEzNzM4NCwiaXNzIjoiYWNtZS5jb20iLCJzdWIiOiIyOWFjMGMxOC0wYjRhLTQyY2YtODJmYy0wM2Q1NzAzMThhMWQiLCJhcHBsaWNhdGlvbklkIjoiNzkxMDM3MzQtOTdhYi00ZDFhLWFmMzctZTAwNmQwNWQyOTUyIiwicm9sZXMiOltdfQ.Mp0Pcwsz5VECK11Kf2ZZNF_SMKu5CgBeLN9ZOP04kZo";
        let result = JsonWebToken::decode(token.to_string(), None);
        let jwt = result.unwrap();
        assert_eq!(jwt.header, "{\"alg\":\"HS256\",\"typ\":\"JWT\"}");
        assert_eq!(jwt.payload, "{\"exp\":1485140984,\"iat\":1485137384,\"iss\":\"acme.com\",\"sub\":\"29ac0c18-0b4a-42cf-82fc-03d570318a1d\",\"applicationId\":\"79103734-97ab-4d1a-af37-e006d05d2952\",\"roles\":[]}");
        assert_eq!(jwt.signature, "Mp0Pcwsz5VECK11Kf2ZZNF_SMKu5CgBeLN9ZOP04kZo");
        assert_eq!(jwt.verified, false);
    }
}
