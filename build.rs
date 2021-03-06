// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate rustc_version;

use rustc_version::{version_meta, Channel};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    match version_meta().unwrap().channel {
        Channel::Nightly => {
            println!("cargo:rustc-cfg=has_specialisation");
        }
        _ => (),
    }
}
