mod common_collections;
mod common_programming_concepts;
mod enums;
mod error_handling;
mod generic_types_traits_and_lifetimes;
mod structures;
mod understanding_ownership;

use common_collections::{hash_maps::hash_maps, vectors::vectors};
use common_programming_concepts::{
    control_flows::control_flows, data_types::data_types, functions::functions,
    variables_and_mutability::variables_and_mutability,
};
use enums::enums::enums;
use error_handling::recoverable_errors::recoverable_errors;
use generic_types_traits_and_lifetimes::{
    generic_types::generic_types, lifetimes::lifetimes, traits::traits,
};
use structures::{struct_method_syntax::struct_method_syntax, structs::structs};
use understanding_ownership::{
    ownership::ownership, references_and_borrowing::references_and_borrowing,
    slice_type::slice_type,
};

fn main() {
    lifetimes();
    traits();
    generic_types();

    recoverable_errors();

    hash_maps();
    vectors();

    enums();

    struct_method_syntax();
    structs();

    slice_type();
    references_and_borrowing();
    ownership();

    control_flows();
    functions();
    data_types();
    variables_and_mutability();
}
