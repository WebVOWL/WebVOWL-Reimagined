use crate::serializers::formats::graph_display::GraphDisplayData;
use fluent_uri::Iri;
use grapher::web::prelude::NodeType;
use memchr::{memchr, memchr2, memchr3};
use rdf_fusion::model::{TermRef, VariableRef};

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

    /// Edges
    fn write_term<'a>(buffer: &mut GraphDisplayData, term: TermRef<'a>) {
        match term {
            TermRef::NamedNode(uri) => {
                // TODO: Collect errors and show to frontend
                let iri = Iri::parse(uri.as_str()).unwrap();
            }
            TermRef::BlankNode(bnode) => {}
            TermRef::Literal(literal) => {}
            #[cfg(feature = "sparql-12")]
            TermRef::Triple(triple) => {}
        }
    }

    // fn get_nodetype(iri: Iri<&str>) -> NodeType {
    //     let path = iri.path();
    //     let haystack = path.as_str().as_bytes();
    //     let iri_types = ["rdf", "owl"];

    //     let iri_type = memchr2("rdf", "owl", haystack);
    // }

    // // TODO: Make one for each supported type, e.g. RDF, RDDS, OWL, SKOS, etc
    // fn try_get_rdf_nodetype(fragment: &str) -> Option<NodeType> {}
}
