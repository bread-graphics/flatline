// This file is part of flatline.
//
// flatline is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option)
// any later version.
//
// flatline is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General
// Public License along with flatline. If not, see
// <https://www.gnu.org/licenses/>.

#![no_std]
#![forbid(unsafe_code, rust_2018_idioms)]

//! Flatline is a utility crate for converting paths into
//! `Polygon`s representing the stroke of the path.

use core::iter::{FusedIterator, once, empty};
use geometry::{Edge, PathEvent, Scalar, Point2D};

pub fn edge_strokes<Num: Scalar>(
    path_events: impl IntoIterator<Item = PathEvent<Point2D<Num>, Point2D<Num>>>,
    config: StrokeConfig,
) -> impl FusedIterator<Item = Edge<Num>> {
    empty()
}

pub struct StrokeConfig {

}