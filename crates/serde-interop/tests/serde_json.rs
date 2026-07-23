// SPDX-License-Identifier: CC0-1.0

//! JSON interoperability tests for `serde_as_consensus`.

mod common;

use common::{TestArray, WithConsensus};

#[test]
fn serialize_array_bytes_as_hex_json() {
    let value = WithConsensus(TestArray([0xef, 0xbe, 0xad, 0xde]));

    let json = serde_json::to_string(&value).unwrap();

    assert_eq!(json, "\"efbeadde\"");
}

#[test]
fn deserialize_hex_json_into_array() {
    let json = "\"efbeadde\"";

    let decoded: WithConsensus = serde_json::from_str(json).unwrap();

    assert_eq!(
        decoded,
        WithConsensus(TestArray([0xef, 0xbe, 0xad, 0xde]))
    );
}

#[test]
fn deserialize_invalid_hex_json() {
    let json = "\"zzbeadde\"";

    let err = serde_json::from_str::<WithConsensus>(json).unwrap_err();

    assert!(
        err.to_string().contains("hex")
            || err.to_string().contains("decode")
            || err.to_string().contains("invalid")
    );
}

#[test]
fn deserialize_odd_length_hex_json() {
    let json = "\"efbeadd\"";

    let result: Result<WithConsensus, _> = serde_json::from_str(json);

    assert!(result.is_err());
}

#[test]
fn deserialize_too_short_hex_json() {
    let json = "\"efbead\"";

    let result: Result<WithConsensus, _> = serde_json::from_str(json);

    assert!(result.is_err());
}

#[test]
fn deserialize_too_long_hex_json() {
    let json = "\"efbeadde00\"";

    let result: Result<WithConsensus, _> = serde_json::from_str(json);

    assert!(result.is_err());
}