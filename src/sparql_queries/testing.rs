pub const TESTING_QUERY: &str = r#" PREFIX owl: <http://www.w3.org/2002/07/owl#>
    PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
    PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
    PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
    PREFIX webvowl: <http://www.example.com/iri#>

    SELECT ?id ?nodeType ?target ?label
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
            ?id owl:intersectionOf ?target .
            BIND(owl:intersectionOf AS ?nodeType)
        }
        UNION
        {
            # 3. Identify Unions
            ?id owl:unionOf ?list .
            BIND(owl:unionOf AS ?nodeType)
        }
        UNION
        {
            ?id a owl:Restriction .
            BIND(owl:Restriction AS ?nodeType)
        }
        UNION
        {
            ?id owl:equivalentClass ?target
            BIND(owl:equivalentClass AS ?nodeType)
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
            ?id rdfs:subClassOf ?target
            BIND(rdfs:subClassOf AS ?nodeType)
        }
        UNION
        {
            # 3. Identify datatypes
            ?id rdfs:datatype ?target
            BIND(owl:datatype AS ?nodeType)
        }
        UNION
        {
            # 4. Identify OWL datatype properties
            ?id owl:DatatypeProperty ?target
            BIND(owl:DatatypeProperty AS ?nodeType)
        }
        UNION
        {
            # 5. Identify OWL disjoint with
            ?id owl:disjointWith ?target
            BIND(owl:disjointWith AS ?nodeType)
        }
        UNION
        {
            # 6. WIP Identify OWL deprecated properties
            ?id owl:deprecated "true"^^<http://www.w3.org/2001/XMLSchema#boolean>
            BIND("DeprecatedProperty" AS ?nodeType)
        }
        BIND(
            IF(?nodeType = owl:Class, 1, 2)
            AS ?weight)
    }
    ORDER BY ?weight"#;

// r#"
//     PREFIX owl: <http://www.w3.org/2002/07/owl#>
//     PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>

//     SELECT ?id ?nodeType ?label
//         {
//             # 1. Identify Named Classes
//             ?id a owl:Class .
//             FILTER(isIRI(?id))
//             BIND(owl:Class AS ?nodeType)
//             OPTIONAL { ?id rdfs:label ?label }
//         }
//     ORDER BY ?nodeType
//     "#;
