use serde::{Deserialize, Serialize};

// & Main pub struct (Head of the JSON)
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct OwlToWovlJSON {
    pub _comment: String,
    pub header: Header,
    pub namespace: Option<Vec<String>>,
    pub metrics: Option<Metrics>,
    pub class: Vec<Class>,
    pub classAttribute: Vec<ClassAttribute>,
    pub property: Vec<Property>,
    pub propertyAttribute: Vec<PropertyAttribute>,
}

// & Level 1
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Header {
    pub languages: Vec<String>,
    pub baseIris: Option<Vec<String>>,
    pub title: Label,
    pub iri: String,
    pub version: String,
    pub author: Vec<String>,
    pub description: Label,
    pub labels: Option<Label>,
    pub other: Option<Other>,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Metrics {
    pub classCount: u32,
    pub objectPropertyCount: u32,
    pub datatypePropertyCount: u32,
    pub individualCount: u32,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Class {
    pub id: String,
    pub r#type: String,
    pub label: Option<String>,
    pub intersection: Option<Vec<String>>,
    pub union: Option<Vec<String>>,
    pub disjointUnion: Option<Vec<String>>,
    pub complement: Option<Vec<String>>,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ClassAttribute {
    pub id: String,
    pub iri: Option<String>,
    pub baseIri: Option<String>,
    pub instances: Option<u32>,
    pub individuals: Option<Vec<Individuals>>,
    pub annotations: Option<Annotations>,
    pub label: Option<Label>,
    pub comment: Option<Label>,
    pub attributes: Option<Vec<String>>,
    pub superClasses: Option<Vec<String>>,
    pub subClasses: Option<Vec<String>>,
    pub complement: Option<Vec<String>>,
    pub union: Option<Vec<String>>,
    pub intersection: Option<Vec<String>>,
    pub equivalent: Option<Vec<String>>,
    pub disjointUnion: Option<Vec<String>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Property {
    pub id: String,
    pub r#type: String,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PropertyAttribute {
    pub iri: Option<String>,
    pub inverse: Option<String>,
    pub baseIri: Option<String>,
    pub range: String,
    pub annotations: Option<Annotations>,
    pub label: Option<Label>,
    pub superproperty: Option<Vec<String>>,
    pub domain: String,
    pub subproperty: Option<Vec<String>>,
    pub comment: Option<Label>,
    pub attributes: Option<Vec<String>>,
    pub id: String,
}

// & Level 2
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    // ^ pub struct for both labels, comments, title and description
    #[serde(rename = "IRI-based")]
    pub IRI_based: Option<String>,
    pub iriBased: Option<String>,
    pub undefined: Option<String>,
    pub en: Option<String>,
    pub de: Option<String>,
    pub fr: Option<String>,
    pub es: Option<String>,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Other {
    pub licence: Option<Vec<ILVT>>,
    pub creator: Option<Vec<ILVT>>,
    pub versionInfo: Option<Vec<ILVT>>,
    pub title: Option<Vec<ILVT>>,
    pub issued: Option<Vec<ILVT>>,
    pub seeAlso: Option<Vec<ILVT>>,
    pub homepage: Option<Vec<ILVT>>,
    pub depiction: Option<Vec<ILVT>>,
    pub priorVersion: Option<Vec<ILVT>>,
    pub date: Option<Vec<ILVT>>,
    pub contributor: Option<Vec<ILVT>>,
    pub incompatibleWith: Option<Vec<ILVT>>,
    pub rights: Option<Vec<ILVT>>,
    pub backwardCompatibleWith: Option<Vec<ILVT>>,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Annotations {
    pub isDefinedBy: Option<Vec<ILVT>>,
    pub versionInfo: Option<Vec<ILVT>>,
}
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Individuals {
    pub iri: String,
    pub baseIri: String,
    pub description: Option<Label>,
    pub labels: Option<Label>,
}

// & Level 3
#[derive(Serialize, Deserialize, Debug)]
pub struct ILVT {
    pub identifier: String,
    pub language: String,
    pub value: String,
    pub r#type: String,
}
