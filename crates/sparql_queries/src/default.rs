pub const DEFAULT_QUERY: &str = r#"
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
            ?id rdf:type owl:Ontology .
            BIND(owl:Ontology AS ?nodeType)
        }
        UNION
        {
            ?id rdf:type owl:ObjectProperty .
            BIND(owl:ObjectProperty AS ?nodeType)
        }
        UNION
        {
            ?id rdfs:domain ?label .
            BIND(rdfs:domain AS ?nodeType)
        }
        UNION
        {
            ?id rdfs:range ?label .
            BIND(rdfs:range AS ?nodeType)
        }
        UNION
        {
            # 2. Identify Intersections
            # Any node (usually blank) that is the subject of an intersectionOf list
            ?id owl:intersectionOf ?label .
            BIND(owl:intersectionOf AS ?nodeType)
        }
        UNION
        {
            # 3. Identify Unions
            ?id owl:unionOf ?label .
            BIND(owl:unionOf AS ?nodeType)
        }
        UNION
        {
            ?id owl:Restriction ?label .
            BIND(owl:Restriction AS ?nodeType)
        }
        UNION
        {
            ?id owl:equivalentClass ?label .
            BIND(owl:equivalentClass AS ?nodeType)
        }
        # Edges
        UNION
        {
            # 2. Identify subclasses
            ?id rdfs:subClassOf ?label .
            BIND(rdfs:subClassOf AS ?nodeType)
        }
        UNION {
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
        }
        UNION
        {
            # 3. Identify datatypes
            ?id rdfs:datatype ?label .
            BIND(owl:datatype AS ?nodeType)
        }
        UNION
        {
            # 4. Identify OWL datatype properties
            ?id owl:DatatypeProperty ?label .
            BIND(owl:DatatypeProperty AS ?nodeType)
        }
        UNION
        {
            # 5. Identify OWL disjoint with
            ?id owl:disjointWith ?label
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
    ORDER BY ?weight
    "#;
