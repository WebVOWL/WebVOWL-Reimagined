use leptos::prelude::*;
use leptos::server_fn::ServerFnError;
use leptos::server_fn::codec::{MultipartData, MultipartFormData};
#[cfg(feature = "server")]
use reqwest::Client;
#[cfg(feature = "server")]
use std::path::Path;
use web_sys::FormData;
use crate::network::DataType;

async fn extract_bytes(
    mut data: MultipartData,
) -> Result<(Vec<u8>, Option<String>), ServerFnError> {
    let mut bytes = Vec::new();
    let mut filename = None;
    let mut inner = data.into_inner().unwrap();

    while let Ok(Some(mut field)) = inner.next_field().await {
        if filename.is_none() {
            filename = field.file_name().map(|f| f.to_string());
        }
        while let Ok(Some(chunk)) = field.chunk().await {
            bytes.extend_from_slice(&chunk);
        }
    }
    Ok((bytes, filename))
}

/// Local reads file and calls for the datatype label and returns (label, data content)
#[server(input = MultipartFormData)]
pub async fn handle_local(data: MultipartData) -> Result<(DataType, String), ServerFnError> {
    let (bytes, filename) = extract_bytes(data).await?;

    let content = match String::from_utf8(bytes) {
        Ok(s) => s,
        Err(e) => return Err(ServerFnError::ServerError(format!("Invalid UTF-8: {}", e))),
    };

    let dtype = filename
        .as_ref()
        .and_then(|name| Path::new(name).extension()?.to_str())
        .map(DataType::from_extension)
        .unwrap_or(DataType::UNKNOWN);

    Ok((dtype, content))
}

#[server]
pub async fn handle_remote(url: String) -> Result<(DataType, String), ServerFnError> {
    let client = Client::new();

    let resp = match client.get(&url).send().await {
        Ok(r) => r,
        Err(e) => {
            return Err(ServerFnError::ServerError(format!(
                "Error fetching URL: {e}"
            )));
        }
    };

    let text = match resp.text().await {
        Ok(t) => t,
        Err(e) => {
            return Err(ServerFnError::ServerError(format!(
                "Error reading response: {e}"
            )));
        }
    };

    let dtype = Path::new(&url)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(DataType::from_extension)
        .unwrap_or(DataType::UNKNOWN);

    Ok((dtype, text))
}

#[server]
pub async fn handle_sparql(
    endpoint: String,
    query: String,
    format: Option<String>,
) -> Result<(DataType, String), ServerFnError> {
    let client = Client::new();

    let accept_type = match format.as_deref() {
        Some("xml") => DataType::SPARQLXML.mime_type(),
        _ => DataType::SPARQLJSON.mime_type(),
    };

    let resp = match client
        .post(&endpoint)
        .header("Accept", accept_type)
        .form(&[("query", query)])
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            return Err(ServerFnError::ServerError(format!(
                "Error querying SPARQL endpoint: {e}"
            )));
        }
    };

    let text = match resp.text().await {
        Ok(r) => r,
        Err(e) => {
            return Err(ServerFnError::ServerError(format!(
                "Error reading SPARQL response: {e}"
            )));
        }
    };

    let dtype = if accept_type.contains("xml") {
        DataType::SPARQLXML
    } else {
        DataType::SPARQLJSON
    };
    Ok((dtype, text))
}

#[derive(Clone)]
pub struct FileUpload {
    pub mode: RwSignal<String>,
    pub local_action: Action<FormData, Result<(DataType, String), ServerFnError>>,
    pub remote_action: Action<String, Result<(DataType, String), ServerFnError>>,
    pub sparql_action:
        Action<(String, String, Option<String>), Result<(DataType, String), ServerFnError>>,
}
impl FileUpload {
    pub fn new() -> Self {
        let mode = RwSignal::new("local".to_string());

        let local_action = Action::<FormData, Result<(DataType, String), ServerFnError>>::new_local(
            |data: &FormData| {
                let multipart: MultipartData = data.clone().into();
                handle_local(multipart)
            },
        );

        let remote_action = Action::new(|url: &String| handle_remote(url.clone()));

        let sparql_action = Action::new(
            |(endpoint, query, format): &(String, String, Option<String>)| {
                handle_sparql(endpoint.clone(), query.clone(), format.clone())
            },
        );

        Self {
            mode,
            local_action,
            remote_action,
            sparql_action,
        }
    }

    pub fn get_result(&self) -> Option<Result<(DataType, String), ServerFnError>> {
        match self.mode.get().as_str() {
            "local" => self.local_action.value().get(),
            "remote" => self.remote_action.value().get(),
            "sparql" => self.sparql_action.value().get(),
            _ => None,
        }
    }
}
