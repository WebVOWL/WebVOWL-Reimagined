use std::panic::Location;

use horned_owl::error::HornedError;
use rdf_fusion::{error::LoaderError, execution::sparql::error::QueryEvaluationError, model::IriParseError};

#[derive(Debug)]
pub enum WebVowlStoreErrorKind {
    InvalidInput(String),
    HornedError(HornedError),
    IOError(std::io::Error),
    IriParseError(IriParseError),
    LoaderError(LoaderError),
    QueryEvaluationError(QueryEvaluationError),
}

#[derive(Debug)]
pub struct WebVowlStoreError {
    inner: WebVowlStoreErrorKind,
    location: &'static Location<'static>,
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

