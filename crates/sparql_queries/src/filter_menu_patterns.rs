use crate::{
    assembly::{DEFAULT_PREFIXES, QueryAssembler},
    element_type_injection::SparqlSnippet,
    prelude::GENERAL_SNIPPETS,
};
use grapher::prelude::{Characteristic, ElementType};
use std::collections::HashMap;

// TODO: Define actual patterns for characteristics.
// fn get_characteristic_pattern(characteristic: &Characteristic) -> Option<String> {
//     match characteristic {
//         Characteristic::Transitive => Some("{ ?p rdf:type owl:TransitiveProperty }".to_string()),
//         Characteristic::FunctionalProperty => {
//             Some("{ ?p rdf:type owl:FunctionalProperty }".to_string())
//         }
//         Characteristic::InverseFunctionalProperty => {
//             Some("{ ?p rdf:type owl:InverseFunctionalProperty }".to_string())
//         }
//         Characteristic::Symmetric => Some("{ ?p rdf:type owl:SymmetricProperty }".to_string()),
//         Characteristic::Asymmetric => Some("{ ?p rdf:type owl:AsymmetricProperty }".to_string()),
//         Characteristic::Reflexive => Some("{ ?p rdf:type owl:ReflexiveProperty }".to_string()),
//         Characteristic::Irreflexive => Some("{ ?p rdf:type owl:IrreflexiveProperty }".to_string()),
//         _ => None,
//     }
// }

/// Create a SPARQL query with elements included based on the truth value in the hashmaps.
pub fn generate_sparql_query(
    element_checks: HashMap<ElementType, bool>,
    // char_checks: HashMap<Characteristic, bool>,
) -> String {
    let mut snippets = element_checks
        .iter()
        .filter(|&(_, &checked)| checked)
        .map(|(elem, _)| elem.snippet())
        .collect::<Vec<&str>>();

    snippets.extend(GENERAL_SNIPPETS);

    // for (char, &checked) in char_checks.iter() {
    //     if checked {
    //         if let Some(pattern) = get_characteristic_pattern(char) {
    //             patterns.push(pattern);
    //         }
    //     }
    // }

    QueryAssembler::assemble_query(DEFAULT_PREFIXES.into(), snippets)
}
