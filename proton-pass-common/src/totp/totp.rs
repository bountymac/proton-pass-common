use crate::totp::algorithm::Algorithm;
use crate::totp::error::TOTPError;
use crate::totp::queries::Queries;
use crate::totp::sanitizer::sanitize_secret;
use url::Url;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct TOTP {
    pub label: Option<String>,
    pub secret: String,
    pub issuer: Option<String>,
    pub algorithm: Option<Algorithm>,
    pub digits: Option<u8>,
    pub period: Option<u16>,
}

pub struct TotpTokenResult {
    pub totp: TOTP,
    pub token: String,
    pub timestamp: u64,
}

pub const OTP_SCHEME: &str = "otpauth";
pub const TOTP_HOST: &str = "totp";
pub const QUERY_SECRET: &str = "secret";
pub const QUERY_ISSUER: &str = "issuer";
pub const QUERY_ALGORITHM: &str = "algorithm";
pub const QUERY_DIGITS: &str = "digits";
pub const QUERY_PERIOD: &str = "period";

pub const DEFAULT_ALGORITHM: Algorithm = Algorithm::SHA1;
pub const DEFAULT_DIGITS: u8 = 6;
pub const DEFAULT_PERIOD: u16 = 30;

impl TOTP {
    pub fn from_uri(uri: &str) -> Result<Self, TOTPError> {
        match Url::parse(uri) {
            Ok(uri) => Self::parse_uri(uri),

            // Not an URI, remove all white spaces and treat the whole string as secret
            _ => Ok(TOTP {
                secret: uri.chars().filter(|c| !c.is_whitespace()).collect(),
                ..Default::default()
            }),
        }
    }

    fn parse_uri(uri: Url) -> Result<Self, TOTPError> {
        Self::check_scheme(&uri)?;
        Self::check_otp_type(&uri)?;

        let label = Self::parse_label(&uri);

        let queries = &Self::parse_queries(&uri)?;
        let secret = queries.get_secret()?;
        let issuer = queries.get_issuer();
        let algorithm = queries.get_algorithm()?;
        let digits = queries.get_digits();
        let period = queries.get_period();

        Ok(Self {
            label,
            secret,
            issuer,
            algorithm,
            digits,
            period,
        })
    }

    fn parse_queries(uri: &Url) -> Result<Queries, TOTPError> {
        let queries_string = uri.query().ok_or(TOTPError::NoQueries)?;
        Ok(Queries::new(queries_string))
    }
}

impl TOTP {
    fn check_scheme(uri: &Url) -> Result<(), TOTPError> {
        let scheme = uri.scheme().to_string();
        if scheme.to_lowercase() == OTP_SCHEME {
            Ok(())
        } else {
            Err(TOTPError::InvalidScheme(scheme))
        }
    }

    fn check_otp_type(uri: &Url) -> Result<(), TOTPError> {
        let authority = uri.authority();
        if authority.is_empty() {
            Err(TOTPError::NoAuthority)
        } else if authority.to_lowercase() == TOTP_HOST {
            Ok(())
        } else {
            Err(TOTPError::InvalidAuthority(authority.to_string()))
        }
    }

