use super::WorkbenchMenuItems;
use grapher::web::prelude::NodeType;
use leptos::prelude::*;
use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;

//Convert a NodeType string to a readable display name with spaces between capital letters
fn format_node_type_name(node_type: &NodeType) -> String {
    let debug_str = format!("{:?}", node_type);
    let mut result = String::new();

    for (i, ch) in debug_str.chars().enumerate() {
        if i > 0 && ch.is_uppercase() {
            result.push(' ');
        }
        result.push(ch);
    }

    result
}

fn get_node_type_pattern(node_type: &NodeType) -> Option<String> {
    match node_type {
        NodeType::Class => Some(
            "{ ?class rdf:type owl:Class . FILTER(!isBlank(?class)) . OPTIONAL { ?class rdfs:label ?label } }"
                .to_string(),
        ),
        NodeType::ExternalClass => Some(
            "{ ?externalClass rdf:type owl:Class . ?externalClass rdfs:isDefinedBy ?definedBy . OPTIONAL { ?externalClass rdfs:label ?label } }"
                .to_string(),
        ),
        NodeType::EquivalentClass => Some(
            "{ ?equivalentClass rdf:type owl:Class . ?equivalentClass owl:equivalentClass ?target . OPTIONAL { ?equivalentClass rdfs:label ?label } }"
                .to_string(),
        ),
        NodeType::DeprecatedClass => Some(
            "{ ?deprecatedClass rdf:type owl:Class . ?deprecatedClass owl:deprecated true . OPTIONAL { ?deprecatedClass rdfs:label ?label } }"
                .to_string(),
        ),
        NodeType::AnonymousClass => Some(
            "{ ?anonymousClass rdf:type owl:Class . FILTER(isBlank(?anonymousClass)) }"
                .to_string(),
        ),
        NodeType::Thing => Some("{ VALUES ?thing { <http://www.w3.org/2002/07/owl#Thing> } }".to_string()),
        NodeType::RdfsClass => Some(
            "{ ?rdfsClass rdf:type rdfs:Class . OPTIONAL { ?rdfsClass rdfs:label ?label } }"
                .to_string(),
        ),
        NodeType::RdfsResource => Some(
            "{ ?rdfsResource rdf:type rdfs:Resource . OPTIONAL { ?rdfsResource rdfs:label ?label } }"
                .to_string(),
        ),
        NodeType::Literal => Some(
            "{ ?literal rdf:type rdfs:Datatype . OPTIONAL { ?literal rdfs:label ?label } }"
                .to_string(),
        ),
        NodeType::Datatype => Some(
            "{ ?datatype rdf:type rdfs:Datatype . OPTIONAL { ?datatype rdfs:label ?label } }"
                .to_string(),
        ),
        NodeType::Union => Some(
            "{ ?unionOf rdf:type owl:Class . FILTER(EXISTS { ?unionOf owl:unionOf ?v }) . OPTIONAL { ?unionOf rdfs:label ?label } . OPTIONAL { ?owner owl:equivalentClass ?unionOf . ?owner rdfs:label ?ownerLabel } }"
                .to_string(),
        ),
        NodeType::Intersection => Some(
            "{ ?intersectionOf rdf:type owl:Class . FILTER(EXISTS { ?intersectionOf owl:intersectionOf ?v }) . OPTIONAL { ?intersectionOf rdfs:label ?label } . OPTIONAL { ?owner owl:equivalentClass ?intersectionOf . ?owner rdfs:label ?ownerLabel } }"
                .to_string(),
        ),
        NodeType::Complement => Some(
            "{ ?complementOf rdf:type owl:Class . FILTER(EXISTS { ?complementOf owl:complementOf ?v }) . OPTIONAL { ?complementOf rdfs:label ?label } . OPTIONAL { ?owner owl:equivalentClass ?complementOf . ?owner rdfs:label ?ownerLabel } }"
                .to_string(),
        ),
        NodeType::DisjointUnion => Some(
            "{ ?disjointUnionOf rdf:type owl:Class . FILTER(EXISTS { ?disjointUnionOf owl:disjointUnionOf ?v }) . OPTIONAL { ?disjointUnionOf rdfs:label ?label } . OPTIONAL { ?owner owl:equivalentClass ?disjointUnionOf . ?owner rdfs:label ?ownerLabel } }"
                .to_string(),
        ),
        NodeType::ObjectProperty => Some(
            "{ ?objectProperty rdf:type owl:ObjectProperty . OPTIONAL { ?objectProperty rdfs:label ?label } }"
                .to_string(),
        ),
        NodeType::DatatypeProperty => Some(
            "{ ?datatypeProperty rdf:type owl:DatatypeProperty . OPTIONAL { ?datatypeProperty rdfs:label ?label } }"
                .to_string(),
        ),
        NodeType::SubclassOf => Some(
            "{ ?subClassOf rdf:type owl:Class . FILTER(EXISTS { ?subClassOf rdfs:subClassOf ?v }) }"
                .to_string(),
        ),
        NodeType::InverseProperty => Some(
            "{ ?inverseOf rdf:type owl:ObjectProperty . FILTER(EXISTS { ?inverseOf owl:inverseOf ?v }) }"
                .to_string(),
        ),
        NodeType::DisjointWith => Some(
            "{ ?disjointWith rdf:type owl:Class . FILTER(EXISTS { ?disjointWith owl:disjointWith ?v }) }"
                .to_string(),
        ),
        NodeType::RdfProperty => Some(
            "{ ?rdfProperty rdf:type rdf:Property . OPTIONAL { ?rdfProperty rdfs:label ?label } }"
                .to_string(),
        ),
        NodeType::DeprecatedProperty => Some(
            "{ ?deprecatedProperty rdf:type owl:DeprecatedProperty . OPTIONAL { ?deprecatedProperty rdfs:comment ?comment } }"
                .to_string(),
        ),
        NodeType::ExternalProperty => Some(
            "{ ?externalProperty rdf:type owl:Property . ?externalProperty rdfs:isDefinedBy ?definedBy . OPTIONAL { ?externalProperty rdfs:label ?label } }"
                .to_string(),
        ),
        NodeType::ValuesFrom => Some(
            "{ ?valuesFrom rdf:type owl:Restriction . FILTER (EXISTS { ?valuesFrom owl:someValuesFrom ?v }) . ?valuesFrom owl:someValuesFrom ?someValuesFrom }"
                .to_string(),
        ),
        NodeType::NoDraw => None,
    }
}

