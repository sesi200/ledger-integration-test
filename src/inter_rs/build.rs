use ic_cdk_bindgen::{Builder, Config};
use std::path::PathBuf;

const TYPE_ATTRIBUTES: &str = "#[derive(CandidType, Deserialize, Debug, PartialEq, Eq)]";

fn main() {
    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("Cannot find manifest dir"));
    let mut counter = Config::new("counter_rs");
    counter.binding.type_attributes = TYPE_ATTRIBUTES.to_string();
    let mut ledger = Config::new("ledger");
    ledger.binding.type_attributes = TYPE_ATTRIBUTES.to_string();
    let mut builder = Builder::new();
    builder.add(counter);
    builder.add(ledger);
    builder.build(Some(manifest_dir.join("declarations")));
}
