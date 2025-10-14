use actix_web::{HttpResponse, web, Responder};
use reqwest::Client;
use std::{collections::HashMap, fs, path::Path};


#[derive(Clone)]
pub struct NetworkModule {
    client: Client,
}

#[derive(Debug, Clone)]
pub enum NetworkEndpoint {
    Local(String),
    Remote(String),
    SPARQL {endpoint: String, query: String},
}

#[derive(Debug, Clone)]
pub enum DataType {
    OWL,
    TTL,
    RDF,
    SPARQLJSON,
    SPARQLXML,
    UNKNOWN,
}


impl DataType {
    // Map file extensions to datatypes
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "owl" => Self::OWL,
            "ttl" => Self::TTL,
            "rdf" => Self::RDF,
            "sqarql" => Self::SPARQLJSON,
            _ => Self::UNKNOWN,
        }
    }

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
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_response(&self, source: NetworkEndpoint) -> impl Responder {
        // 1: Fetch the data
        let fetch_result: Result<(DataType, String), String> = match source {
            NetworkEndpoint::Local(path) => {
                let content = fs::read_to_string(&path) 
                    .map(|e| format!("Error reading local file: {e}"))?;
                let dtype = Path::new(&path)
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(DataType::RDF);
                Ok((dtype, content))
            }

            NetworkEndpoint::Remote(url) => {
                let resp = self.client.get(&url).send().await 
                    .map_err(|e| format!("Error fetching remote URL: {e}"))?;
                let text = resp.text().await
                    .map_err(|e| format!("Error reading remote response text: {e}"))?;
                let dtype = Path::new(&url)
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(DataType::RDF);
                Ok((dtype, content))
            }

            NetworkEndpoint::SPARQL { endpoint, query } => {
                let resp = self.client
                    .post(&endpoint)
                    .header("Accept", DataType::SPARQLJSON.mime_type())
                    .form (&[("query", query)])
                    .send()
                    .await 
                    .map_err(|e| format!("Error querying SPARQL endpoint: {e}"))?;
                let text = resp.text().await
                    .map_err(|e| format!("Error reading SPARQL response: {e}"))?;
                Ok((DataType::SPARQLJSON, text))
            }
        };

        // 2: Handle result 
        match result {
            Ok((dtype, content)) => {

                // ADD PARSER HERE!

                HttpResponse::Ok()
                    .content_type(dtype.mime_type())
                    .body(content)
            }
            Err(e) => HttpResponse::InternalServerError().body(e),
        }
    }
}


pub async fn fetch_handler(
    query: web::Query<HashMap<String, String>>, 
    data: web::Data<NetworkModule>
) -> impl Responder {
    // Determine datatype of source
    let source = if let Some(path) = query.get("local"){
        NetworkEndpoint::Local(path.clone())

    }   else if let Some(url) = query.get("remote") {
        NetworkEndpoint::Remote(url.clone())

    }   else if let (Some(endpoint), Some(q)) = (query.get("sparql_endpoint"), query.get("sparql_query")) {
        NetworkEndpoint::SPARQL { 
            endpoint: endpoint.clone(), 
            query: q.clone() 
        }

    }   else   {
        return HttpResponse::BadRequest().body("Missing parameters");
        
    };

    // Fetch and Parse data
    data.fetch_respone(source).await
}