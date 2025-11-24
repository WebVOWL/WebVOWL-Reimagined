pub mod owl {
    use horned_owl::io::{
        ofn::{reader as ofn_reader, writer as ofn_writer},
        owx::{reader as owx_reader, writer as owx_writer},
        rdf::reader as rdf_reader,
    };

    pub struct OwlTools;

    impl OwlTools {
        pub fn owl_to_rdf() {}

        pub fn rdf_to_owl() {}
    }
}
