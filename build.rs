// Copyright (C) 2020 Scott Lamb <slamb@slamb.org>
// SPDX-License-Identifier: Apache-2.0

fn main() {
    println!("cargo:rustc-link-lib=tensorflowlite_c");
    if cfg!(feature = "edgetpu") {
        println!("cargo:rustc-link-lib=edgetpu");
    }
}
