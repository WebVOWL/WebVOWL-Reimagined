use actix_web::{HttpResponse, web};
use leptos::server_fn::codec::MultipartFormData;
use leptos::{prelude::*, server_fn::codec::MultipartData};
use reqwest::Client;
use std::{collections::HashMap, fs, path::Path};

#[derive(Clone)]
pub struct NetworkModule {
    client: Client,
}

/// Represents the different endpoints needed
#[derive(Debug, Clone)]
pub enum NetworkEndpoint {
    /// path
    Local(String),
    /// url
    Remote(String),
    /// url and query
    SPARQL { endpoint: String, query: String },
}

#[derive(Debug, Clone)]
pub enum DataType {
    OWL,
    TTL,
    RDF,
    SPARQLJSON,
    SPARQLXML,
    /// fallback when type cant be determined
    UNKNOWN,
}

impl DataType {
    /// Map file extensions to datatypes
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "owl" => Self::OWL,
            "ttl" => Self::TTL,
            "rdf" => Self::RDF,
            "sparql" => Self::SPARQLJSON,
            _ => Self::UNKNOWN,
        }
    }

    // Fixed string literals called by reference as to not allocate new memory each time the function is called
    /// labels the data extension type
    pub fn mime_type(&self) -> &'static str {
        match self {
            Self::OWL => "application/owl+xml",
            Self::TTL => "text/turtle",
            Self::RDF => "application/rdf+xml",
            Self::SPARQLJSON => "application/sparql-results+json",
            Self::SPARQLXML => "application/sparql-results+xml",
            Self::UNKNOWN => "application/octet-stream",
        }
    }
}

impl NetworkModule {
    // constructor
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    // example: is it OWL, TTL, RDF, etc. the local endpoint is given?
    /// Determines what datatype the given data at the endpoint is
    fn find_data_type(path: &str) -> Option<DataType> {
        Path::new(path)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(DataType::from_extension) // identifies the type from the file extension
    }

    pub async fn retrieval_response(&self, source: NetworkEndpoint) -> HttpResponse {
        // 1: retrieves the data
        let result = match source {
            NetworkEndpoint::Remote(url) => match self.client.get(&url).send().await {
                Ok(resp) => match resp.text().await {
                    Ok(text) => Ok((Self::find_data_type(&url).unwrap_or(DataType::RDF), text)),
                    Err(e) => Err(format!("Error reading remote response text: {}", e)),
                },
                Err(e) => Err(format!("Error retrieving remote URL: {}", e)),
            },
            NetworkEndpoint::SPARQL { endpoint, query } => {
                let accept_type = DataType::SPARQLJSON.mime_type(); //default

                match self
                    .client
                    .post(&endpoint)
                    .header("Accept", accept_type)
                    .form(&[("query", query)])
                    .send()
                    .await
                {
                    Ok(resp) => match resp.text().await {
                        Ok(text) => Ok((
                            if accept_type.contains("xml") {
                                DataType::SPARQLXML
                            } else {
                                DataType::SPARQLJSON
                            },
                            text,
                        )),
                        Err(e) => Err(format!("Error reading SPARQL response: {}", e)),
                    },
                    Err(e) => Err(format!("Error querying SPARQL endpoint: {}", e)),
                }
            }
            NetworkEndpoint::Local(_) => todo!(),
        };

        // 2: Handles response result so (datatype label, data content)
        match result {
            Ok((dtype, content)) => {
                // TODO ADD PARSER HERE!

                HttpResponse::Ok()
                    .content_type(dtype.mime_type())
                    .body(content)
            }
            Err(e) => HttpResponse::InternalServerError().body(e),
        }
    }
}

/// Local reads file and calls for the datatype label and returns (label, data content)
#[server(input = MultipartFormData)]
pub async fn handle_local(data: MultipartData) -> Result<usize, ServerFnError> {
    let mut data = data.into_inner().unwrap();

    // match fs::read_to_string(&path) {
    //     Ok(content) => Ok((
    //         Self::find_data_type(&path).unwrap_or(DataType::RDF),
    //         content,
    //     )),
    //     Err(e) => Err(format!("Error reading local file: {}", e)),
    // }

    // this will just measure the total number of bytes uploaded
    let mut count = 0;
    while let Ok(Some(mut field)) = data.next_field().await {
        while let Ok(Some(chunk)) = field.chunk().await {
            let len = chunk.len();
            count += len;
        }
    }

    Ok(count)
}

pub async fn retrieval_handler(
    query: web::Query<HashMap<String, String>>, // turns URL into query parameters (endpoint, data)
    data: web::Data<NetworkModule>, // shared instance so we only have one NetworkModule with client shared by all users.
) -> HttpResponse {
    // Determine datatype of source
    let source = if let Some(path) = query.get("local") {
        NetworkEndpoint::Local(path.clone())
    } else if let Some(url) = query.get("remote") {
        NetworkEndpoint::Remote(url.clone())
    } else if let (Some(endpoint), Some(q)) =
        (query.get("sparql_endpoint"), query.get("sparql_query"))
    {
        let dtype = query
            .get("format")
            .map(|f| {
                if f.to_lowercase() == "xml" {
                    DataType::SPARQLXML
                } else {
                    DataType::SPARQLJSON
                }
            })
            .unwrap_or(DataType::SPARQLJSON);

        NetworkEndpoint::SPARQL {
            endpoint: endpoint.clone(),
            query: q.clone(),
        }
    } else {
        return HttpResponse::BadRequest().body("Missing parameters");
    };

    // Retrieve and Parse data (takes the determined endpoint and passes it to retrieval_response)
    data.retrieval_response(source).await
}
