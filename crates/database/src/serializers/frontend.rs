use crate::serializers::formats::graph_display::GraphDisplayData;
use fluent_uri::Iri;
use grapher::web::prelude::{
    ElementType, GenericNode, GenericType, RdfEdge, RdfNode, RdfType, RdfsEdge, RdfsNode, RdfsType,
};
use log::info;
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

    // fn write_predicate<'a>(buffer: &mut GraphDisplayData, term: TermRef<'a>) {
    //     match term {}
    // }

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
                    // rdf::ALT => {}
                    // rdf::BAG => {}
                    // rdf::FIRST => {}
                    // rdf::HTML => {}
                    // rdf::LANG_STRING => {}
                    // rdf::LIST => {}
                    // rdf::NIL => {}
                    // rdf::OBJECT => {}
                    // rdf::PREDICATE => {}
                    rdf::PROPERTY => buffer
                        .nodes
                        .push(ElementType::Rdf(RdfType::Edge(RdfEdge::RdfProperty))),
                    // rdf::REST => {}
                    // rdf::SEQ => {}
                    // rdf::STATEMENT => {}
                    // rdf::SUBJECT => {}
                    // rdf::TYPE => {}
                    // rdf::VALUE => {}
                    // rdf::XML_LITERAL => {}
                    rdfs::CLASS => {
                        buffer
                            .nodes
                            .push(ElementType::Rdfs(RdfsType::Node(RdfsNode::Class)));
                    }
                    // rdfs::COMMENT => {}
                    // rdfs::CONTAINER => {}
                    // rdfs::CONTAINER_MEMBERSHIP_PROPERTY => {}
                    rdfs::DATATYPE => buffer
                        .nodes
                        .push(ElementType::Rdfs(RdfsType::Edge(RdfsEdge::Datatype))),
                    rdfs::DOMAIN => {}
                    // rdfs::IS_DEFINED_BY => {}
                    rdfs::LABEL => {}
                    rdfs::LITERAL => buffer
                        .nodes
                        .push(ElementType::Rdfs(RdfsType::Node(RdfsNode::Literal))),
                    // rdfs::MEMBER => {}
                    rdfs::RANGE => {}
                    // rdfs::RESOURCE => {}
                    // rdfs::SEE_ALSO => {}
                    rdfs::SUB_CLASS_OF => buffer
                        .nodes
                        .push(ElementType::Rdfs(RdfsType::Edge(RdfsEdge::SubclassOf))),
                    rdfs::SUB_PROPERTY_OF => {}
                    _ => {
                        // Visualization of this element is not supported
                        info!("Visualization of element '{}' is not supported", term)
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
            TermRef::Literal(literal) => {
                info!("Is literal: '{}'", literal.value())
                // match literal {

                // }
            }
        }
    }

    // fn find_prefix(iri: Iri<&str>) -> NodeType {
    //     let path = iri.path();

    //     let iri_type = memchr2("rdf", "owl", haystack);
    // }
}
