pub const NAMED_CLASS: &str = 
            r#"{
                ?id a owl:Class .
                FILTER(isIRI(?id))
                BIND(owl:Class AS ?nodeType)
            }"#;

pub const EXTERNAL_CLASS: &str = 
            r#"{
                # SPARQL query not defined
                BIND(<http://example.org/nothing> AS ?id)
                BIND(<http://example.org/nothing> AS ?nodeType)
                BIND("" AS ?label)
                FILTER(false)
            }"#;

pub const DEPRECATED_CLASS: &str = 
            r#"{
                ?id a owl:DeprecatedClass .
                BIND(owl:DeprecatedClass AS ?nodeType)
            }"#;

pub const EQUIVALENT_CLASS: &str = 
            r#"{
                ?id owl:equivalentClass ?target
                BIND(owl:equivalentClass AS ?nodeType)
            }"#;

pub const ANONYMOUS_CLASS: &str = 
            r#"{
                FILTER(!isIRI(?id))
                BIND("blanknode" AS ?nodeType)
            }"#;

pub const THING: &str = 
            r#"{
                ?id a owl:Thing .
                BIND(owl:Thing AS ?nodeType)
            }"#;

pub const RDFS_CLASS: &str = 
            r#"{
                ?id a rdfs:Class .
                FILTER(?id != owl:Class)
                BIND(rdfs:Class AS ?nodeType)
            }"#;

pub const RDFS_RESOURCE: &str = 
            r#"{
                ?id a rdfs:Resource .
                BIND(rdfs:Resource AS ?nodeType)
            }"#;

pub const RDFS_LITERAL: &str = 
            r#"{
                ?id rdfs:label ?target.
                FILTER(isLiteral(?target))
                BIND(rdfs:Literal AS ?nodeType)
            }"#;

pub const INTERSECTION_OF: &str = 
            r#"{
                ?id owl:intersectionOf ?target .
                BIND(owl:intersectionOf AS ?nodeType)
            }"#;

pub const UNION_OF: &str = 
            r#"{
                ?id owl:unionOf ?list .
                BIND(owl:unionOf AS ?nodeType)
            }"#;

pub const COMPLEMENT: &str = 
            r#"{
                ?id owl:complementOf ?target .
                BIND(owl:complementOf AS ?nodeType)
            }"#;

pub const DISJOINT_UNION: &str = 
            r#"{
                ?id owl:disjointUnionOf ?list .
                BIND(owl:disjointUnionOf AS ?nodeType)
            }"#;

pub const OBJECTPROPERTY: &str = 
            r#"{
                # SPARQL query not defined
                BIND(<http://example.org/nothing> AS ?id)
                BIND(<http://example.org/nothing> AS ?nodeType)
                BIND("" AS ?label)
                FILTER(false)
            }"#;

pub const DATATYPEPROPERTY: &str = 
            r#"{
                # SPARQL query not defined
                BIND(<http://example.org/nothing> AS ?id)
                BIND(<http://example.org/nothing> AS ?nodeType)
                BIND("" AS ?label)
                FILTER(false)
            }"#;

pub const SUBCLASSOF: &str = 
            r#"{
                ?id rdfs:subClassOf ?target .
                BIND(rdfs:subClassOf AS ?nodeType)
            }"#;

pub const INVERSEOF: &str = 
            r#"{
                # SPARQL query not defined
                BIND(<http://example.org/nothing> AS ?id)
                BIND(<http://example.org/nothing> AS ?nodeType)
                BIND("" AS ?label)
                FILTER(false)
            }"#;

pub const DISJOINTWITH: &str = 
            r#"{
                # SPARQL query not defined
                BIND(<http://example.org/nothing> AS ?id)
                BIND(<http://example.org/nothing> AS ?nodeType)
                BIND("" AS ?label)
                FILTER(false)
            }"#;
pub const COLLECTIONS: &str = 
            r#"{
                # BRIDGE: Start at the Named Class, jump to the intermediate node
                ?id ?connector ?intermediateNode .
                FILTER(isIRI(?id)) 

                # Match the logic property (unionOf, etc) on the intermediate node
                ?intermediateNode ?nodeType ?blanknode .

                # Flatten the list from the blanknode
                ?blanknode rdf:rest*/rdf:first ?target .

                # Filter for Logic Types
                FILTER(?nodeType IN (owl:intersectionOf, owl:unionOf, owl:oneOf, owl:disjointUnionOf, owl:disjointWith))
                FILTER(?target != rdf:nil)
            }"#;

pub const RDFPROPERTY: &str = 
            r#"{
                # SPARQL query not defined
                BIND(<http://example.org/nothing> AS ?id)
                BIND(<http://example.org/nothing> AS ?nodeType)
                BIND("" AS ?label)
                FILTER(false)
            }"#;

pub const DEPRECATEDPROPERTY: &str = 
            r#"{
                ?id owl:deprecated ?target .
                BIND(owl:deprecated AS ?nodeType)
            }"#;

pub const EXTERNALPROPERTY: &str = 
            r#"{
                # SPARQL query not defined
                BIND(<http://example.org/nothing> AS ?id)
                BIND(<http://example.org/nothing> AS ?nodeType)
                BIND("" AS ?label)
                FILTER(false)
            }"#;

pub const VALUESFROM: &str = 
            r#"{
                # SPARQL query not defined
                BIND(<http://example.org/nothing> AS ?id)
                BIND(<http://example.org/nothing> AS ?nodeType)
                BIND("" AS ?label)
                FILTER(false)
            }"#;
