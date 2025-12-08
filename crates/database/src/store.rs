use futures::StreamExt;
use rdf_fusion::store::Store;
use std::fs::File;
use std::path::Path;

use webvowl_parser::{
    errors::WebVowlStoreError,
    parser_util::{ResourceType, parse_stream_to, parser_from_format},
};

static GLOBAL_STORE: std::sync::OnceLock<Store> = std::sync::OnceLock::new();

pub struct WebVOWLStore {
    pub session: Store,
    upload_handle: Option<tempfile::NamedTempFile>,
}
impl WebVOWLStore {
    pub fn new(session: Store) -> Self {
        Self {
            session,
            upload_handle: None,
        }
    }

    pub fn default() -> Self {
        let session = GLOBAL_STORE.get_or_init(Store::default).clone();
        Self {
            session,
            upload_handle: None,
        }
    }

    // TTL format -> (oxittl) RDF XML quads -> (horned_owl) Normalize OWL/RDF -> Quads -> Insert into Oxigraph
    pub async fn insert_file(&self, fs: &Path, lenient: bool) -> Result<(), WebVowlStoreError> {
        let parser = parser_from_format(fs, lenient)?;

        self.session
            .load_from_reader(parser.parser, parser.input.as_slice())
            .await?;

        Ok(())
    }

    pub async fn serialize_to_file(&self, path: &Path) -> Result<(), WebVowlStoreError> {
        let mut file = File::create(path)?;
        let mut results = parse_stream_to(self.session.stream().await?, ResourceType::OWL).await?;
        while let Some(result) = results.next().await {
            let result = result.unwrap();
            std::io::Write::write_all(&mut file, &result)?;
        }

        Ok(())
    }

    pub async fn serialize_to_string(&self) -> Result<String, WebVowlStoreError> {
        println!(
            "Store size before export: {}",
            self.session.len().await.unwrap_or(0)
        );
        let mut results = parse_stream_to(self.session.stream().await?, ResourceType::OWL).await?;
        let mut out = vec![];
        while let Some(result) = results.next().await {
            let result = result.unwrap();
            out.extend(result);
        }
        Ok(String::from_utf8(out).unwrap())
    }

    pub async fn start_upload(&mut self, filename: &str) -> Result<(), WebVowlStoreError> {
        let extension = Path::new(filename)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("owl");
        let file = tempfile::Builder::new()
            .suffix(&format!(".{}", extension))
            .tempfile()?;
        self.upload_handle = Some(file);
        Ok(())
    }

    pub async fn upload_chunk(&mut self, data: &[u8]) -> Result<(), WebVowlStoreError> {
        if let Some(file) = &mut self.upload_handle {
            std::io::Write::write_all(file, data)?;
            Ok(())
        } else {
            println!("Warning: upload_chunk called without start_upload");
            Ok(())
        }
    }

    pub async fn complete_upload(&mut self) -> Result<(), WebVowlStoreError> {
        if let Some(file) = &mut self.upload_handle {
            std::io::Write::flush(file)?;
            let path = file.path();
            let parser = parser_from_format(path, false)?;
            self.session
                .load_from_reader(parser.parser, parser.input.as_slice())
                .await?;
            println!("Loaded ontology");
        }
        self.upload_handle = None;
        Ok(())
    }
}

pub const DEFAULT_QUERY: &str = r#"
    PREFIX owl: <http://www.w3.org/2002/07/owl#>
    PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
    PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
    PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
    PREFIX webvowl: <http://www.example.com/iri#>

    SELECT ?id ?nodeType ?label
    WHERE {
        {
            # 1. Identify Named Classes
            ?id a owl:Class .
            FILTER(isIRI(?id))
            BIND(owl:Class AS ?nodeType)
            OPTIONAL { ?id rdfs:label ?label }
        }
        UNION
        {
            ?id a owl:Class
            FILTER(!isIRI(?id))
            BIND("blanknode" AS ?nodeType)
        }
        UNION
        {
            # 2. Identify Intersections
            # Any node (usually blank) that is the subject of an intersectionOf list
            ?id owl:intersectionOf ?label .
            BIND("IntersectionOf" AS ?nodeType)
        }
        UNION
        {
            # 3. Identify Unions
            ?id owl:unionOf ?list .
            BIND("UnionOf" AS ?nodeType)
        }
        UNION
        {
            # 4. Identify Restrictions (Anonymous Classes in WebVOWL)
            ?id a owl:Restriction .
            BIND("AnonymousClass" AS ?nodeType)
        }
        UNION
        {
            ?id owl:equivalentClass ?label
            BIND("EquivalentClass" AS ?nodeType)
        }
        # Edges
        UNION
        {
            # 1. Identify RDF properties
            ?id rdf:Property ?target
            BIND("SubClass" AS ?nodeType)
        }
        UNION
        {
            # 2. Identify subclasses
            ?id rdfs:SubClassOf ?target
            BIND("SubClass" AS ?nodeType)
        }
        UNION
        {
            # 3. Identify datatypes
            ?id rdfs:Datatype ?target
            BIND("Datatype" AS ?nodeType)
        }
        UNION
        {
            # 4. Identify OWL datatype properties
            ?id owl:DatatypeProperty ?target
            BIND("DatatypeProperty" AS ?nodeType)
        }
        UNION
        {
            # 5. Identify OWL disjoint with
            ?id owl:disjointWith ?target
            BIND("disjointWith" AS ?nodeType)
        }
        UNION
        {
            # 6. WIP Identify OWL deprecated properties
            ?id owl:deprecated "true"^^<http://www.w3.org/2001/XMLSchema#boolean>
            BIND("DeprecatedProperty" AS ?nodeType)
        }
    }
    ORDER BY ?nodeType
    "#;

