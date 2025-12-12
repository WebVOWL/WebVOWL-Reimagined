use super::WorkbenchMenuItems;
use crate::components::user_input::file_upload::handle_internal_sparql;
use grapher::prelude::{
    Characteristic, ElementType, GenericEdge, GenericNode, GenericType, GraphDisplayData, OwlEdge,
    OwlNode, OwlType, RdfEdge, RdfType, RdfsEdge, RdfsNode, RdfsType,
};
use grapher::prelude::{EVENT_DISPATCHER, RenderEvent};
use leptos::prelude::*;
use leptos::task::spawn_local;
use log::error;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FilterNode {
    Owl(OwlNode),
    Rdfs(RdfsNode),
    Generic(GenericNode),
}

impl std::fmt::Display for FilterNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterNode::Owl(n) => n.fmt(f),
            FilterNode::Rdfs(n) => n.fmt(f),
            FilterNode::Generic(n) => n.fmt(f),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FilterEdge {
    Owl(OwlEdge),
    Rdf(RdfEdge),
    Rdfs(RdfsEdge),
    Generic(GenericEdge),
}

impl std::fmt::Display for FilterEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilterEdge::Owl(e) => e.fmt(f),
            FilterEdge::Rdf(e) => e.fmt(f),
            FilterEdge::Rdfs(e) => e.fmt(f),
            FilterEdge::Generic(e) => e.fmt(f),
        }
    }
}

fn get_node_pattern(node: &FilterNode) -> Option<String> {
    match node {
        FilterNode::Owl(OwlNode::Class) => Some(
            r#"{
            ?id a owl:Class .
            FILTER(isIRI(?id))
            BIND(owl:Class AS ?nodeType)
            OPTIONAL { ?id rdfs:label ?label }
            }"#
            .to_string(),
        ),
        // FilterNode::Owl(OwlNode::ExternalClass) => Some(
        //     "{ ?externalClass rdf:type owl:Class . ?externalClass rdfs:isDefinedBy ?definedBy . OPTIONAL { ?externalClass rdfs:label ?label } }"
        //         .to_string(),
        // ),
        FilterNode::Owl(OwlNode::EquivalentClass) => Some(
            r#"{
            ?id owl:equivalentClass ?label
            BIND("EquivalentClass" AS ?nodeType)
            }"#
            .to_string(),
        ),
        // FilterNode::Owl(OwlNode::DeprecatedClass) => Some(
        //     "{ ?deprecatedClass rdf:type owl:Class . ?deprecatedClass owl:deprecated true . OPTIONAL { ?deprecatedClass rdfs:label ?label } }"
        //         .to_string(),
        // ),
        FilterNode::Owl(OwlNode::AnonymousClass) => Some(
            r#"{
            ?id a owl:Class
            FILTER(!isIRI(?id))
            BIND("AnonymousClass" AS ?nodeType)
            OPTIONAL { ?id rdfs:label ?label }
            }"#
            .to_string(),
        ),
        // FilterNode::Owl(OwlNode::Thing) => Some("{ VALUES ?thing { <http://www.w3.org/2002/07/owl#Thing> } }".to_string()),
        // FilterNode::Rdfs(RdfsNode::Class) => Some(
        //     "{ ?rdfsClass rdf:type rdfs:Class . OPTIONAL { ?rdfsClass rdfs:label ?label } }"
        //         .to_string(),
        // ),
        // FilterNode::Rdfs(RdfsNode::Resource) => Some(
        //     "{ ?rdfsResource rdf:type rdfs:Resource . OPTIONAL { ?rdfsResource rdfs:label ?label } }"
        //         .to_string(),
        // ),
        // FilterNode::Rdfs(RdfsNode::Literal) => Some(
        //     "{ ?literal rdf:type rdfs:Datatype . OPTIONAL { ?literal rdfs:label ?label } }"
        //         .to_string(),
        // ),
        FilterNode::Owl(OwlNode::UnionOf) => Some(
            r#"{
            ?id owl:unionOf ?label .
            BIND("UnionOf" AS ?nodeType)
            }"#
            .to_string(),
        ),
        FilterNode::Owl(OwlNode::IntersectionOf) => Some(
            r#"{
            ?id owl:intersectionOf ?label .
            BIND("IntersectionOf" AS ?nodeType)
            }"#
            .to_string(),
        ),
        // FilterNode::Owl(OwlNode::Complement) => Some(
        //     "{ ?complementOf rdf:type owl:Class . FILTER(EXISTS { ?complementOf owl:complementOf ?v }) . OPTIONAL { ?complementOf rdfs:label ?label } . OPTIONAL { ?owner owl:equivalentClass ?complementOf . ?owner rdfs:label ?ownerLabel } }"
        //         .to_string(),
        // ),
        // FilterNode::Owl(OwlNode::DisjointUnion) => Some(
        //     "{ ?disjointUnionOf rdf:type owl:Class . FILTER(EXISTS { ?disjointUnionOf owl:disjointUnionOf ?v }) . OPTIONAL { ?disjointUnionOf rdfs:label ?label } . OPTIONAL { ?owner owl:equivalentClass ?disjointUnionOf . ?owner rdfs:label ?ownerLabel } }"
        //         .to_string(),
        // ),
        _ => None,
    }
}

