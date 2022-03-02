#![cfg(feature = "bincode")]

use std::fmt::Debug;

/// taken from bincode/tests/std.rs & bincode/tests/utils.rs
pub trait TheSameTrait: bincode::Encode + bincode::Decode + Debug + 'static {}
impl<T> TheSameTrait for T where T: bincode::Encode + bincode::Decode + Debug + 'static {}

#[test]
fn map() {
    let mut map = hashbrown::HashMap::new();
    map.insert("Hello".to_owned(), "world".to_owned());
    map.insert("How".to_owned(), "are".to_owned());
    map.insert("you".to_owned(), "doing?".to_owned());
    the_same(map);
}

#[test]
fn set() {
    let mut set = hashbrown::HashSet::new();
    set.insert("Hello".to_owned());
    set.insert("world".to_owned());
    set.insert("How".to_owned());
    set.insert("are".to_owned());
    set.insert("you".to_owned());
    set.insert("doing?".to_owned());
    the_same(set);
}

/// taken from bincode/tests/std.rs & bincode/tests/utils.rs
#[allow(dead_code)]
pub fn the_same<V: TheSameTrait + PartialEq>(element: V) {
    the_same_with_comparer(element, |a, b| a == b);
}

/// taken from bincode/tests/std.rs & bincode/tests/utils.rs
pub fn the_same_with_comparer<V, CMP>(element: V, cmp: CMP)
    where
        V: TheSameTrait,
        CMP: Fn(&V, &V) -> bool,
{
    // A matrix of each different config option possible
    the_same_with_config(
        &element,
        bincode::config::standard()
            .with_little_endian()
            .with_fixed_int_encoding()
            .skip_fixed_array_length(),
        &cmp,
    );
    the_same_with_config(
        &element,
        bincode::config::standard()
            .with_big_endian()
            .with_fixed_int_encoding()
            .skip_fixed_array_length(),
        &cmp,
    );
    the_same_with_config(
        &element,
        bincode::config::standard()
            .with_little_endian()
            .with_variable_int_encoding()
            .skip_fixed_array_length(),
        &cmp,
    );
    the_same_with_config(
        &element,
        bincode::config::standard()
            .with_big_endian()
            .with_variable_int_encoding()
            .skip_fixed_array_length(),
        &cmp,
    );
    the_same_with_config(
        &element,
        bincode::config::standard()
            .with_little_endian()
            .with_fixed_int_encoding(),
        &cmp,
    );
    the_same_with_config(
        &element,
        bincode::config::standard()
            .with_big_endian()
            .with_fixed_int_encoding(),
        &cmp,
    );
    the_same_with_config(
        &element,
        bincode::config::standard()
            .with_little_endian()
            .with_variable_int_encoding(),
        &cmp,
    );
    the_same_with_config(
        &element,
        bincode::config::standard()
            .with_big_endian()
            .with_variable_int_encoding(),
        &cmp,
    );
}

/// taken from bincode/tests/std.rs & bincode/tests/utils.rs
fn the_same_with_config<V, C, CMP>(element: &V, config: C, cmp: CMP)
    where
        V: TheSameTrait,
        C: bincode::config::Config,
        CMP: Fn(&V, &V) -> bool,
{
    let mut buffer = [0u8; 2048];
    let len = bincode::encode_into_slice(&element, &mut buffer, config).unwrap();
    println!(
        "{:?} ({}): {:?} ({:?})",
        element,
        core::any::type_name::<V>(),
        &buffer[..len],
        core::any::type_name::<C>()
    );
    let (decoded, decoded_len): (V, usize) =
        bincode::decode_from_slice(&mut buffer, config).unwrap();

    assert!(
        cmp(&element, &decoded),
        "Comparison failed\nDecoded:  {:?}\nExpected: {:?}\nBytes: {:?}",
        decoded,
        element,
        &buffer[..len],
    );
    assert_eq!(len, decoded_len);
}