fn generate_sparql_query(checks: &HashMap<NodeType, bool>) -> String {
    let mut patterns: Vec<String> = Vec::new();

    for (node_type, &checked) in checks.iter() {
        if checked {
            if let Some(pattern) = get_node_type_pattern(node_type) {
                patterns.push(pattern);
            }
        }
    }

    if patterns.is_empty() {
        return "SELECT DISTINCT * WHERE { ?s ?p ?o }".to_string();
    }

    let union_clause = patterns.join(" UNION ");
    format!("SELECT DISTINCT * WHERE {{ {} }}", union_clause)
}

#[derive(Clone)]
struct NodeChild {
    node_type: NodeType,
    display: &'static str,
}

#[derive(Clone)]
struct NodeGroup {
    name: &'static str,
    children: &'static [NodeChild],
}

#[component]
pub fn FilterMenu() -> impl IntoView {
    // Mocked counts for each node type
    let mut counts: HashMap<NodeType, usize> = HashMap::new();
    counts.insert(NodeType::Class, 15);
    counts.insert(NodeType::Thing, 1);
    counts.insert(NodeType::EquivalentClass, 10);
    counts.insert(NodeType::DeprecatedClass, 17);
    counts.insert(NodeType::AnonymousClass, 2);
    counts.insert(NodeType::ExternalClass, 5);
    counts.insert(NodeType::RdfsClass, 6);
    counts.insert(NodeType::RdfsResource, 4);
    counts.insert(NodeType::Literal, 8);
    counts.insert(NodeType::Union, 3);
    counts.insert(NodeType::Intersection, 2);
    counts.insert(NodeType::Complement, 5);
    counts.insert(NodeType::DisjointUnion, 0);
    counts.insert(NodeType::Datatype, 10);
    counts.insert(NodeType::ObjectProperty, 12);
    counts.insert(NodeType::DatatypeProperty, 9);
    counts.insert(NodeType::SubclassOf, 14);
    counts.insert(NodeType::InverseProperty, 7);
    counts.insert(NodeType::DisjointWith, 11);
    counts.insert(NodeType::RdfProperty, 13);
    counts.insert(NodeType::DeprecatedProperty, 16);
    counts.insert(NodeType::ExternalProperty, 18);
    counts.insert(NodeType::ValuesFrom, 4);
    counts.insert(NodeType::NoDraw, 0);

    let groups = vec![
        // Define groups of node types
        NodeGroup {
            name: "Classes",
            children: &[
                NodeChild {
                    node_type: NodeType::Class,
                    display: "Class",
                },
                NodeChild {
                    node_type: NodeType::ExternalClass,
                    display: "External Class",
                },
                NodeChild {
                    node_type: NodeType::EquivalentClass,
                    display: "Equivalent Class",
                },
                NodeChild {
                    node_type: NodeType::DeprecatedClass,
                    display: "Deprecated Class",
                },
                NodeChild {
                    node_type: NodeType::AnonymousClass,
                    display: "Anonymous Class",
                },
                NodeChild {
                    node_type: NodeType::Thing,
                    display: "Thing",
                },
            ],
        },
        NodeGroup {
            name: "RDF",
            children: &[
                NodeChild {
                    node_type: NodeType::RdfsClass,
                    display: "Rdfs Class",
                },
                NodeChild {
                    node_type: NodeType::RdfsResource,
                    display: "Rdfs Resource",
                },
                NodeChild {
                    node_type: NodeType::Literal,
                    display: "Literal",
                },
                NodeChild {
                    node_type: NodeType::Datatype,
                    display: "Datatype",
                },
            ],
        },
        NodeGroup {
            name: "Set Operators",
            children: &[
                NodeChild {
                    node_type: NodeType::Union,
                    display: "Union",
                },
                NodeChild {
                    node_type: NodeType::Intersection,
                    display: "Intersection",
                },
                NodeChild {
                    node_type: NodeType::Complement,
                    display: "Complement",
                },
                NodeChild {
                    node_type: NodeType::DisjointUnion,
                    display: "Disjoint Union",
                },
            ],
        },
        NodeGroup {
            name: "Properties",
            children: &[
                NodeChild {
                    node_type: NodeType::ObjectProperty,
                    display: "Object Property",
                },
                NodeChild {
                    node_type: NodeType::DatatypeProperty,
                    display: "Datatype Property",
                },
                NodeChild {
                    node_type: NodeType::SubclassOf,
                    display: "Subclass Of",
                },
                NodeChild {
                    node_type: NodeType::InverseProperty,
                    display: "Inverse Property",
                },
                NodeChild {
                    node_type: NodeType::DisjointWith,
                    display: "Disjoint With",
                },
                NodeChild {
                    node_type: NodeType::RdfProperty,
                    display: "Rdf Property",
                },
                NodeChild {
                    node_type: NodeType::DeprecatedProperty,
                    display: "Deprecated Property",
                },
                NodeChild {
                    node_type: NodeType::ExternalProperty,
                    display: "External Property",
                },
                NodeChild {
                    node_type: NodeType::ValuesFrom,
                    display: "Values From",
                },
                NodeChild {
                    node_type: NodeType::NoDraw,
                    display: "No Draw",
                },
            ],
        },
    ];

    //Collect all defined node types from groups
    let defined_types: HashSet<NodeType> = groups
        .iter()
        .flat_map(|g| g.children.iter().map(|c| c.node_type))
        .collect();

    //Get all possible node types from the enum
    let all_node_types: Vec<NodeType> = NodeType::iter().collect();

    //Find undefined types
    let undefined_types: Vec<NodeType> = all_node_types
        .into_iter()
        .filter(|t| !defined_types.contains(t))
        .collect();

    //Add undefined types to counts with static pseudo-random values.
    let predefined_values = vec![
        2, 4, 1, 20, 7, 3, 5, 11, 13, 17, 4, 2, 6, 8, 9, 10, 12, 14, 15, 16, 18, 19,
    ];
    for (i, node_type) in undefined_types.iter().enumerate() {
        counts.insert(*node_type, predefined_values[i]);
    }

    //Create Misc group if there are undefined types.
    let mut groups = groups;
    if !undefined_types.is_empty() {
        let misc_children: Vec<NodeChild> = undefined_types
            .iter()
            .map(|node_type| NodeChild {
                node_type: *node_type,
                display: Box::leak(format_node_type_name(node_type).into_boxed_str()),
            })
            .collect();

        groups.push(NodeGroup {
            name: "Misc",
            children: Box::leak(misc_children.into_boxed_slice()),
        });
    }

    let groups = groups;
    // Signals to track open/closed state of groups and checked state of node types
    let (opens, set_opens) = signal(vec![false; groups.len()]);
    let mut initial_checks: HashMap<NodeType, bool> = HashMap::new();
    // sets all node types to checked by default
    for g in groups.iter() {
        for child in g.children.iter() {
            initial_checks.insert(child.node_type, true);
        }
    }
    let (checks, set_checks) = signal(initial_checks);

    let groups_len = groups.len();

    view! {
        <WorkbenchMenuItems title="Filter by node type">
            <div class="flex gap-2 items-center pb-3 mb-3 border-b">
                <label class="flex gap-2 items-center cursor-pointer">
                    // checkbox to enable/disable all filters
                    <input
                        type="checkbox"
                        prop:checked=move || {
                            let checksMap = checks.get();
                            let nodeTypeKeys: Vec<_> = checksMap
                                .keys()
                                .copied()
                                .collect();
                            nodeTypeKeys
                                .iter()
                                .all(|nodeType| *checksMap.get(nodeType).unwrap_or(&true))
                        }
                        on:change=move |_| {
                            let checksMap = checks.get();
                            let nodeTypeKeys: Vec<_> = checksMap
                                .keys()
                                .copied()
                                .collect();
                            let allEnabled = nodeTypeKeys
                                .iter()
                                .all(|nodeType| *checksMap.get(nodeType).unwrap_or(&true));
                            let mut updatedChecks = checksMap;
                            for nodeType in nodeTypeKeys {
                                updatedChecks.insert(nodeType, !allEnabled);
                            }
                            let query = generate_sparql_query(&updatedChecks);
                            leptos::logging::log!("{}", query);
                            set_checks.set(updatedChecks);
                        }
                    />
                    <span class="text-sm">
                        {move || {
                            let checksMap = checks.get();
                            let nodeTypeKeys: Vec<_> = checksMap
                                .keys()
                                .copied()
                                .collect();
                            let allEnabled = nodeTypeKeys
                                .iter()
                                .all(|nodeType| *checksMap.get(nodeType).unwrap_or(&true));
                            if allEnabled {
                                "Disable all filters"
                            } else {
                                "Enable all filters"
                            }
                        }}
                    </span>
                </label>
            </div>
            // Map each node group and node type to a checkbox with count
            {(0..groups_len)
                .map(|groupIndex| {
                    let group = groups[groupIndex].clone();
                    let opens = opens.clone();
                    let set_opens = set_opens.clone();
                    let checks = checks.clone();
                    let set_checks = set_checks.clone();
                    let total: usize = group
                        .children
                        .iter()
                        .map(|node| *counts.get(&node.node_type).unwrap_or(&0))
                        .sum();
                    view! {
                        // Group header with toggle and select/deselect all in group
                        <div class="pb-2 mb-2 border-b">
                            <div class="flex gap-2 justify-between items-center">
                                <button
                                    class="flex-1 py-2 text-left hover:bg-gray-100"
                                    on:click=move |_| {
                                        let mut openStates = opens.get();
                                        openStates[groupIndex] = !openStates[groupIndex];
                                        set_opens.set(openStates);
                                    }
                                >
                                    <div class="flex justify-between items-center">
                                        <div class="font-medium">
                                            // Display group name with counts
                                            {
                                                let counts = counts.clone();
                                                move || {
                                                    let rendered: usize = group
                                                        .children
                                                        .iter()
                                                        .map(|node| {
                                                            if *checks.get().get(&node.node_type).unwrap_or(&true) {
                                                                *counts.get(&node.node_type).unwrap_or(&0)
                                                            } else {
                                                                0
                                                            }
                                                        })
                                                        .sum();
                                                    if rendered == total {
                                                        format!("{}: {}", group.name, total)
                                                    } else {
                                                        format!("{}: ({}/{})", group.name, rendered, total)
                                                    }
                                                }
                                            }
                                        </div>
                                        // Toggle arrow
                                        <div class="text-sm text-gray-500">
                                            {move || {
                                                if opens.get()[groupIndex] { "▾" } else { "▸" }
                                            }}
                                        </div>
                                    </div>
                                </button>
                                <label class="flex gap-1 items-center">
                                    // Checkbox to select/deselect all in group
                                    <input
                                        type="checkbox"
                                        class="w-4 h-4 cursor-pointer"
                                        prop:checked=move || {
                                            let checksMap = checks.get();
                                            group
                                                .children
                                                .iter()
                                                .all(|node| {
                                                    *checksMap.get(&node.node_type).unwrap_or(&true)
                                                })
                                        }
                                        on:change=move |_| {
                                            let checksMap = checks.get();
                                            let allEnabled = group
                                                .children
                                                .iter()
                                                .all(|node| {
                                                    *checksMap.get(&node.node_type).unwrap_or(&true)
                                                });
                                            let mut updatedChecks = checksMap;
                                            for child in group.children.iter() {
                                                updatedChecks.insert(child.node_type, !allEnabled);
                                            }
                                            let query = generate_sparql_query(&updatedChecks);
                                            leptos::logging::log!("{}", query);
                                            set_checks.set(updatedChecks);
                                        }
                                    />
                                </label>
                            </div>
                            // Expandable area for node type checkboxes with animation
                            <div style=move || {
                                if opens.get()[groupIndex] {
                                    "max-height: 1000px; opacity: 1; overflow: hidden; transition: max-height 0.5s ease, opacity 0.35s ease; margin-top: 0.5rem; padding-left: 1rem;"
                                } else {
                                    "max-height: 0px; opacity: 0; overflow: hidden; transition: max-height 0.5s ease, opacity 0.35s ease; margin-top: 0; padding-left: 1rem;"
                                }
                            }>
                                // Node type checkboxes with counts inside the group accordion
                                {group
                                    .children
                                    .iter()
                                    .map(|child| {
                                        let child_type = child.node_type;
                                        let child_display = child.display;
                                        let cnt = *counts.get(&child_type).unwrap_or(&0);
                                        let checks = checks.clone();
                                        let set_checks = set_checks.clone();
                                        view! {
                                            <div class="flex justify-between items-center py-1 text-sm text-gray-700">
                                                <label class="flex gap-3 items-center cursor-pointer">
                                                    <img
                                                        src=format!("/node_legends/{:?}.png", child_type)
                                                        alt={format!("{} icon", child_display)}
                                                        class="w-8 h-8 object-contain"
                                                    />
                                                    // Checkbox for individual node type
                                                    <input
                                                        type="checkbox"
                                                        prop:checked=move || {
                                                            *checks.get().get(&child_type).unwrap_or(&true)
                                                        }
                                                        on:change=move |_| {
                                                            let mut m = checks.get();
                                                            let entry = m.entry(child_type).or_insert(true);
                                                            *entry = !*entry;
                                                            let query = generate_sparql_query(&m);
                                                            leptos::logging::log!("{}", query);
                                                            set_checks.set(m);
                                                        }
                                                    />
                                                    <span>{child_display}": "</span>
                                                </label>
                                                <div class="text-sm text-gray-600">
                                                    // Display count based on checkbox state
                                                    {move || {
                                                        if *checks.get().get(&child_type).unwrap_or(&true) {
                                                            format!("{}", cnt)
                                                        } else {
                                                            format!("(0/{})", cnt)
                                                        }
                                                    }}
                                                </div>
                                            </div>
                                        }
                                    })
                                    .collect::<Vec<_>>()}
                            </div>
                        </div>
                    }
                })
            .collect::<Vec<_>>()}
        </WorkbenchMenuItems>
    }
}
