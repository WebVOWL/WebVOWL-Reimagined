export default class GlobalPaths {
    /** Global source code path */
    static src = "src";

    /** Global path for static assets */
    static static = `${GlobalPaths.src}/static`;

    /** Path to built-in ontologies */
    static ontology = `${GlobalPaths.static}/ontology`;

    /** Deployment path */
    static deploy = `deploy`;

    /** Path to Cargo.toml */
    static rustSrc = ""; // Empty string means top-level directory

    /** Path to compiled WebAssembly package */
    static pkgWasm = `target/pkg`;

    /** Path to node modules */
    static nodeModules = "node_modules";
}
