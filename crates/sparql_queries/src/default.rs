//Nodes

/// Named Classes query
pub const NAMED_CLASS: &str = r#"{
                ?id a owl:Class .
                FILTER(isIRI(?id))
                BIND(owl:Class AS ?nodeType)
            }"#;
/// Anonymous Classes query
pub const ANONYMOUS_CLASS: &str = r#"{
                ?id a owl:Class .
                FILTER(!isIRI(?id))
                BIND("blanknode" AS ?nodeType)
            }"#;
/// External Classes query
pub const EXTERNAL_CLASS: &str = r#"{
                # SPARQL query not defined
                BIND(<http://example.org/nothing> AS ?id)
                BIND(<http://example.org/nothing> AS ?nodeType)
                BIND("" AS ?label)
                FILTER(false)
            }"#;
/// Deprecated Classes query
pub const DEPRECATED_CLASS: &str = r#"{
                ?id a owl:DeprecatedClass .
                BIND(owl:DeprecatedClass AS ?nodeType)
            }"#;
/// Equivalent Classes query
pub const EQUIVALENT_CLASS: &str = r#"{
                ?id owl:equivalentClass ?target
                BIND(owl:equivalentClass AS ?nodeType)
            }"#;

/// Thing query
pub const THING: &str = r#"{
                ?id a owl:Thing .
                BIND(owl:Thing AS ?nodeType)
            }"#;

/// RDFS Classes query
pub const RDFS_CLASS: &str = r#"{
                ?id a rdfs:Class .
                FILTER(?id != owl:Class)
                BIND(rdfs:Class AS ?nodeType)
            }"#;

/// RDFS Resources query
pub const RDFS_RESOURCE: &str = r#"{
                ?id a rdf:Resource.
                FILTER(isIRI(?id) || isBlank(?id))
                BIND(rdfs:Resource AS ?nodeType)
            }"#;

/// RDFS Literals query
pub const RDFS_LITERAL: &str = r#"{
                ?id rdfs:label ?label.
                FILTER(isLiteral(?label))
                BIND(rdfs:Literal AS ?nodeType)
            }"#;

/// RDFS Datatypes label query
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

/// Intersection Of query
pub const INTERSECTION_OF: &str = r#"{
                ?id owl:intersectionOf ?target .
                BIND(owl:intersectionOf AS ?nodeType)
            }"#;

/// Union Of query
pub const UNION_OF: &str = r#"{
                ?id owl:unionOf ?target .
                BIND(owl:unionOf AS ?nodeType)
            }"#;

/// Complement Of query
pub const COMPLEMENT: &str = r#"{
                ?id owl:complementOf ?target .
                BIND(owl:complementOf AS ?nodeType)
            }"#;

/// Disjoint Union query
pub const DISJOINT_UNION: &str = r#"{
                ?id owl:disjointUnionOf ?target .
                BIND(owl:disjointUnionOf AS ?nodeType)
            }"#;

//Edges

/// Object Properties query
pub const OBJECTPROPERTY: &str = r#"{
                ?id a owl:ObjectProperty .
                OPTIONAL {?id rdfs:range ?target}
                BIND(owl:ObjectProperty AS ?nodeType)
            }"#;

/// Datatype Properties query
pub const DATATYPEPROPERTY: &str = r#"{
                ?id a owl:DatatypeProperty .
                OPTIONAL {?id rdfs:range ?target}
                BIND(owl:DatatypeProperty AS ?nodeType)
            }"#;

/// Subclass Of query
pub const SUBCLASSOF: &str = r#"{
                ?id rdfs:subClassOf ?target .
                BIND(rdfs:subClassOf AS ?nodeType)
            }"#;

/// Inverse Of query
pub const INVERSEOF: &str = r#"{
                ?id owl:inverseOf ?target .
                BIND(owl:inverseOf AS ?nodeType)
            }"#;

/// Disjoint With query
pub const DISJOINTWITH: &str = r#"{
                ?id owl:disjointWith ?target.
                BIND(owl:disjointWith AS ?nodeType)
            }"#;
/// Collections flattening query
pub const COLLECTIONS: &str = 
            r#"{
                ?target ?nodeType ?intermediate .
                ?intermediate rdf:first ?firstItem .
                ?intermediate rdf:rest*/rdf:first ?id .
                FILTER(?nodeType IN (
                    owl:intersectionOf, 
                    owl:unionOf, 
                    owl:oneOf,
                    owl:disjointUnionOf
                ))

                # 6. Safety: Remove nil to avoid phantom edges
                # FILTER(?label != rdf:nil)
            }"#;

/// RDF Properties query
pub const RDFPROPERTY: &str = r#"{
                ?id a rdf:Property .
                FILTER NOT EXISTS {?id a owl:ObjectProperty}
                FILTER NOT EXISTS {?id a owl:DatatypeProperty}
                OPTIONAL {?id rdfs:range ?target}
                BIND(rdf:Property AS ?nodeType)
            }"#;

/// Deprecated Properties query
pub const DEPRECATEDPROPERTY: &str = r#"{
                ?id a owl:DeprecatedProperty .
                OPTIONAL {?id rdfs:range ?target}
                BIND(owl:DeprecatedProperty AS ?nodeType)
            }"#;

/// External Properties query
pub const EXTERNALPROPERTY: &str = r#"{
                # SPARQL query not defined
                BIND(<http://example.org/nothing> AS ?id)
                BIND(<http://example.org/nothing> AS ?nodeType)
                BIND("" AS ?label)
                FILTER(false)
            }"#;

/// Values From query
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