#[cfg(test)]
#[allow(unused_must_use)]
mod test {
    use super::*;
    use test_generator::test_resources;

    #[test_resources("crates/database/data/owl-functional/*.ofn")]
    async fn test_ofn_parser_format(resource: &str) -> Result<(), WebVowlStoreError> {
        let store = WebVOWLStore::default();
        store
            .insert_file(Path::new(&resource), false)
            .await
            .unwrap();
        assert_ne!(
            store.session.len().await.unwrap(),
            0,
            "Expected non-zero quads for: {}",
            resource
        );
        store.session.clear().await?;
        Ok(())
    }
    #[test_resources("crates/database/data/owl-rdf/*.owl")]
    async fn test_owl_parser_format(resource: &str) -> Result<(), WebVowlStoreError> {
        let store = WebVOWLStore::default();
        store
            .insert_file(Path::new(&resource), false)
            .await
            .unwrap();
        assert_ne!(
            store.session.len().await.unwrap(),
            0,
            "Expected non-zero quads for: {}",
            resource
        );
        store.session.clear().await?;
        Ok(())
    }
    #[test_resources("crates/database/data/owl-ttl/*.ttl")]
    async fn test_ttl_parser_format(resource: &str) -> Result<(), WebVowlStoreError> {
        let store = WebVOWLStore::default();
        store
            .insert_file(Path::new(&resource), false)
            .await
            .unwrap();
        assert_ne!(
            store.session.len().await.unwrap(),
            0,
            "Expected non-zero quads for: {}",
            resource
        );
        store.session.clear().await?;
        Ok(())
    }
    #[test_resources("crates/database/data/owl-xml/*.owx")]
    async fn test_owx_parser_format(resource: &str) -> Result<(), WebVowlStoreError> {
        let store = WebVOWLStore::default();
        store
            .insert_file(Path::new(&resource), false)
            .await
            .unwrap();
        assert_ne!(
            store.session.len().await.unwrap(),
            0,
            "Expected non-zero quads for: {}",
            resource
        );
        store.session.clear().await?;
        Ok(())
    }

    #[test_resources("crates/database/data/owl-functional/*.ofn")]
    async fn test_ofn_parser_stream(resource: &str) -> Result<(), WebVowlStoreError> {
        let mut out = vec![];
        let store = WebVOWLStore::default();
        store.insert_file(Path::new(&resource), false).await?;
        let mut results = parse_stream_to(store.session.stream().await?, ResourceType::OWL).await?;
        while let Some(result) = results.next().await {
            out.extend(result?);
        }

        assert_ne!(out.len(), 0, "Expected non-zero quads for: {}", resource);
        store.session.clear().await?;
        Ok(())
    }
    #[test_resources("crates/database/data/owl-rdf/*.owl")]
    async fn test_owl_parser_stream(resource: &str) -> Result<(), WebVowlStoreError> {
        let mut out = vec![];
        let store = WebVOWLStore::default();
        store.insert_file(Path::new(&resource), false).await?;
        let mut results = parse_stream_to(store.session.stream().await?, ResourceType::OWL).await?;
        while let Some(result) = results.next().await {
            out.extend(result?);
        }

        assert_ne!(out.len(), 0, "Expected non-zero quads for: {}", resource);
        store.session.clear().await?;
        Ok(())
    }
    #[test_resources("crates/database/data/owl-ttl/*.ttl")]
    async fn test_ttl_parser_stream(resource: &str) -> Result<(), WebVowlStoreError> {
        let mut out = vec![];
        let store = WebVOWLStore::default();
        store.insert_file(Path::new(&resource), false).await?;
        let mut results = parse_stream_to(store.session.stream().await?, ResourceType::OWL).await?;
        while let Some(result) = results.next().await {
            out.extend(result?);
        }

        assert_ne!(out.len(), 0, "Expected non-zero quads for: {}", resource);
        store.session.clear().await?;
        Ok(())
    }
    #[test_resources("crates/database/data/owl-xml/*.owx")]
    async fn test_owx_parser_stream(resource: &str) -> Result<(), WebVowlStoreError> {
        let mut out = vec![];
        let store = WebVOWLStore::default();
        store.insert_file(Path::new(&resource), false).await?;
        let mut results = parse_stream_to(store.session.stream().await?, ResourceType::OWL).await?;
        while let Some(result) = results.next().await {
            out.extend(result?);
        }

        assert_ne!(out.len(), 0, "Expected non-zero quads for: {}", resource);
        store.session.clear().await?;
        Ok(())
    }
}
