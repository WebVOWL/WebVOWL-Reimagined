pub const DEFAULT_QUERY: &str = r#"
    PREFIX owl: <http://www.w3.org/2002/07/owl#>
    PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
    PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
    PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>
    PREFIX webvowl: <http://www.example.com/iri#>
    PREFIX xml: <http://www.w3.org/XML/1998/namespace>

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
            # 4. Identify Restrictions (Anonymous Classes in WebVOWL)
            ?id a owl:Restriction .
            BIND(owl:Restriction AS ?nodeType)
        }
        UNION
        {
            ?id a owl:Class
            FILTER(!isIRI(?id))
            BIND("blanknode" AS ?nodeType)
        }
        UNION
        {   
            # COMPLEMENT 
            ?id owl:complementOf ?target .
            BIND(owl:complementOf AS ?nodeType)
        }
        UNION
        {   
            # DEPRECATED CLASS
            ?id a owl:DeprecatedClass .
            BIND(owl:DeprecatedClass AS ?nodeType)
        }
        UNION
        {   
            #EQUILVALENT CLASS
            ?id owl:equivalentClass ?target
            BIND(owl:equivalentClass AS ?nodeType)
        }
        UNION
        {   
            # DISJONT UNION
            ?id owl:disjointUnionOf ?list .
            BIND(owl:disjointUnionOf AS ?nodeType)
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
            # THING
            ?id a owl:Thing .
            FILTER(?id = owl:Thing)
            BIND(owl:Thing AS ?nodeType)
        }
        UNION
        {   
            # 3. Identify Unions
            ?id owl:unionOf ?list .
            BIND(owl:unionOf AS ?nodeType)
        }
        UNION
        {
            # CLASS
            ?id a rdfs:Class .
            FILTER(?id != owl:Class)
            BIND(owl:Class AS ?nodeType)
        }
        UNION
        {
            # LITERAL
            FILTER(isLiteral(?id))
            BIND(rdfs:Literal AS ?nodeType)
        }
        UNION
        {
            # RESOURCE
            ?id a rdfs:Resource .
            BIND(rdfs:Resource AS ?nodeType)
        }
        #########
        # EDGES #
        #########
        UNION
        {
            # 1. Identify RDF properties
            ?id a rdf:Property .
            OPTIONAL {?id rdfs:range ?target}
            BIND(rdf:Property AS ?nodeType)
        }
        UNION
        {
            # 3. Identify datatypes
            ?id a rdfs:Datatype .
            OPTIONAL {?id rdfs:range ?target}
            BIND(rdfs:Datatype AS ?nodeType)
        }
        UNION
        {
            # 2. Identify subclasses
            ?id rdfs:subClassOf ?target .
            BIND(rdfs:subClassOf AS ?nodeType)
        }
        UNION
        {
            # 4. Identify OWL datatype properties
            ?id owl:DatatypeProperty ?target .
            BIND(owl:DatatypeProperty AS ?nodeType)
        }
        UNION
        {
            # 5. Identify OWL disjoint with
            ?id owl:disjointWith ?target .
            BIND(owl:disjointWith AS ?nodeType)
        }
        UNION
        {
            # DEPRECATED PROPERTY
            ?id owl:DeprecatedProperty ?target .
            BIND(owl:DeprecatedProperty AS ?nodeType)
        }
        UNION
        {
            # INVERSE OF
            ?id owl:inverseOf ?target .
            BIND(owl:inverseOf AS ?nodeType)
        }
        UNION
        {
            # OBJECT PROPERTY
            ?id a owl:ObjectProperty .
            OPTIONAL {?id rdfs:range ?target}
            BIND(owl:ObjectProperty AS ?nodeType)
        }
        UNION
        {
            # VALUES FROM
            {
                ?id owl:someValuesFrom ?target .
            }
            UNION
            {
                ?id owl:allValuesFrom ?target .
            }
            BIND("ValuesFrom" AS ?nodeType)
        }
        ###########
        # General #
        ###########
        UNION
        {
            # EXTERNALS
            ?id xml:base ?base .
            BIND(xml:base AS ?nodeType)
        }
        UNION
        {
            # DEPRECATED
            ?id owl:deprecated ?target .
            BIND(owl:deprecated AS ?nodeType)
        }
    }

    ORDER BY ?nodeType
    "#;