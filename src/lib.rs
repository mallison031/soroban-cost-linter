#![feature(rustc_private)]
#![warn(unused_extern_crates)]

dylint_linting::dylint_library!();

// Module declarations for our core cost heuristics
mod unbounded_storage;

#[no_mangle]
pub fn register_lints(_info: &dylint_linting::DylintInfo) -> Vec<Box<dyn rustc_lint::LateLintPass>> {
    vec![
        // Register the individual lints here
        Box::new(unbounded_storage::UnboundedStorage::default()),
    ]
}
