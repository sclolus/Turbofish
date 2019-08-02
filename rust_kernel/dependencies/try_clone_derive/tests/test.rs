#![feature(try_reserve)]
extern crate try_clone_derive;
use try_clone_derive::TryClone;
extern crate alloc;

use fallible_collections::TryClone;

#[derive(Debug, TryClone, PartialEq)]
struct Unit;

#[derive(Debug, TryClone, PartialEq)]
struct Struct {
    a: u32,
    b: u32,
}

#[derive(Debug, TryClone, PartialEq)]
struct TupleStruct(u32, Vec<u32>);

#[test]
fn test_unit() {
    let s = Unit;
    assert_eq!(s, s.try_clone().unwrap());
}

#[test]
fn test_struct() {
    let s = Struct { a: 42, b: 42 };
    assert_eq!(s, s.try_clone().unwrap());
}

#[test]
fn test_tuple_struct() {
    let s = TupleStruct(42, vec![42; 42]);
    assert_eq!(s, s.try_clone().unwrap());
}

#[derive(Debug, TryClone, PartialEq)]
enum Enum {
    A,
    B(Struct),
    C(Struct, Vec<u8>),
    D { e: Struct, f: Vec<u8> },
}

#[test]
fn test_enum_simple() {
    let s = Enum::A;
    assert_eq!(s, s.try_clone().unwrap());
}

#[test]
fn test_enum_tuple() {
    let s = Enum::B(Struct { a: 42, b: 42 });
    assert_eq!(s, s.try_clone().unwrap());
}

#[test]
fn test_enum_multi_tuble() {
    let s = Enum::C(Struct { a: 42, b: 42 }, vec![42; 100]);
    assert_eq!(s, s.try_clone().unwrap());
}

#[test]
fn test_enum_struct() {
    let s = Enum::D {
        e: Struct { a: 42, b: 42 },
        f: vec![42; 100],
    };
    assert_eq!(s, s.try_clone().unwrap());
}
