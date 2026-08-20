// Copyright (c) 2026, Maptek Pty Ltd
// Licensed under the MIT License

//! `cobalt_renderer_sys` provides C API bindings for the Cobalt Renderer. Use the `cobalt_renderer`
//! crate for a safer Rust API.
//!
//! For more detailed information on using this crate, see the docs for the `cobalt_renderer` crate.

// Bindings will have these issues which we will allow
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Binary directory where plugins can be found during local development
///
/// # IMPORTANT
///
/// This path is the absolute path to the bin directory of the Cobalt SDK
/// on the current machine. You should only use this variable for local builds.
/// For distribution your program should bundle the required shared libraries and access
/// them at a different path.
///
/// This is provided as a convenience for development and for examples/tests
pub const LOCAL_RUNTIME_BIN_DIR: &str = env!("COBALT_RUNTIME_BIN_DIR");
