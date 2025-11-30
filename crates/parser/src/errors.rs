use std::{
    fmt::{Display, Formatter},
    io::{Error, ErrorKind},
    panic::Location,
};

use horned_owl::error::HornedError;
use rdf_fusion::{
    error::LoaderError, execution::sparql::error::QueryEvaluationError, model::IriParseError,
};
use tokio::task::JoinError;

#[derive(Debug)]
pub enum WebVowlStoreErrorKind {
    InvalidInput(String),
    HornedError(HornedError),
    IOError(std::io::Error),
    IriParseError(IriParseError),
    LoaderError(LoaderError),
    QueryEvaluationError(QueryEvaluationError),
    JoinError(JoinError),
}

#[derive(Debug)]
pub struct WebVowlStoreError {
    inner: WebVowlStoreErrorKind,
    location: &'static Location<'static>,
}

impl Display for WebVowlStoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "WebVowlStoreError: {:?}", self.inner)
    }
}

impl Into<Error> for WebVowlStoreError {
    fn into(self) -> Error {
        Error::new(ErrorKind::Other, self.to_string())
    }
}

impl From<HornedError> for WebVowlStoreError {
    #[track_caller]
    fn from(error: HornedError) -> Self {
        WebVowlStoreError {
            inner: WebVowlStoreErrorKind::HornedError(error),
            location: &Location::caller(),
        }
    }
}

impl From<IriParseError> for WebVowlStoreError {
    #[track_caller]
    fn from(error: IriParseError) -> Self {
        WebVowlStoreError {
            inner: WebVowlStoreErrorKind::IriParseError(error),
            location: &Location::caller(),
        }
    }
}

impl From<LoaderError> for WebVowlStoreError {
    #[track_caller]
    fn from(error: LoaderError) -> Self {
        WebVowlStoreError {
            inner: WebVowlStoreErrorKind::LoaderError(error),
            location: &Location::caller(),
        }
    }
}
impl From<WebVowlStoreErrorKind> for WebVowlStoreError {
    #[track_caller]
    fn from(error: WebVowlStoreErrorKind) -> Self {
        WebVowlStoreError {
            inner: error,
            location: &Location::caller(),
        }
    }
}

impl From<std::io::Error> for WebVowlStoreError {
    #[track_caller]
    fn from(error: std::io::Error) -> Self {
        WebVowlStoreError {
            inner: WebVowlStoreErrorKind::IOError(error),
            location: &Location::caller(),
        }
    }
}
impl From<QueryEvaluationError> for WebVowlStoreError {
    #[track_caller]
    fn from(error: QueryEvaluationError) -> Self {
        WebVowlStoreError {
            inner: WebVowlStoreErrorKind::QueryEvaluationError(error),
            location: &Location::caller(),
        }
    }
}
impl From<JoinError> for WebVowlStoreError {
    #[track_caller]
    fn from(error: JoinError) -> Self {
        WebVowlStoreError {
            inner: WebVowlStoreErrorKind::JoinError(error),
            location: &Location::caller(),
        }
    }
}
