use crate::serializers::formats::graph_display::GraphDisplayData;
use fluent_uri::Iri;
// use grapher::web::prelude::ElementType;
use rdf_fusion::model::{
    BlankNodeRef, TermRef, VariableRef,
    vocab::{rdf, rdfs, xsd},
};

// TODO: Use the structure of RDF Fusion's JSON serializer for this module
pub struct GraphDisplayDataSolutionSerializer;

impl GraphDisplayDataSolutionSerializer {
    pub fn serialize<'a>(
        buffer: &mut GraphDisplayData,
        solution: impl IntoIterator<Item = (VariableRef<'a>, TermRef<'a>)>,
    ) {
        for (variable, value) in solution {
            Self::write_term(buffer, value);
        }
    }

    fn write_term<'a>(buffer: &mut GraphDisplayData, term: TermRef<'a>) {
        // TODO: Collect errors and show to frontend
        // let iri = Iri::parse(value.as_str()).unwrap();
        // let path = iri.path();
        // let haystack = path.as_str().as_bytes();

        // A prefix determines what namespace the element belongs to
        let prefixes = [
            "22-rdf-syntax-ns", // RDF
            "rdf-schema",       // RDFS
            "owl",              // OWL
        ];

        // Doesnt work
        // let iri_type = memchr2("rdf", "owl", haystack);

        match term {
            TermRef::NamedNode(uri) => {
                // NOTE: Only supports RDF 1.1
                // TODO: Make a vocab for OWL
                let element_type = match uri {
                    rdf::ALT => {}
                    rdf::BAG => {}
                    rdf::FIRST => {}
                    rdf::HTML => {}
                    rdf::LANG_STRING => {}
                    rdf::LIST => {}
                    rdf::NIL => {}
                    rdf::OBJECT => {}
                    rdf::PREDICATE => {}
                    rdf::PROPERTY => {}
                    rdf::REST => {}
                    rdf::SEQ => {}
                    rdf::STATEMENT => {}
                    rdf::SUBJECT => {}
                    rdf::TYPE => {}
                    rdf::VALUE => {}
                    rdf::XML_LITERAL => {}
                    rdfs::CLASS => {}
                    rdfs::COMMENT => {}
                    rdfs::CONTAINER => {}
                    rdfs::CONTAINER_MEMBERSHIP_PROPERTY => {}
                    rdfs::DATATYPE => {}
                    rdfs::DOMAIN => {}
                    rdfs::IS_DEFINED_BY => {}
                    rdfs::LABEL => {}
                    rdfs::LITERAL => {}
                    rdfs::MEMBER => {}
                    rdfs::RANGE => {}
                    rdfs::RESOURCE => {}
                    rdfs::SEE_ALSO => {}
                    rdfs::SUB_CLASS_OF => {}
                    rdfs::SUB_PROPERTY_OF => {}
                    _ => {
                        // Visualization of this element is not supported
                    }
                };
            }
            TermRef::BlankNode(bnode) => {
                // REVIEW: Test if this works
                if let Some(_) = bnode.unique_id() {
                    // Anonymous node
                } else {
                    // Named node
                }
            }
            TermRef::Literal(literal) => {}
        }
    }

    // fn find_prefix(iri: Iri<&str>) -> NodeType {
    //     let path = iri.path();

    //     let iri_type = memchr2("rdf", "owl", haystack);
    // }
}
