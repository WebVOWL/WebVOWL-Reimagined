use crate::serializers::formats::graph_display::GraphDisplayData;
use crate::vocab::owl;
use fluent_uri::Iri;
use grapher::web::prelude::{
    ElementType, GenericNode, GenericType, OwlEdge, OwlNode, OwlType, RdfEdge, RdfNode, RdfType,
    RdfsEdge, RdfsNode, RdfsType,
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

        let result = match term {
            TermRef::BlankNode(bnode) => {
                // REVIEW: Test if this works
                if let Some(_) = bnode.unique_id() {
                    // Anonymous individual
                    buffer
                        .elements
                        .push(ElementType::Owl(OwlNode::AnonymousClass));
                    Ok(())
                } else {
                    // RDFS Named individual
                    Ok(())
                }
            }
            TermRef::Literal(literal) => {
                info!("Is literal: '{}'", literal.value());
                // match literal {

                // }
            }
            TermRef::NamedNode(uri) => {
                // NOTE: Only supports RDF 1.1
                match uri {
                    // ----------- RDF ----------- //

                    // rdf::ALT => {}
                    // rdf::BAG => {}
                    // rdf::FIRST => {}
                    // rdf::HTML => {}
                    // rdf::LANG_STRING => {}
                    // rdf::LIST => {}
                    // rdf::NIL => {}
                    // rdf::OBJECT => {}
                    // rdf::PREDICATE => {}
                    rdf::PROPERTY => {
                        buffer
                            .elements
                            .push(ElementType::Rdf(RdfType::Edge(RdfEdge::RdfProperty)));
                        Ok(())
                    }
                    // rdf::REST => {}
                    // rdf::SEQ => {}
                    // rdf::STATEMENT => {}
                    // rdf::SUBJECT => {}
                    // rdf::TYPE => {}
                    // rdf::VALUE => {}
                    // rdf::XML_LITERAL => {}

                    // ----------- RDFS ----------- //
                    rdfs::CLASS => {
                        buffer
                            .elements
                            .push(ElementType::Rdfs(RdfsType::Node(RdfsNode::Class)));
                        Ok(())
                    }
                    // rdfs::COMMENT => {}
                    // rdfs::CONTAINER => {}
                    // rdfs::CONTAINER_MEMBERSHIP_PROPERTY => {}
                    rdfs::DATATYPE => {
                        buffer
                            .elements
                            .push(ElementType::Rdfs(RdfsType::Edge(RdfsEdge::Datatype)));
                        Ok(())
                    }
                    rdfs::DOMAIN => {}
                    // rdfs::IS_DEFINED_BY => {}
                    rdfs::LABEL => {}
                    rdfs::LITERAL => {
                        buffer
                            .elements
                            .push(ElementType::Rdfs(RdfsType::Node(RdfsNode::Literal)));
                        Ok(())
                    }
                    // rdfs::MEMBER => {}
                    rdfs::RANGE => {}
                    // rdfs::RESOURCE => {}
                    // rdfs::SEE_ALSO => {}
                    rdfs::SUB_CLASS_OF => {
                        buffer
                            .elements
                            .push(ElementType::Rdfs(RdfsType::Edge(RdfsEdge::SubclassOf)));
                        Ok(())
                    }
                    // rdfs::SUB_PROPERTY_OF => {},

                    // ----------- OWL 2 ----------- //

                    // owl::ALL_DIFFERENT => {},
                    // owl::ALL_DISJOINT_CLASSES => {},
                    // owl::ALL_DISJOINT_PROPERTIES => {},
                    owl::ALL_VALUES_FROM => {}
                    // owl::ANNOTATED_PROPERTY => {},
                    // owl::ANNOTATED_SOURCE => {},
                    // owl::ANNOTATED_TARGET => {},
                    // owl::ANNOTATION => {},
                    // owl::ANNOTATION_PROPERTY => {},
                    // owl::ASSERTION_PROPERTY => {},
                    // owl::ASYMMETRIC_PROPERTY => {},
                    // owl::AXIOM => {},
                    // owl::BACKWARD_COMPATIBLE_WITH => {},
                    // owl::BOTTOM_DATA_PROPERTY => {},
                    // owl::BOTTOM_OBJECT_PROPERTY => {},
                    owl::CARDINALITY => {}
                    owl::CLASS => {
                        buffer
                            .elements
                            .push(ElementType::Owl(OwlType::Node(OwlNode::Class)));
                        Ok(())
                    }
                    owl::COMPLEMENT_OF => {}
                    owl::DATATYPE_COMPLEMENT_OF => {}
                    owl::DATATYPE_PROPERTY => {
                        buffer
                            .elements
                            .push(ElementType::Owl(OwlType::Edge(OwlEdge::DatatypeProperty)));
                        Ok(())
                    }
                    // owl::DATA_RANGE => {}
                    // owl::DEPRECATED => {}
                    owl::DEPRECATED_CLASS => {
                        buffer
                            .elements
                            .push(ElementType::Owl(OwlType::Node(OwlNode::DeprecatedClass)));
                        Ok(())
                    }
                    owl::DEPRECATED_PROPERTY => {
                        buffer
                            .elements
                            .push(ElementType::Owl(OwlType::Edge(OwlEdge::DeprecatedProperty)));
                        Ok(())
                    }
                    // owl::DIFFERENT_FROM => {}
                    // owl::DISJOINT_UNION_OF => {}
                    owl::DISJOINT_WITH => {
                        buffer
                            .elements
                            .push(ElementType::Owl(OwlType::Edge(OwlEdge::DisjointWith)));
                        Ok(())
                    }
                    // owl::DISTINCT_MEMBERS => {}
                    owl::EQUIVALENT_CLASS => {
                        buffer
                            .elements
                            .push(ElementType::Owl(OwlType::Node(OwlNode::EquivalentClass)));
                        Ok(())
                    }
                    // owl::EQUIVALENT_PROPERTY => {}
                    // owl::FUNCTIONAL_PROPERTY => {}
                    // owl::HAS_KEY => {}
                    // owl::HAS_SELF => {}
                    // owl::HAS_VALUE => {}
                    // owl::IMPORTS => {}
                    // owl::INCOMPATIBLE_WITH => {}
                    owl::INTERSECTION_OF => {}
                    owl::INVERSE_FUNCTIONAL_PROPERTY => {}
                    owl::INVERSE_OF => {}
                    owl::IRREFLEXIVE_PROPERTY => {}
                    // owl::MAX_CARDINALITY => {}
                    // owl::MAX_QUALIFIED_CARDINALITY => {}
                    // owl::MEMBERS => {}
                    // owl::MIN_CARDINALITY => {}
                    // owl::MIN_QUALIFIED_CARDINALITY => {}
                    owl::NAMED_INDIVIDUAL => {}
                    // owl::NEGATIVE_PROPERTY_ASSERTION => {}
                    owl::NOTHING => {}
                    owl::OBJECT_PROPERTY => {
                        buffer
                            .elements
                            .push(ElementType::Owl(OwlType::Edge(OwlEdge::ObjectProperty)));
                        Ok(())
                    } // owl::ONE_OF => {}
                    // owl::ONTOLOGY => {}
                    // owl::ONTOLOGY_PROPERTY => {}
                    // owl::ON_CLASS => {}
                    // owl::ON_DATARANGE => {}
                    // owl::ON_DATATYPE => {}
                    // owl::ON_PROPERTIES => {}
                    // owl::ON_PROPERTY => {}
                    // owl::PRIOR_VERSION => {}
                    // owl::PROPERTY_CHAIN_AXIOM => {}
                    // owl::PROPERTY_DISJOINT_WITH => {}
                    // owl::QUALIFIED_CARDINALITY => {}
                    owl::REFLEXIVE_PROPERTY => {}
                    // owl::RESTRICTION => {}
                    // owl::SAME_AS => {}
                    // owl::SOME_VALUES_FROM => {}
                    // owl::SOURCE_INDIVIDUAL => {}
                    owl::SYMMETRIC_PROPERTY => {}
                    // owl::TARGET_INDIVIDUAL => {}
                    // owl::TARGET_VALUE => {}
                    owl::THING => {
                        buffer
                            .elements
                            .push(ElementType::Owl(OwlType::Node(OwlNode::Thing)));
                        Ok(())
                    }
                    // owl::TOP_DATA_PROPERTY => {}
                    // owl::TOP_OBJECT_PROPERTY => {}
                    // owl::TRANSITIVE_PROPERTY => {}
                    owl::UNION_OF => {}
                    // owl::VERSION_INFO => {}
                    // owl::VERSION_IRI => {}
                    // owl::WITH_RESTRICTIONS => {}
                    _ => {
                        // Visualization of this element is not supported
                        info!("Visualization of element '{}' is not supported", term);
                    }
                };
            }
        };
    }

    // fn find_prefix(iri: Iri<&str>) -> NodeType {
    //     let path = iri.path();

    //     let iri_type = memchr2("rdf", "owl", haystack);
    // }
}
