use crate::serializers::vowl_extract::{Node, VowlExtract};
use futures::StreamExt;
use rdf_fusion::{execution::results::QueryResults, store::Store};
use webvowl_parser::errors::WebVowlStoreError;

pub struct NewSerializer<A> {
    extract: VowlExtract<A>,
    query: String,
}

impl<A: Clone + Eq> Default for NewSerializer<A> {
    fn default() -> Self {
        Self {
            extract: VowlExtract::default(),
            query: DEFAULT_QUERY.to_string(),
        }
    }
}
impl NewSerializer<String> {
    pub async fn serialize(
        &mut self,
        store: Store,
    ) -> Result<VowlExtract<String>, WebVowlStoreError> {
        if let QueryResults::Solutions(mut solutions) = store.query(&self.query).await? {
            while let Some(solution) = solutions.next().await {
                let solution = solution?;
                let Some(id_term) = solution.get("id") else {
                    continue;
                };
                let Some(node_type_term) = solution.get("nodeType") else {
                    continue;
                };
                let triple = (
                    id_term.to_string(),
                    node_type_term.to_string(),
                    solution.get("label").map(|term| term.to_string()),
                );
                let t = triple.clone();
                if t.2.is_some() && !is_iri(t.2.as_ref().unwrap().as_str()) {
                    self.extract.resolve(&t.2.clone().unwrap());
                }
                let (id_value, node_type_raw, label_value) = t;
                let id = self.extract.insert(id_value);
                let node_type_clean = node_type_raw.trim_matches('"').to_string();
                let node_type: Node<usize> = Node::from_str(node_type_clean.as_str(), id)?;
                println!(
                    "id: {}, node_type: {}, label: {:?}",
                    id, node_type, label_value
                );
            }
        }
        for (iri, id) in self.extract.irivec.iter().enumerate() {
            println!("iri: {}, id: {}", iri, id);
        }
        Ok(std::mem::take(&mut self.extract))
    }
}
pub fn is_iri(s: &str) -> bool {
    s.starts_with("<") && s.ends_with(">")
}

pub const DEFAULT_IRI: &str = "<http://www.example.com/iri#";
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
            BIND("Intersection" AS ?nodeType)
        }
        UNION
        {
            # 3. Identify Unions
            ?id owl:unionOf ?list .
            BIND("Union" AS ?nodeType)
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

    }
    ORDER BY ?nodeType
    "#;
