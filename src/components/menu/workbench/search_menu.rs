use leptos::prelude::*;
use grapher::web::prelude::NodeType as NodeType;

#[component]
pub fn SearchMenu() -> impl IntoView {
    let search_query = RwSignal::new(String::new());
    let expanded_category = RwSignal::new(Option::<String>::None);

    let categories = vec!["Classes", "RDF", "Properties", "Misc"];

    fn get_source_instances(category: &str) -> Vec<&str> {
        match category {
            "Classes" => vec![
                "Person", "Agent", "Organization", "Group", "Project", "Document", "Image",
                "Turtle", "Bob", "Alice", "User", "Admin", "Guest", "System", "Manager",
                "Developer", "Designer", "Tester", "Architect", "Engineer", "Consultant",
                "Director", "Executive", "Officer", "Employee", "Intern", "Volunteer",
                "Customer", "Client", "Partner", "Vendor", "Supplier", "Distributor",
                "Reseller", "Affiliate", "Associate", "Member", "Subscriber", "Follower",
                "Fan", "Supporter", "Donor", "Sponsor", "Patron", "Benefactor", "Contributor",
                "Participant", "Attendee", "Visitor", "Guest", "Tourist", "Traveler",
                "Explorer", "Adventurer", "Pioneer", "Settler", "Colonist", "Native",
                "Resident", "Citizen", "Subject", "Voter", "Taxpayer", "Ratepayer",
                "Homeowner", "Landlord", "Tenant", "Occupant", "Inhabitant", "Dweller",
                "LivingThing", "Animal", "Plant", "Fungus", "Bacterium", "Virus",
                "Mammal", "Bird", "Reptile", "Amphibian", "Fish", "Insect", "Arachnid",
                "Crustacean", "Mollusc", "Worm", "Sponge", "Jellyfish", "Coral",
                "Anemone", "Starfish", "Urchin", "Cucumber", "Lily", "Fern", "Moss",
                "Algae", "Lichen", "Mushroom", "Yeast", "Mold", "Mildew", "Rust", "Smut",
            ],
            "RDF" => vec![
                "Property", "Resource", "Literal", "Statement", "Subject", "Predicate", "Object",
                "Bag", "Seq", "Alt", "List", "nil", "first", "rest", "value", "type",
                "Turtle", "Node", "Graph", "Triple", "Quad", "Dataset", "NamedGraph",
                "BlankNode", "IRI", "URI", "URL", "URN", "Datatype", "LanguageTag",
                "PlainLiteral", "TypedLiteral", "XMLLiteral", "HTML", "JSON", "CSV",
                "TSV", "N-Triples", "N-Quads", "TriG", "RDF/XML", "RDFa", "JSON-LD",
                "HDT", "Thrift", "BinaryRDF", "Microdata", "Microformats", "Schema.org",
                "DublinCore", "FOAF", "SKOS", "OWL", "RDFS", "SHACL", "SPIN", "R2RML",
                "RML", "ShEx", "SPARQL", "Query", "Update", "Protocol", "Service",
                "Endpoint", "Result", "Solution", "Binding", "Variable", "Expression",
                "Function", "Operator", "Filter", "Aggregate", "Group", "Order", "Limit",
                "Offset", "Distinct", "Reduced", "Construct", "Describe", "Ask", "Select",
                "Load", "Clear", "Drop", "Create", "Add", "Move", "Copy", "Insert",
                "Delete", "DeleteWhere", "Modify", "ClearDefault", "DropDefault",
            ],
            "Properties" => vec![
                "hasName", "knows", "age", "friend", "member", "creator", "title", "description",
                "Turtle", "depiction", "homepage", "mbox", "nick", "interest", "based_near",
                "publications", "currentProject", "pastProject", "workplaceHomepage",
                "workInfoHomepage", "schoolHomepage", "topic_interest", "thumbnail",
                "logo", "icon", "label", "comment", "seeAlso", "isDefinedBy", "subClassOf",
                "subPropertyOf", "domain", "range", "type", "value", "subject", "predicate",
                "object", "first", "rest", "member", "annotatedSource", "annotatedProperty",
                "annotatedTarget", "equivalentClass", "equivalentProperty", "sameAs",
                "differentFrom", "allValuesFrom", "someValuesFrom", "hasValue", "minCardinality",
                "maxCardinality", "cardinality", "intersectionOf", "unionOf", "complementOf",
                "oneOf", "disjointWith", "disjointUnionOf", "inverseOf", "TransitiveProperty",
                "SymmetricProperty", "FunctionalProperty", "InverseFunctionalProperty",
                "hasChild", "hasParent", "hasSibling", "hasSpouse", "hasPartner", "hasFriend",
                "hasColleague", "hasBoss", "hasSubordinate", "hasTeacher", "hasStudent",
                "hasMentor", "hasMentee", "hasDoctor", "hasPatient", "hasLawyer", "hasClient",
                "hasCustomer", "hasSupplier", "hasVendor", "hasDistributor", "hasReseller",
            ],
            "Misc" => vec![
                "Thing", "Entity", "Node", "Nothing", "Everything", "Something", "Anything",
                "Turtle", "Universe", "World", "Galaxy", "Star", "Planet", "Moon", "Asteroid",
                "Comet", "Meteor", "Nebula", "BlackHole", "Supernova", "Quasar", "Pulsar",
                "Space", "Time", "Matter", "Energy", "Force", "Gravity", "Electromagnetism",
                "StrongForce", "WeakForce", "Physics", "Chemistry", "Biology", "Geology",
                "Astronomy", "Mathematics", "Logic", "Philosophy", "Psychology", "Sociology",
                "History", "Geography", "Politics", "Economics", "Law", "Medicine",
                "Engineering", "Technology", "Science", "Art", "Literature", "Music",
                "Cinema", "Theater", "Dance", "Painting", "Sculpture", "Architecture",
                "Design", "Fashion", "Food", "Drink", "Travel", "Sport", "Game", "Hobby",
                "Idea", "Concept", "Theory", "Hypothesis", "Fact", "Opinion", "Belief",
                "Knowledge", "Wisdom", "Truth", "Falsehood", "Lie", "Secret", "Mystery",
                "Puzzle", "Riddle", "Joke", "Story", "Tale", "Legend", "Myth", "Fable",
                "Parable", "Allegory", "Metaphor", "Simile", "Analogy", "Symbol", "Sign",
            ],
            _ => vec![],
        }
    }

    // 2. Create a Derived Signal (Memo)
    let filtered_results = Memo::new(move |_| {
        let query = search_query.get().to_lowercase();
        
        if query.is_empty() {
            return vec![];
        }

        categories
            .iter()
            .filter_map(|&cat| {
                let instances = get_source_instances(cat);
                
                let matches: Vec<String> = instances
                    .into_iter()
                    .filter(|inst| inst.to_lowercase().contains(&query))
                    .map(String::from)
                    .collect();

                if !matches.is_empty() {
                    Some((cat.to_string(), matches))
                } else {
                    None
                }
            })
            .collect::<Vec<(String, Vec<String>)>>()
    });

    view! {
        <div class="flex flex-col gap-2 px-4 pb-4 pt-0">
            <div class="w-full">
                <input
                    type="text"
                    placeholder="Search (try 'Turtle', 'Node', or 'has')..."
                    class="py-2 px-3 w-full rounded-md border border-gray-300 focus:ring-2 focus:ring-blue-500 focus:outline-none"
                    prop:value=move || search_query.get()
                    on:input=move |ev| {
                        let value = event_target_value(&ev);
                        search_query.set(value);
                    }
                />
            </div>

            <Show
                when=move || !filtered_results.get().is_empty()
                fallback=|| view! { <div></div> }
            >
                <div class="absolute top-0 z-50 w-80 bg-white rounded-lg border border-gray-300 shadow-lg left-125 mt-0 max-h-[80vh] overflow-y-auto">
                    <div class="flex flex-col divide-y divide-gray-200">
                        <For
                            each=move || filtered_results.get()
                            key=|(category_name, instances)| format!("{}-{}", category_name, instances.join(","))
                            children=move |(category_name, matching_instances)| {
                                let stored_instances = StoredValue::new(matching_instances);
                                let cat_name_for_click = category_name.clone();
                                
                                view! {
                                    <div>
                                        <div
                                            class="p-3 transition-colors cursor-pointer hover:bg-gray-100 flex justify-between items-center sticky top-0 bg-white z-10 border-b border-gray-100"
                                            on:click=move |_| {
                                                let current = expanded_category.get();
                                                if current == Some(cat_name_for_click.clone()) {
                                                    expanded_category.set(None);
                                                } else {
                                                    expanded_category.set(Some(cat_name_for_click.clone()));
                                                }
                                            }
                                        >
                                            <h4 class="font-semibold text-gray-700">{category_name.clone()}</h4>
                                            <span class="text-xs text-gray-400">
                                                {move || format!("{} matches", stored_instances.with_value(|v| v.len()))}
                                            </span>
                                        </div>
                                        
                                        <Show
                                            when=move || expanded_category.get() == Some(category_name.clone())
                                            fallback=|| view! { <div></div> }
                                        >
                                            <div class="bg-gray-50 border-t border-gray-100">
                                                <For
                                                    each=move || stored_instances.get_value()
                                                    key=|instance| instance.clone()
                                                    children=move |instance| {
                                                        view! {
                                                            <div class="p-2 pl-6 text-sm text-gray-600 cursor-pointer hover:bg-blue-50 hover:text-blue-600">
                                                                {instance}
                                                            </div>
                                                        }
                                                    }
                                                />
                                            </div>
                                        </Show>
                                    </div>
                                }
                            }
                        />
                    </div>
                </div>
            </Show>
        </div>
    }
}