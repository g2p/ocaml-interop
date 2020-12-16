// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

use core::marker::PhantomData;
pub use ocaml_sys::{
    extract_exception, field as field_val, is_block, is_exception_result, is_long, string_val,
    tag_val, wosize_val, Intnat, Uintnat as UIntnat, Value as RawOCaml, EMPTY_LIST, FALSE,
    MAX_FIXNUM, MIN_FIXNUM, NONE, TRUE, UNIT,
};

pub mod tag;

/// `OCaml<OCamlList<T>>` is a reference to an OCaml `list` containing
/// values of type `T`.
pub struct OCamlList<A> {
    _marker: PhantomData<A>,
}

/// `OCaml<OCamlBytes>` is a reference to an OCaml `bytes` value.
///
/// # Note
///
/// Unlike with `OCaml<String>`, there is no validation being performed when converting this
/// value into `String`.
pub struct OCamlBytes {}

/// `OCaml<OCamlInt>` is an OCaml integer (tagged and unboxed) value.
pub type OCamlInt = Intnat;

/// `OCaml<OCamlInt32>` is a reference to an OCaml `Int32.t` (boxed `int32`) value.
pub struct OCamlInt32 {}

/// `OCaml<OCamlInt64>` is a reference to an OCaml `Int64.t` (boxed `int64`) value.
pub struct OCamlInt64 {}

/// `OCaml<OCamlFloat>` is a reference to an OCaml `float` (boxed `float`) value.
pub struct OCamlFloat {}

/// `OCaml<OCamlAbstract>` is a reference to an OCaml value that's opaque to us.
/// Doesn't implement the FromOCaml/ToOCaml/ToRust traits, and no allocator
/// function is provided.
pub struct OCamlAbstract {}
