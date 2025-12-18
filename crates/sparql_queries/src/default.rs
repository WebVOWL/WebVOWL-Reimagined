//Nodes
pub const NAMED_CLASS: &str = r#"{
                ?id a owl:Class .
                FILTER(isIRI(?id))
                BIND(owl:Class AS ?nodeType)
            }"#;
pub const ANONYMOUS_CLASS: &str = r#"{
                ?id a owl:Class .
                FILTER(!isIRI(?id))
                BIND("blanknode" AS ?nodeType)
            }"#;
pub const EXTERNAL_CLASS: &str = r#"{
                # SPARQL query not defined
                BIND(<http://example.org/nothing> AS ?id)
                BIND(<http://example.org/nothing> AS ?nodeType)
                BIND("" AS ?label)
                FILTER(false)
            }"#;

pub const DEPRECATED_CLASS: &str = r#"{
                ?id a owl:DeprecatedClass .
                BIND(owl:DeprecatedClass AS ?nodeType)
            }"#;

pub const EQUIVALENT_CLASS: &str = r#"{
                ?id owl:equivalentClass ?target
                BIND(owl:equivalentClass AS ?nodeType)
            }"#;

pub const THING: &str = r#"{
                ?id a owl:Thing .
                BIND(owl:Thing AS ?nodeType)
            }"#;

pub const RDFS_CLASS: &str = r#"{
                ?id a rdfs:Class .
                FILTER(?id != owl:Class)
                BIND(rdfs:Class AS ?nodeType)
            }"#;

pub const RDFS_RESOURCE: &str = r#"{
                ?id a rdf:Resource.
                FILTER(isIRI(?id) || isBlank(?id))
                BIND(rdfs:Resource AS ?nodeType)
            }"#;

pub const RDFS_LITERAL: &str = r#"{
                ?id rdfs:label ?label.
                FILTER(isLiteral(?label))
                BIND(rdfs:Literal AS ?nodeType)
            }"#;

pub const RDFS_DATATYPE: &str = r#"{
                VALUES ?dt{
                    rdf:HTML rdf:PlainLiteral rdf:XMLLiteral xsd:anySimpleType 
                    xsd:anyURI xsd:base64Binary xsd:boolean xsd:byte xsd:date 
                    xsd:dateTime xsd:decimal xsd:double xsd:duration xsd:ENTITY
                    xsd:float xsd:gDay xsd:gMonth xsd:gMonthDay xsd:gYear
                    xsd:gYearMonth xsd:hexBinary xsd:ID xsd:IDREF xsd:int xsd:integer
                    xsd:language xsd:long xsd:Name xsd:NCName xsd:negativeInteger
                    xsd:NMTOKEN xsd:nonNegativeInteger xsd:nonPositiveInteger 
                    xsd:normalizedString xsd:NOTATION xsd:positiveInteger xsd:QName 
                    xsd:short xsd:string xsd:time xsd:token xsd:unsignedByte 
                    xsd:unsignedInt xsd:unsignedLong xsd:unsignedShort
                }
                BIND(rdfs:Datatype AS ?nodeType)
                BIND(?dt AS ?labelIRI)
                BIND(?dt AS ?label)
            }"#;

pub const INTERSECTION_OF: &str = r#"{
                ?id owl:intersectionOf ?target .
                BIND(owl:intersectionOf AS ?nodeType)
            }"#;

pub const UNION_OF: &str = r#"{
                ?id owl:unionOf ?target .
                BIND(owl:unionOf AS ?nodeType)
            }"#;

pub const COMPLEMENT: &str = r#"{
                ?id owl:complementOf ?target .
                BIND(owl:complementOf AS ?nodeType)
            }"#;

pub const DISJOINT_UNION: &str = r#"{
                ?id owl:disjointUnionOf ?target .
                BIND(owl:disjointUnionOf AS ?nodeType)
            }"#;

//Edges
pub const OBJECTPROPERTY: &str = r#"{
                ?id a owl:ObjectProperty .
                OPTIONAL {?id rdfs:range ?target}
                BIND(owl:ObjectProperty AS ?nodeType)
            }"#;

pub const DATATYPEPROPERTY: &str = r#"{
                ?id a owl:DatatypeProperty .
                OPTIONAL {?id rdfs:range ?target}
                BIND(owl:DatatypeProperty AS ?nodeType)
            }"#;

pub const SUBCLASSOF: &str = r#"{
                ?id rdfs:subClassOf ?target .
                BIND(rdfs:subClassOf AS ?nodeType)
            }"#;

pub const INVERSEOF: &str = r#"{
                ?id owl:inverseOf ?target .
                BIND(owl:inverseOf AS ?nodeType)
            }"#;

pub const DISJOINTWITH: &str = r#"{
                ?id owl:disjointWith ?target.
                BIND(owl:disjointWith AS ?nodeType)
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

pub const RDFPROPERTY: &str = r#"{
                ?id a rdf:Property .
                FILTER NOT EXISTS {?id a owl:ObjectProperty}
                FILTER NOT EXISTS {?id a owl:DatatypeProperty}
                OPTIONAL {?id rdfs:range ?target}
                BIND(rdf:Property AS ?nodeType)
            }"#;

pub const DEPRECATEDPROPERTY: &str = r#"{
                ?id a owl:DeprecatedProperty .
                OPTIONAL {?id rdfs:range ?target}
                BIND(owl:DeprecatedProperty AS ?nodeType)
            }"#;

pub const EXTERNALPROPERTY: &str = r#"{
                # SPARQL query not defined
                BIND(<http://example.org/nothing> AS ?id)
                BIND(<http://example.org/nothing> AS ?nodeType)
                BIND("" AS ?label)
                FILTER(false)
            }"#;

pub const VALUESFROM: &str = r#"{
                {
                    ?id owl:someValuesFrom ?target .
                }
                UNION
                {
                    ?id owl:allValuesFrom ?target .
                }
                BIND("ValuesFrom" AS ?nodeType)
            }"#;