fn get_edge_pattern(edge: &FilterEdge) -> Option<String> {
    match edge {
        //  FilterEdge::Owl(OwlEdge::ObjectProperty) => Some(
        //     "{ ?objectProperty rdf:type owl:ObjectProperty . OPTIONAL { ?objectProperty rdfs:label ?label } }"
        //         .to_string(),
        // ),
        // FilterEdge::Owl(OwlEdge::DatatypeProperty) => Some(
        //     "{ ?datatypeProperty rdf:type owl:DatatypeProperty . OPTIONAL { ?datatypeProperty rdfs:label ?label } }"
        //         .to_string(),
        // ),
        // FilterEdge::Rdfs(RdfsEdge::SubclassOf) => Some(
        //     "{ ?subClassOf rdf:type owl:Class . FILTER(EXISTS { ?subClassOf rdfs:subClassOf ?v }) }"
        //         .to_string(),
        // ),
        // FilterEdge::Owl(OwlEdge::InverseOf) => Some(
        //     "{ ?inverseOf rdf:type owl:ObjectProperty . FILTER(EXISTS { ?inverseOf owl:inverseOf ?v }) }"
        //         .to_string(),
        // ),
        // FilterEdge::Owl(OwlEdge::DisjointWith) => Some(
        //     "{ ?disjointWith rdf:type owl:Class . FILTER(EXISTS { ?disjointWith owl:disjointWith ?v }) }"
        //         .to_string(),
        // ),
        // FilterEdge::Rdf(RdfEdge::RdfProperty) => Some(
        //     "{ ?rdfProperty rdf:type rdf:Property . OPTIONAL { ?rdfProperty rdfs:label ?label } }"
        //         .to_string(),
        // ),
        // FilterEdge::Owl(OwlEdge::DeprecatedProperty) => Some(
        //     "{ ?deprecatedProperty rdf:type owl:DeprecatedProperty . OPTIONAL { ?deprecatedProperty rdfs:comment ?comment } }"
        //         .to_string(),
        // ),
        // FilterEdge::Owl(OwlEdge::ExternalProperty) => Some(
        //     "{ ?externalProperty rdf:type owl:Property . ?externalProperty rdfs:isDefinedBy ?definedBy . OPTIONAL { ?externalProperty rdfs:label ?label } }"
        //         .to_string(),
        // ),
        // FilterEdge::Owl(OwlEdge::ValuesFrom) => Some(
        //     "{ ?valuesFrom rdf:type owl:Restriction . FILTER (EXISTS { ?valuesFrom owl:someValuesFrom ?v }) . ?valuesFrom owl:someValuesFrom ?someValuesFrom }"
        //         .to_string(),
        // ),
        _ => None,
    }
}

