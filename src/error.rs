use hyper;
use serde;

#[derive(serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum Error {
    InternalServerError,
    NotFoundError,
    PermissionDeniedError,
    RateLimitedError { 
        #[serde(rename = "retryAfter")]
        retry_after: i64 
    },
    UnprocessableEntityError,
    ValidationError,
    BadRequest,
    Unauthorized,
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InternalServerError => {
                write!(formatter, "Internal Server Error")
            },
            Error::NotFoundError => {
                write!(formatter, "Not Found Error")
            },
            Error::PermissionDeniedError => {
                write!(formatter, "Permission Denied Error")
            },
            Error::RateLimitedError { retry_after } => {
                write!(formatter, "Rate Limited Error (retry after {} seconds)", retry_after)
            },
            Error::UnprocessableEntityError => { 
                write!(formatter,"Unprocessable Entity Error")
            },
            Error::ValidationError => {
                write!(formatter, "Validation Error")
            },
            Error::BadRequest => {
                write!(formatter, "Bad Request")
            },
            Error::Unauthorized => {
                write!(formatter, "Unauthorized. Invalid token.")
            },
        }
    }
}

impl std::error::Error for Error { }

impl Error {
    pub fn is_error_code(
        status_code: hyper::StatusCode,
    )-> bool {
        use hyper::StatusCode;
        match status_code {
            StatusCode::OK => false,
            StatusCode::CREATED => false,
            _ => true,
        }
    }

    pub async fn to_error(
        status_code: hyper::StatusCode,
        body: hyper::Body,
    ) -> Error {
        use hyper::StatusCode;
        use super::extractor;
        match status_code {
            StatusCode::BAD_REQUEST => Error::BadRequest,
            StatusCode::UNAUTHORIZED |
            StatusCode::FORBIDDEN |
            StatusCode::NOT_FOUND |
            StatusCode::UNPROCESSABLE_ENTITY |
            StatusCode::TOO_MANY_REQUESTS => {
                extractor::extract_error(body)
                    .await
                    .map_or(Error::InternalServerError, |error| error)
            },
            _ => Error::InternalServerError,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use hyper::StatusCode;
    use tokio_test;

    #[test]
    fn is_error_code() {
        let true_status_codes = [
            StatusCode::OK,
            StatusCode::CREATED,
        ];
        let false_status_codes = [
            StatusCode::BAD_REQUEST,
            StatusCode::UNAUTHORIZED,
            StatusCode::FORBIDDEN,
            StatusCode::NOT_FOUND,
            StatusCode::UNPROCESSABLE_ENTITY,
            StatusCode::TOO_MANY_REQUESTS,
            StatusCode::NON_AUTHORITATIVE_INFORMATION,
            StatusCode::MULTI_STATUS,
        ];
        true_status_codes
            .iter()
            .for_each(|value| {
                let result = Error::is_error_code(value.clone());
                assert!(!result, "Expected this status code is not error code")
            });
        false_status_codes
            .iter()
            .for_each(|value| {
                let result = Error::is_error_code(value.clone());
                assert!(result, "Expected this status code is error code")
            });
    }

    #[test]
    fn to_error() {
        let empty_body = "{}";
        let (mut sender, body) = hyper::Body::channel();
        tokio_test::block_on(
            sender.send_data(hyper::body::Bytes::from(empty_body)))
            .expect("Failed to send data");
        let status_code = StatusCode::BAD_REQUEST;
        let error = tokio_test::block_on(Error::to_error(status_code, body));
        println!("{:?}", error);
        match error {
            Error::BadRequest => assert!(true),
            _ => assert!(false, "Failed to get error code from empty enum!")
        }
    }
}
