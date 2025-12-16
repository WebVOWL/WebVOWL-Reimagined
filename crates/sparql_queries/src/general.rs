pub const EXTERNALS : &str = 
            r#"{
                ?id xml:base ?base .
                BIND(xml:base AS ?nodeType)         
            }"#;

pub const DEPRECATED : &str = 
            r#"{
                ?id owl:deprecated ?target .
                BIND(owl:deprecated AS ?nodeType)
            }"#;
pub const LABEL : &str = 
            r#"{
                OPTIONAL { ?id rdfs:label ?theLabel }
                OPTIONAL { ?id rdf:resource ?resLabel }
                OPTIONAL { ?id rdf:ID ?idLabel }
                BIND (
                    COALESCE(
                        IF( BOUND(?theLabel), ?theLabel, 1/0 ),
                        IF( BOUND(?resLabel), ?resLabel, 1/0 ),
                        IF( BOUND(?idLabel), ?idLabel, 1/0 ),
                        ""
                    ) AS ?label
                )
            }"#;