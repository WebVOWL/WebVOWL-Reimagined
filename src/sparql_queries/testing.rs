// pub const TESTING_QUERY: &str = r#"
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

pub const TESTING_QUERY: &str = r#"
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
            BIND("AnonymousClass" AS ?nodeType)
            OPTIONAL { ?id rdfs:label ?label }
        }
        UNION
        {
            # 2. Identify Intersections
            # Any node (usually blank) that is the subject of an intersectionOf list
            ?id owl:intersectionOf ?label .
            BIND("IntersectionOf" AS ?nodeType)
        }
        UNION
        {
            # 3. Identify Unions
            ?id owl:unionOf ?list .
            BIND("UnionOf" AS ?nodeType)
        }
        UNION
        {
            # 4. Identify Restrictions (Anonymous Classes in WebVOWL)
            ?id a owl:Restriction .
            BIND("AnonymousClass" AS ?nodeType)
            OPTIONAL { ?id rdfs:label ?label }
        }
        UNION
        {
            ?id owl:equivalentClass ?label
            BIND("EquivalentClass" AS ?nodeType)
        }
    }
    ORDER BY ?nodeType
    "#;
// UNION
// {
//     ?id a owl:Class
//     FILTER(!isIRI(?id))
//     BIND("blanknode" AS ?nodeType)
// }
// UNION
// {
//     # 2. Identify Intersections
//     # Any node (usually blank) that is the subject of an intersectionOf list
//     ?id owl:intersectionOf ?label .
//     BIND("IntersectionOf" AS ?nodeType)
// }
// UNION
// {
//     # 3. Identify Unions
//     ?id owl:unionOf ?list .
//     BIND("UnionOf" AS ?nodeType)
// }
// UNION
// {
//     # 4. Identify Restrictions (Anonymous Classes in WebVOWL)
//     ?id a owl:Restriction .
//     BIND("AnonymousClass" AS ?nodeType)
// }
// UNION
// {
//     ?id owl:equivalentClass ?label
//     BIND("EquivalentClass" AS ?nodeType)
// }
// # Edges
// UNION
// {
//     # 1. Identify RDF properties
//     ?id rdf:Property ?label
//     BIND("SubClass" AS ?nodeType)
// }
// UNION
// {
//     # 2. Identify subclasses
//     ?id rdfs:SubClassOf ?label
//     BIND("SubClass" AS ?nodeType)
// }
// UNION
// {
//     # 3. Identify datatypes
//     ?id rdfs:Datatype ?label
//     BIND("Datatype" AS ?nodeType)
// }
// UNION
// {
//     # 4. Identify OWL datatype properties
//     ?id owl:DatatypeProperty ?label
//     BIND("DatatypeProperty" AS ?nodeType)
// }
// UNION
// {
//     # 5. Identify OWL disjoint with
//     ?id owl:disjointWith ?label
//     BIND("disjointWith" AS ?nodeType)
// }
// UNION
// {
//     # 6. WIP Identify OWL deprecated properties
//     ?id owl:deprecated "true"^^<http://www.w3.org/2001/XMLSchema#boolean>
//     BIND("DeprecatedProperty" AS ?nodeType)
// }
