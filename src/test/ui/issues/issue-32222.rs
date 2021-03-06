// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// compile-pass
// skip-codegen
#![allow(warnings)]

mod foo {
    pub fn bar() {}
}

pub use foo::*;
use b::bar;

mod foobar {
    use super::*;
}

mod a {
    pub mod bar {}
}

mod b {
    pub use a::bar;
}


fn main() {}