// TODO: Define actual patterns for characteristics.
fn get_characteristic_pattern(characteristic: &Characteristic) -> Option<String> {
    match characteristic {
        // Characteristic::Transitive => Some("{ ?p rdf:type owl:TransitiveProperty }".to_string()),
        // Characteristic::FunctionalProperty => {
        //     Some("{ ?p rdf:type owl:FunctionalProperty }".to_string())
        // }
        // Characteristic::InverseFunctionalProperty => {
        //     Some("{ ?p rdf:type owl:InverseFunctionalProperty }".to_string())
        // }
        // Characteristic::Symmetric => Some("{ ?p rdf:type owl:SymmetricProperty }".to_string()),
        // Characteristic::Asymmetric => Some("{ ?p rdf:type owl:AsymmetricProperty }".to_string()),
        // Characteristic::Reflexive => Some("{ ?p rdf:type owl:ReflexiveProperty }".to_string()),
        // Characteristic::Irreflexive => Some("{ ?p rdf:type owl:IrreflexiveProperty }".to_string()),
        _ => None,
    }
}

fn generate_sparql_query(
    node_checks: &HashMap<FilterNode, bool>,
    edge_checks: &HashMap<FilterEdge, bool>,
    char_checks: &HashMap<Characteristic, bool>,
) -> String {
    let mut patterns: Vec<String> = Vec::new();

    for (node, &checked) in node_checks.iter() {
        if checked {
            if let Some(pattern) = get_node_pattern(node) {
                patterns.push(pattern);
            }
        }
    }

    for (edge, &checked) in edge_checks.iter() {
        if checked {
            if let Some(pattern) = get_edge_pattern(edge) {
                patterns.push(pattern);
            }
        }
    }

    for (char, &checked) in char_checks.iter() {
        if checked {
            if let Some(pattern) = get_characteristic_pattern(char) {
                patterns.push(pattern);
            }
        }
    }

    let union_clause = if patterns.is_empty() {
        r#"
            BIND(<http://example.org/nothing> AS ?id)
            BIND(<http://example.org/nothing> AS ?nodeType)
            BIND("" AS ?label)
            FILTER(false)
        "#
        .to_string()
    } else {
        patterns.join(" UNION ")
    };

    format!(
        r#"
        PREFIX owl: <http://www.w3.org/2002/07/owl#>
        PREFIX rdfs: <http://www.w3.org/2000/01/rdf-schema#>
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>

        SELECT ?id ?nodeType ?label
        {{
            {}
        }}
        ORDER BY ?nodeType
        "#,
        union_clause
    )
    .to_string()
}

pub fn update_graph(query: String, graph_data: RwSignal<GraphDisplayData>) {
    spawn_local(async move {
        let output_result = handle_internal_sparql(query).await;
        match output_result {
            Ok(new_graph_data) => {
                graph_data.set(new_graph_data.clone());
                EVENT_DISPATCHER
                    .rend_write_chan
                    .send(RenderEvent::LoadGraph(new_graph_data));
            }
            Err(e) => error!("{}", e),
        }
    });
}

#[component]
pub fn FilterMenu(
    graph_data: RwSignal<GraphDisplayData>,
    total_graph_data: RwSignal<GraphDisplayData>,
) -> impl IntoView {
    let node_counts = Memo::new(move |_| {
        let mut counts: HashMap<FilterNode, usize> = HashMap::new();
        total_graph_data.with(|data| {
            for element in &data.elements {
                match element {
                    ElementType::Owl(OwlType::Node(n)) => {
                        *counts.entry(FilterNode::Owl(*n)).or_insert(0) += 1;
                    }
                    ElementType::Rdfs(RdfsType::Node(n)) => {
                        *counts.entry(FilterNode::Rdfs(*n)).or_insert(0) += 1;
                    }
                    ElementType::Generic(GenericType::Node(n)) => {
                        *counts.entry(FilterNode::Generic(*n)).or_insert(0) += 1;
                    }
                    _ => {}
                }
            }
        });
        counts
    });

    let edge_counts = Memo::new(move |_| {
        let mut counts: HashMap<FilterEdge, usize> = HashMap::new();
        total_graph_data.with(|data| {
            for element in &data.elements {
                match element {
                    ElementType::Owl(OwlType::Edge(e)) => {
                        *counts.entry(FilterEdge::Owl(*e)).or_insert(0) += 1;
                    }
                    ElementType::Rdf(RdfType::Edge(e)) => {
                        *counts.entry(FilterEdge::Rdf(*e)).or_insert(0) += 1;
                    }
                    ElementType::Rdfs(RdfsType::Edge(e)) => {
                        *counts.entry(FilterEdge::Rdfs(*e)).or_insert(0) += 1;
                    }
                    ElementType::Generic(GenericType::Edge(e)) => {
                        *counts.entry(FilterEdge::Generic(*e)).or_insert(0) += 1;
                    }
                    _ => {}
                }
            }
        });
        counts
    });

    let char_counts = Memo::new(move |_| {
        let mut counts: HashMap<Characteristic, usize> = HashMap::new();
        total_graph_data.with(|data| {
            for char_str in data.characteristics.values() {
                for part in char_str.split('\n') {
                    let c = match part.trim() {
                        "transitive" => Some(Characteristic::Transitive),
                        "functional" => Some(Characteristic::FunctionalProperty),
                        "inverse functional" => Some(Characteristic::InverseFunctionalProperty),
                        "symmetric" => Some(Characteristic::Symmetric),
                        "asymmetric" => Some(Characteristic::Asymmetric),
                        "reflexive" => Some(Characteristic::Reflexive),
                        "irreflexive" => Some(Characteristic::Irreflexive),
                        _ => None,
                    };
                    if let Some(characteristic) = c {
                        *counts.entry(characteristic).or_insert(0) += 1;
                    }
                }
            }
        });
        counts
    });

    let mut initial_node_checks = HashMap::new();
    let all_nodes = vec![
        FilterNode::Owl(OwlNode::Class),
        FilterNode::Owl(OwlNode::ExternalClass),
        FilterNode::Owl(OwlNode::Thing),
        FilterNode::Owl(OwlNode::DeprecatedClass),
        FilterNode::Owl(OwlNode::AnonymousClass),
        FilterNode::Owl(OwlNode::EquivalentClass),
        FilterNode::Owl(OwlNode::DisjointUnion),
        FilterNode::Owl(OwlNode::IntersectionOf),
        FilterNode::Owl(OwlNode::UnionOf),
        FilterNode::Owl(OwlNode::Complement),
        FilterNode::Rdfs(RdfsNode::Class),
        FilterNode::Rdfs(RdfsNode::Resource),
        FilterNode::Rdfs(RdfsNode::Literal),
    ];
    for n in &all_nodes {
        initial_node_checks.insert(n.clone(), true);
    }
    let (node_checks, set_node_checks) = signal(initial_node_checks);

    let mut initial_edge_checks = HashMap::new();
    let all_edges = vec![
        FilterEdge::Owl(OwlEdge::ObjectProperty),
        FilterEdge::Owl(OwlEdge::DatatypeProperty),
        FilterEdge::Rdfs(RdfsEdge::SubclassOf),
        FilterEdge::Owl(OwlEdge::InverseOf),
        FilterEdge::Owl(OwlEdge::DisjointWith),
        FilterEdge::Rdf(RdfEdge::RdfProperty),
        FilterEdge::Owl(OwlEdge::DeprecatedProperty),
        FilterEdge::Owl(OwlEdge::ExternalProperty),
        FilterEdge::Owl(OwlEdge::ValuesFrom),
    ];
    for edge in &all_edges {
        initial_edge_checks.insert(edge.clone(), true);
    }
    let (edge_checks, set_edge_checks) = signal(initial_edge_checks);

    let mut initial_char_checks = HashMap::new();
    let all_chars = vec![
        Characteristic::Transitive,
        Characteristic::FunctionalProperty,
        Characteristic::InverseFunctionalProperty,
        Characteristic::Reflexive,
        Characteristic::Irreflexive,
        Characteristic::Symmetric,
        Characteristic::Asymmetric,
    ];
    for characteristic in &all_chars {
        initial_char_checks.insert(characteristic.clone(), true);
    }
    let (char_checks, set_char_checks) = signal(initial_char_checks);

    let update_query = move || {
        let query =
            generate_sparql_query(&node_checks.get(), &edge_checks.get(), &char_checks.get());
        leptos::logging::log!("{}", query);
        update_graph(query, graph_data);
    };

    // 1. Classes
    let class_nodes = vec![
        FilterNode::Owl(OwlNode::Class),
        FilterNode::Owl(OwlNode::ExternalClass),
        FilterNode::Owl(OwlNode::EquivalentClass),
        FilterNode::Owl(OwlNode::DeprecatedClass),
        FilterNode::Owl(OwlNode::AnonymousClass),
        FilterNode::Owl(OwlNode::Thing),
    ];

    // 2. RDF
    let rdf_nodes = vec![
        FilterNode::Rdfs(RdfsNode::Class),
        FilterNode::Rdfs(RdfsNode::Resource),
        FilterNode::Rdfs(RdfsNode::Literal),
    ];

    // 3. Set Operators
    let set_nodes = vec![
        FilterNode::Owl(OwlNode::UnionOf),
        FilterNode::Owl(OwlNode::IntersectionOf),
        FilterNode::Owl(OwlNode::Complement),
        FilterNode::Owl(OwlNode::DisjointUnion),
    ];

    // 4. Properties (Edges)
    let properties = vec![
        FilterEdge::Owl(OwlEdge::ObjectProperty),
        FilterEdge::Owl(OwlEdge::DatatypeProperty),
        FilterEdge::Rdfs(RdfsEdge::SubclassOf),
        FilterEdge::Owl(OwlEdge::InverseOf),
        FilterEdge::Owl(OwlEdge::DisjointWith),
        FilterEdge::Rdf(RdfEdge::RdfProperty),
        FilterEdge::Owl(OwlEdge::DeprecatedProperty),
        FilterEdge::Owl(OwlEdge::ExternalProperty),
        FilterEdge::Owl(OwlEdge::ValuesFrom),
    ];

    // 5. Characteristics
    let characteristics = all_chars.clone();

    // Accordion State
    let (open_classes, set_open_classes) = signal(false);
    let (open_rdf, set_open_rdf) = signal(false);
    let (open_set, set_open_set) = signal(false);
    let (open_props, set_open_props) = signal(false);
    let (open_chars, set_open_chars) = signal(false);

    view! {
        <WorkbenchMenuItems title="Filter by Type">
             <div class="flex gap-2 items-center pb-3 mb-3 border-b">
                <button
                    class="text-sm text-blue-600 hover:text-blue-800"
                    on:click=move |_| {
                        let all_n = node_checks.get().values().all(|&v| v);
                        let all_e = edge_checks.get().values().all(|&v| v);
                        let all_c = char_checks.get().values().all(|&v| v);
                        let target = !(all_n && all_e && all_c);

                        let mut n = node_checks.get();
                        for v in n.values_mut() { *v = target; }
                        set_node_checks.set(n);

                        let mut e = edge_checks.get();
                        for v in e.values_mut() { *v = target; }
                        set_edge_checks.set(e);

                        let mut c = char_checks.get();
                        for v in c.values_mut() { *v = target; }
                        set_char_checks.set(c);

                        update_query();
                    }
                >
                    {move || {
                        let all_n = node_checks.get().values().all(|&v| v);
                        let all_e = edge_checks.get().values().all(|&v| v);
                        let all_c = char_checks.get().values().all(|&v| v);
                        if all_n && all_e && all_c { "Disable All" } else { "Enable All" }
                    }}
                </button>
             </div>

            <FilterGroup
                name="Classes"
                is_open=open_classes
                set_open=set_open_classes
                items=class_nodes
                checks=node_checks.into()
                set_checks=set_node_checks
                counts=node_counts.into()
                on_change=update_query
            />

            <FilterGroup
                name="RDF"
                is_open=open_rdf
                set_open=set_open_rdf
                items=rdf_nodes
                checks=node_checks.into()
                set_checks=set_node_checks
                counts=node_counts.into()
                on_change=update_query
            />

            <FilterGroup
                name="Set Operators"
                is_open=open_set
                set_open=set_open_set
                items=set_nodes
                checks=node_checks.into()
                set_checks=set_node_checks
                counts=node_counts.into()
                on_change=update_query
            />

            <FilterGroup
                name="Properties"
                is_open=open_props
                set_open=set_open_props
                items=properties
                checks=edge_checks.into()
                set_checks=set_edge_checks
                counts=edge_counts.into()
                on_change=update_query
            />

             <FilterGroup
                name="Characteristics"
                is_open=open_chars
                set_open=set_open_chars
                items=characteristics
                checks=char_checks.into()
                set_checks=set_char_checks
                counts=char_counts.into()
                on_change=update_query
            />

        </WorkbenchMenuItems>
    }
}

#[component]
fn FilterGroup<T>(
    name: &'static str,
    is_open: ReadSignal<bool>,
    set_open: WriteSignal<bool>,
    items: Vec<T>,
    checks: Signal<HashMap<T, bool>>,
    set_checks: WriteSignal<HashMap<T, bool>>,
    counts: Signal<HashMap<T, usize>>,
    on_change: impl Fn() + 'static + Clone,
) -> impl IntoView
where
    T: Clone + Eq + std::hash::Hash + std::fmt::Display + 'static + std::fmt::Debug + Send + Sync,
{
    let items_total = items.clone();
    let counts_total = counts;

    let items_count = items.clone();

    let items_all_check = items.clone();

    let items_all_change = items.clone();
    let on_change_all = on_change.clone();

    view! {
        <div class="pb-2 mb-2 border-b">
             <div class="flex gap-2 justify-between items-center">
                 <button
                    class="flex-1 py-2 text-left hover:bg-gray-100"
                    on:click=move |_| set_open.set(!is_open.get())
                 >
                    <div class="flex justify-between items-center">
                         <div class="font-medium">
                            {move || {
                                let total_count: usize = items_total
                                    .iter()
                                    .map(|item| *counts_total.get().get(item).unwrap_or(&0))
                                    .sum();

                                let rendered: usize = items_count.iter()
                                    .map(|item| if *checks.get().get(item).unwrap_or(&true) { *counts_total.get().get(item).unwrap_or(&0) } else { 0 })
                                    .sum();
                                format!("{}: ({}/{})", name, rendered, total_count)
                            }}
                         </div>
                         <div class="text-sm text-gray-500">
                             {move || if is_open.get() { "▾" } else { "▸" }}
                         </div>
                    </div>
                </button>
                <label class="flex gap-1 items-center">
                     <input
                        type="checkbox"
                        class="w-4 h-4 cursor-pointer"
                        prop:checked=move || {
                             items_all_check.iter().all(|item| *checks.get().get(item).unwrap_or(&true))
                        }
                        on:change=move |_| {
                            let current_checks = checks.get();
                            let all_enabled = items_all_change.iter().all(|item| *current_checks.get(item).unwrap_or(&true));
                            let mut new_checks = current_checks.clone();
                            for item in &items_all_change {
                                new_checks.insert(item.clone(), !all_enabled);
                            }
                            set_checks.set(new_checks);
                            on_change_all();
                        }
                    />
                </label>
             </div>

             <div style=move || {
                if is_open.get() {
                    "max-height: 1000px; opacity: 1; overflow: hidden; transition: max-height 0.5s ease, opacity 0.35s ease; margin-top: 0.5rem; padding-left: 1rem;"
                } else {
                    "max-height: 0px; opacity: 0; overflow: hidden; transition: max-height 0.5s ease, opacity 0.35s ease; margin-top: 0; padding-left: 1rem;"
                }
             }>
                {
                    items.into_iter().map(|item| {
                        let item_key = item.clone();
                        let item_key_check = item_key.clone();
                        let display = item.to_string();
                        let on_change_clone = on_change.clone();

                        view! {
                            <div class="flex justify-between items-center py-1 text-sm text-gray-700">
                                <label class="flex gap-3 items-center cursor-pointer">
                                    <input
                                        type="checkbox"
                                        prop:checked=move || *checks.get().get(&item_key_check).unwrap_or(&true)
                                        on:change=move |_| {
                                            let mut m = checks.get();
                                            let val = m.entry(item_key.clone()).or_insert(true);
                                            *val = !*val;
                                            set_checks.set(m);
                                            on_change_clone();
                                        }
                                    />
                                    <span>{display}</span>
                                </label>
                                <div class="text-sm text-gray-600">
                                     {move || if *checks.get().get(&item).unwrap_or(&true) {
                                         format!("{}", *counts.get().get(&item).unwrap_or(&0))
                                     } else {
                                         format!("(0/{})", *counts.get().get(&item).unwrap_or(&0))
                                     }}
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }
             </div>
        </div>
    }
}
