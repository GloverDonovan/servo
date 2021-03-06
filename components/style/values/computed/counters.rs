/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Computed values for counter properties

use values::generics::counters::CounterIncrement as GenericCounterIncrement;
use values::generics::counters::CounterReset as GenericCounterReset;

/// A computed value for the `counter-increment` property.
pub type CounterIncrement = GenericCounterIncrement<i32>;

/// A computed value for the `counter-increment` property.
pub type CounterReset = GenericCounterReset<i32>;
