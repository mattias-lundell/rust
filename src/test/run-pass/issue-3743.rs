// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// If `Mul` used an associated type for its output, this test would
// work more smoothly.
#![feature(associated_types, default_type_params, old_orphan_check)]

use std::ops::Mul;

struct Vec2 {
    x: f64,
    y: f64
}

impl Copy for Vec2 {}

// methods we want to export as methods as well as operators
impl Vec2 {
#[inline(always)]
    fn vmul(self, other: f64) -> Vec2 {
        Vec2 { x: self.x * other, y: self.y * other }
    }
}

// Right-hand-side operator visitor pattern
trait RhsOfVec2Mul<Result> { fn mul_vec2_by(&self, lhs: &Vec2) -> Result; }

// Vec2's implementation of Mul "from the other side" using the above trait
impl<Res, Rhs: RhsOfVec2Mul<Res>> Mul<Rhs> for Vec2 {
    type Output = Res;

    fn mul(self, rhs: Rhs) -> Res { rhs.mul_vec2_by(&self) }
}

// Implementation of 'f64 as right-hand-side of Vec2::Mul'
impl RhsOfVec2Mul<Vec2> for f64 {
    fn mul_vec2_by(&self, lhs: &Vec2) -> Vec2 { lhs.vmul(*self) }
}

// Usage with failing inference
pub fn main() {
    let a = Vec2 { x: 3.0f64, y: 4.0f64 };

    // the following compiles and works properly
    let v1: Vec2 = a * 3.0f64;
    println!("{} {}", v1.x, v1.y);

    // the following compiles but v2 will not be Vec2 yet and
    // using it later will cause an error that the type of v2
    // must be known
    let v2 = a * 3.0f64;
    println!("{} {}", v2.x, v2.y); // error regarding v2's type
}
