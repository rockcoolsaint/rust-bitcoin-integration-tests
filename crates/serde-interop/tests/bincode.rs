// SPDX-License-Identifier: CC0-1.0

//! Bincode interoperability tests for `serde_as_consensus`.

mod common;

use common::{TestArray, WithConsensus};

#[test]
fn binary_roundtrip() {
    let original = WithConsensus(TestArray([0xef, 0xbe, 0xad, 0xde]));

    let bytes = bincode::serialize(&original).unwrap();

    let decoded: WithConsensus =
        bincode::deserialize(&bytes).unwrap();

    assert_eq!(decoded, original);
}