    fn parse_label(uri: &Url) -> Option<String> {
        match uri.path_segments() {
            Some(segments) => {
                if let Some(label) = segments.last() {
                    if !label.is_empty() {
                        Some(label.to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl TOTP {
    pub fn has_default_params(&self) -> bool {
        let default_algorithm = match &self.algorithm {
            Some(value) => *value == DEFAULT_ALGORITHM,
            _ => true,
        };

        let default_digits = match &self.digits {
            Some(value) => *value == DEFAULT_DIGITS,
            _ => true,
        };

        let default_period = match &self.period {
            Some(value) => *value == DEFAULT_PERIOD,
            _ => true,
        };

        default_algorithm && default_digits && default_period
    }

    pub fn unwrap_algorithm(&self) -> Algorithm {
        self.algorithm.unwrap_or(DEFAULT_ALGORITHM)
    }

    pub fn unwrap_digits(&self) -> u8 {
        self.digits.unwrap_or(DEFAULT_DIGITS)
    }

    pub fn unwrap_period(&self) -> u16 {
        self.period.unwrap_or(DEFAULT_PERIOD)
    }
}

impl TOTP {
    pub fn to_uri(&self, original_label: Option<String>, original_issuer: Option<String>) -> String {
        let base_uri = format!("{}://{}/", OTP_SCHEME, TOTP_HOST);

        let mut uri = match Url::parse(&base_uri) {
            Ok(value) => value,
            _ => panic!(
                "Should be able to create Url struct with scheme {} and host {}",
                OTP_SCHEME, TOTP_HOST
            ),
        };

        // Add label path
        if let Some(edited_label) = &self.label {
            uri.set_path(edited_label.as_str());
        } else if let Some(original_label) = original_label {
            uri.set_path(original_label.as_str());
        }

        // Set secret query
        uri.query_pairs_mut().append_pair(QUERY_SECRET, &self.secret);

        // Set issuer query
        if let Some(edited_issuer) = &self.issuer {
            uri.query_pairs_mut().append_pair(QUERY_ISSUER, edited_issuer.as_str());
        } else if let Some(original_issuer) = original_issuer {
            uri.query_pairs_mut()
                .append_pair(QUERY_ISSUER, original_issuer.as_str());
        }

        // Set algorithm query
        let algorithm = match &self.algorithm {
            Some(entered_algorithm) => entered_algorithm,
            _ => &DEFAULT_ALGORITHM,
        };
        uri.query_pairs_mut().append_pair(QUERY_ALGORITHM, algorithm.value());

        // Set digits
        let digits = match &self.digits {
            Some(entered_digits) => entered_digits,
            _ => &DEFAULT_DIGITS,
        };
        uri.query_pairs_mut().append_pair(QUERY_DIGITS, &format!("{}", digits));

        // Set period
        let period = match &self.period {
            Some(entered_period) => entered_period,
            _ => &DEFAULT_PERIOD,
        };
        uri.query_pairs_mut().append_pair(QUERY_PERIOD, &format!("{}", period));
        uri.as_str().to_string()
    }
}

impl TOTP {
    pub fn generate_token(&self, current_time: u64) -> Result<String, TOTPError> {
        let sanitized_secret = sanitize_secret(self.secret.as_str());
        let encoded_secret = totp_rs::Secret::Encoded(sanitized_secret)
            .to_bytes()
            .map_err(|_| TOTPError::SecretParseError)?;
        let totp = totp_rs::TOTP::new_unchecked(
            self.algorithm.as_ref().unwrap_or(&DEFAULT_ALGORITHM).into(),
            self.digits.unwrap_or(DEFAULT_DIGITS) as usize,
            1,
            self.period.unwrap_or(DEFAULT_PERIOD) as u64,
            encoded_secret,
        );
        Ok(totp.generate(current_time))
    }
}

#[cfg(test)]
mod test_from_uri {
    use super::*;

    fn make_sut(uri: &str) -> Result<TOTP, TOTPError> {
        TOTP::from_uri(uri)
    }

    #[test]
    fn invalid_scheme() {
        // Given
        let uri = "https://totp/john.doe%40example.com?secret=somesecret&algorithm=SHA1&digits=8&period=30";

        // When
        let sut = make_sut(uri);

        // Then
        assert_eq!(sut, Err(TOTPError::InvalidScheme("https".to_string())));
    }

    #[test]
    fn invalid_authority() {
        // Given
        let uri = "otpauth://hotp/john.doe%40example.com?secret=somesecret&algorithm=SHA1&digits=8&period=30";

        // When
        let sut = make_sut(uri);

        // Then
        assert_eq!(sut, Err(TOTPError::InvalidAuthority("hotp".to_string())));
    }

    #[test]
    fn no_authority() {
        // Given
        let uri = "otpauth://?secret=somesecret&algorithm=SHA1&digits=8&period=30";

        // When
        let sut = make_sut(uri);

        // Then
        assert_eq!(sut, Err(TOTPError::NoAuthority));
    }

    #[test]
    fn no_queries() {
        // Given
        let uri = "otpauth://totp/";

        // When
        let sut = make_sut(uri);

        // Then
        assert_eq!(sut, Err(TOTPError::NoQueries));
    }

    #[test]
    fn no_secret() {
        // Given
        let uri = "otpauth://totp/john.doe%40example.com?algorithm=SHA1&digits=8&period=30";

        // When
        let sut = make_sut(uri);

        // Then
        assert_eq!(sut, Err(TOTPError::NoSecret));
    }

    #[test]
    fn empty_secret() {
        // Given
        let uri = "otpauth://totp/john.doe%40example.com?secret=&algorithm=SHA1&digits=8&period=30";

        // When
        let sut = make_sut(uri);

        // Then
        assert_eq!(sut, Err(TOTPError::EmptySecret));
    }

    #[test]
    fn invalid_algorithm() {
        // Given
        let uri = "otpauth://totp/john.doe%40example.com?secret=somesecret&algorithm=SHA128&digits=8&period=30";

        // When
        let sut = make_sut(uri);

        // Then
        assert_eq!(sut, Err(TOTPError::InvalidAlgorithm("SHA128".to_string())));
    }

    #[test]
    fn explicit_params() {
        // Given
        let uri =
            "otpauth://totp/john.doe%40example.com?secret=somesecret&issuer=ProtonMail&algorithm=SHA512&digits=8&period=45";

        // When
        let sut = make_sut(uri);

        // Then
        match sut {
            Ok(components) => {
                assert_eq!(components.label, Some("john.doe%40example.com".to_string()));
                assert_eq!(components.secret, "somesecret");
                assert_eq!(components.issuer, Some("ProtonMail".to_string()));
                assert_eq!(components.algorithm, Some(Algorithm::SHA512));
                assert_eq!(components.digits, Some(8));
                assert_eq!(components.period, Some(45));
            }
            _ => panic!("Should be able to parse"),
        }
    }

    #[test]
    fn implicit_params() {
        // Given
        let uri = "otpauth://totp/?secret=somesecret";

        // When
        let sut = make_sut(uri);

        // Then
        match sut {
            Ok(components) => {
                assert_eq!(components.label, None);
                assert_eq!(components.secret, "somesecret");
                assert_eq!(components.issuer, None);
                assert_eq!(components.algorithm, None);
                assert_eq!(components.digits, None);
                assert_eq!(components.period, None);
            }
            _ => panic!("Should be able to parse"),
        }
    }

    #[test]
    fn whole_uri_as_secret() {
        // Given
        let uri = "not an uri";

        // When
        let sut = make_sut(uri);

        // Then
        match sut {
            Ok(components) => {
                assert_eq!(components.label, None);
                assert_eq!(components.secret, "notanuri");
                assert_eq!(components.issuer, None);
                assert_eq!(components.algorithm, None);
                assert_eq!(components.digits, None);
                assert_eq!(components.period, None);
            }
            _ => panic!("Should be able to parse"),
        }
    }
}

#[cfg(test)]
mod test_has_default_params {
    use super::*;

    #[test]
    fn custom_params() {
        // Given
        let sut = TOTP {
            label: None,
            secret: "somesecret".to_string(),
            issuer: None,
            algorithm: Some(Algorithm::SHA512),
            digits: Some(DEFAULT_DIGITS),
            period: Some(DEFAULT_PERIOD),
        };

        // Then
        assert!(!sut.has_default_params());
        assert_eq!(sut.unwrap_algorithm(), Algorithm::SHA512);
        assert_eq!(sut.unwrap_digits(), DEFAULT_DIGITS);
        assert_eq!(sut.unwrap_period(), DEFAULT_PERIOD);
    }

    #[test]
    fn explicit_default_params() {
        // Given
        let sut = TOTP {
            label: None,
            secret: "somesecret".to_string(),
            issuer: None,
            algorithm: Some(DEFAULT_ALGORITHM),
            digits: Some(DEFAULT_DIGITS),
            period: Some(DEFAULT_PERIOD),
        };

        // Then
        assert!(sut.has_default_params());
        assert_eq!(sut.unwrap_algorithm(), DEFAULT_ALGORITHM);
        assert_eq!(sut.unwrap_digits(), DEFAULT_DIGITS);
        assert_eq!(sut.unwrap_period(), DEFAULT_PERIOD);
    }

    #[test]
    fn implicit_default_params() {
        // Given
        let sut = TOTP {
            label: None,
            secret: "somesecret".to_string(),
            issuer: None,
            algorithm: None,
            digits: None,
            period: None,
        };

        // Then
        assert!(sut.has_default_params());
        assert_eq!(sut.unwrap_algorithm(), DEFAULT_ALGORITHM);
        assert_eq!(sut.unwrap_digits(), DEFAULT_DIGITS);
        assert_eq!(sut.unwrap_period(), DEFAULT_PERIOD);
    }
}

#[cfg(test)]
mod test_to_uri {
    use super::*;

    #[test]
    fn to_uri() {
        assert_eq!(
            TOTP {
                label: None,
                secret: "some_secret".to_string(),
                issuer: None,
                algorithm: None,
                digits: None,
                period: None,
            }
            .to_uri(None, None),
            "otpauth://totp/?secret=some_secret&algorithm=SHA1&digits=6&period=30".to_string()
        );

        assert_eq!(
            TOTP {
                label: None,
                secret: "some_secret".to_string(),
                issuer: None,
                algorithm: None,
                digits: None,
                period: None,
            }
            .to_uri(Some("john.doe".to_string()), None),
            "otpauth://totp/john.doe?secret=some_secret&algorithm=SHA1&digits=6&period=30".to_string()
        );

        assert_eq!(
            TOTP {
                label: None,
                secret: "some_secret".to_string(),
                issuer: None,
                algorithm: None,
                digits: None,
                period: None,
            }
            .to_uri(Some("john.doe".to_string()), Some("Proton".to_string())),
            "otpauth://totp/john.doe?secret=some_secret&issuer=Proton&algorithm=SHA1&digits=6&period=30".to_string()
        );

        assert_eq!(
            TOTP {
                label: Some("jane.doe".to_string()),
                secret: "some_secret".to_string(),
                issuer: None,
                algorithm: None,
                digits: None,
                period: None,
            }
            .to_uri(Some("john.doe".to_string()), Some("Proton".to_string())),
            "otpauth://totp/jane.doe?secret=some_secret&issuer=Proton&algorithm=SHA1&digits=6&period=30".to_string()
        );

        assert_eq!(
            TOTP {
                label: Some("jane.doe".to_string()),
                secret: "some_secret".to_string(),
                issuer: None,
                algorithm: Some(Algorithm::SHA512),
                digits: Some(8),
                period: None,
            }
            .to_uri(Some("john.doe".to_string()), Some("Proton".to_string())),
            "otpauth://totp/jane.doe?secret=some_secret&issuer=Proton&algorithm=SHA512&digits=8&period=30".to_string()
        );
    }
}
