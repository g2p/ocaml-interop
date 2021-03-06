// Copyright (c) SimpleStaking and Tezedge Contributors
// SPDX-License-Identifier: MIT

use ocaml_sys::{caml_shutdown, caml_startup};
use std::marker::PhantomData;

use crate::{memory::GCFrame, value::make_ocaml, OCaml, OCamlRooted};

/// OCaml runtime handle.
pub struct OCamlRuntime {
    _private: (),
}

impl OCamlRuntime {
    /// Initializes the OCaml runtime and returns an OCaml runtime handle.
    pub fn init() -> Self {
        OCamlRuntime::init_persistent();
        unsafe { Self::recover_handle() }
    }

    /// Initializes the OCaml runtime.
    pub fn init_persistent() {
        let arg0 = "ocaml".as_ptr() as *const i8;
        let c_args = vec![arg0, core::ptr::null()];
        unsafe { caml_startup(c_args.as_ptr()) }
    }

    /// Recover the runtime handle.
    ///
    /// This method is used internally, do not use directly in code, only when writing tests.
    ///
    /// # Safety
    ///
    /// This function is unsafe because the OCaml runtime handle should be obtained once
    /// upon initialization of the OCaml runtime and then passed around. This method exists
    /// only to ease the authoring of tests.
    pub unsafe fn recover_handle() -> Self {
        OCamlRuntime { _private: () }
    }

    /// Release the OCaml runtime lock, call `f`, and re-acquire the OCaml runtime lock.
    pub fn releasing_runtime<T, F>(&mut self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        OCamlBlockingSection::new().perform(f)
    }

    /// Performs the necessary cleanup and shuts down the OCaml runtime.
    pub fn shutdown(self) {
        unsafe { caml_shutdown() }
    }

    /// Produces a token that can be used to recover the OCaml runtime handle.
    ///
    /// # Safety
    ///
    /// Meant to be used internally when calling allocation functions, do not use
    /// directly.
    pub unsafe fn token(&self) -> OCamlAllocToken {
        OCamlAllocToken {
            _marker: PhantomData,
        }
    }

    #[doc(hidden)]
    pub fn open_frame<'a, 'gc>(&'a self) -> GCFrame<'gc> {
        Default::default()
    }

    /// Returns the OCaml valued to which this GC tracked reference points to.
    pub fn get<'tmp, T>(&'tmp self, reference: &OCamlRooted<T>) -> OCaml<'tmp, T> {
        make_ocaml(reference.cell.get())
    }
}

struct OCamlBlockingSection {}

impl OCamlBlockingSection {
    fn new() -> Self {
        Self {}
    }

    fn perform<T, F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        unsafe { ocaml_sys::caml_enter_blocking_section() };
        f()
    }
}

impl Drop for OCamlBlockingSection {
    fn drop(&mut self) {
        unsafe { ocaml_sys::caml_leave_blocking_section() };
    }
}

/// Token used by allocation functions. Used internally.
pub struct OCamlAllocToken<'a> {
    _marker: PhantomData<&'a i32>,
}

impl<'a> OCamlAllocToken<'a> {
    /// Recover the runtime handle from this token.
    ///
    /// # Safety
    ///
    /// It is important that functions that make use of this method of
    /// recovering the function handle are only called with the [`ocaml_alloc!`]
    /// and [`ocaml_call!`] macros to perform the necessary bookkeeping operations
    /// to enforce the correctness of OCaml value lifetimes.
    pub unsafe fn recover_runtime_handle(self) -> OCamlRuntime {
        OCamlRuntime::recover_handle()
    }
}
