use crate::errors::{Error, ErrorKind, Result};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub(crate) enum AlgorithmFamily {
    Hmac,
    Rsa,
    Ec,
    Ed,
    INVALID,
}

/// The algorithms supported for signing/verifying JWTs
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Algorithm {
    /// HMAC using SHA-256
    HS256,
    /// HMAC using SHA-384
    HS384,
    /// HMAC using SHA-512
    HS512,

    /// ECDSA using SHA-256
    ES256,
    /// ECDSA using SHA-384
    ES384,

    /// RSASSA-PKCS1-v1_5 using SHA-256
    RS256,
    /// RSASSA-PKCS1-v1_5 using SHA-384
    RS384,
    /// RSASSA-PKCS1-v1_5 using SHA-512
    RS512,

    /// RSASSA-PSS using SHA-256
    PS256,
    /// RSASSA-PSS using SHA-384
    PS384,
    /// RSASSA-PSS using SHA-512
    PS512,

    /// Edwards-curve Digital Signature Algorithm (EdDSA)
    EdDSA,

    // Now JWE algorithms; see https://www.rfc-editor.org/rfc/rfc7518.html#page-12
    // see https://github.com/Keats/jsonwebtoken/issues/252
    /// RSAES-PKCS1-v1_5
    #[serde(rename = "RSA1-5")]
    RSA1_5,

    /// RSA-OAEP using default parameters
    #[serde(rename = "RSA-OAEP")]
    RSA_OAEP,

    /// RSA-OAEP using SHA-256 and MGF1 with SHA-256
    #[serde(rename = "RSA-OAEP-256")]
    RSA_OAEP_256,

    /// AES Key Wrap with default initial value using 128-bit key
    A128KW,

    /// AES Key Wrap with default initial value using 192-bit key
    A192KW,

    /// AES Key Wrap with default initial value using 256-bit key
    A256KW,

    /// Direct use of a shared symmetric key as the CEK
    Dir,

    /// Elliptic Curve Diffie-Hellman Ephemeral Static key agreement using Concat KDF
    #[serde(rename = "ECOH-ES")]
    ECDH_ES,

    /// ECDH-ES using Concat KDF and CEK wrapped with "A128KW"
    #[serde(rename = "ECOH-ES-A128KW")]
    ECDH_ES_A128KW,

    /// ECDH-ES using Concat KDF and CEK wrapped with "A192KW"
    #[serde(rename = "ECOH-ES-A192KW")]
    ECDH_ES_A192KW,

    /// ECDH-ES using Concat KDF and CEK wrapped with "A256KW"
    #[serde(rename = "ECOH-ES-A256KW")]
    ECDH_ES_A256KW,

    /// Key wrapping with AES GCM using 128-bit key
    A128GCMKW,

    /// Key wrapping with AES GCM using 192-bit key
    A192GCMKW,

    /// Key wrapping with AES GCM using 256-bit key
    A256GCMKW,

    /// PBES2 with HMAC SHA-256 and "A128KW" wrapping
    #[serde(rename = "PBES2-HS256-A128KW")]
    PBES2_HS256_A128KW,

    /// PBES2 with HMAC SHA-384 and "A192KW" wrapping
    #[serde(rename = "PBES2-HS384-A192KW")]
    PBES2_HS384_A192KW,

    /// PBES2 with HMAC SHA-512 and "A256KW" wrapping
    #[serde(rename = "PBES2-HS512-A256KW")]
    PBES2_HS512_A256KW,
}

impl Default for Algorithm {
    fn default() -> Self {
        Algorithm::HS256
    }
}

impl FromStr for Algorithm {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "HS256" => Ok(Algorithm::HS256),
            "HS384" => Ok(Algorithm::HS384),
            "HS512" => Ok(Algorithm::HS512),
            "ES256" => Ok(Algorithm::ES256),
            "ES384" => Ok(Algorithm::ES384),
            "RS256" => Ok(Algorithm::RS256),
            "RS384" => Ok(Algorithm::RS384),
            "PS256" => Ok(Algorithm::PS256),
            "PS384" => Ok(Algorithm::PS384),
            "PS512" => Ok(Algorithm::PS512),
            "RS512" => Ok(Algorithm::RS512),
            "EdDSA" => Ok(Algorithm::EdDSA),
            _ => Err(ErrorKind::InvalidAlgorithmName.into()),
        }
    }
}

impl Algorithm {
    pub(crate) fn family(self) -> AlgorithmFamily {
        match self {
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => AlgorithmFamily::Hmac,
            Algorithm::RS256
            | Algorithm::RS384
            | Algorithm::RS512
            | Algorithm::PS256
            | Algorithm::PS384
            | Algorithm::PS512 => AlgorithmFamily::Rsa,
            Algorithm::ES256 | Algorithm::ES384 => AlgorithmFamily::Ec,
            Algorithm::EdDSA => AlgorithmFamily::Ed,
            _ => AlgorithmFamily::INVALID,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_algorithm_enum_from_str() {
        assert!(Algorithm::from_str("HS256").is_ok());
        assert!(Algorithm::from_str("HS384").is_ok());
        assert!(Algorithm::from_str("HS512").is_ok());
        assert!(Algorithm::from_str("RS256").is_ok());
        assert!(Algorithm::from_str("RS384").is_ok());
        assert!(Algorithm::from_str("RS512").is_ok());
        assert!(Algorithm::from_str("PS256").is_ok());
        assert!(Algorithm::from_str("PS384").is_ok());
        assert!(Algorithm::from_str("PS512").is_ok());
        assert!(Algorithm::from_str("").is_err());
    }
}
