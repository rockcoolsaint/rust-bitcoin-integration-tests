// SPDX-License-Identifier: CC0-1.0

//! JSON interoperability tests for `serde_as_consensus`.

use consensus_encoding::{
    ArrayDecoder, ArrayEncoder, Decode, Decoder, DecoderStatus, Encode,
    UnexpectedEofError,
};
use serde::{Deserialize, Serialize};

/// A fixed-size byte array that implements consensus encoding.
#[derive(Debug, Clone, PartialEq, Eq)]
struct TestArray<const N: usize>([u8; N]);

impl<const N: usize> Encode for TestArray<N> {
    type Encoder<'e> = ArrayEncoder<N>
    where
        Self: 'e;

    fn encoder(&self) -> Self::Encoder<'_> {
        ArrayEncoder::without_length_prefix(self.0)
    }
}

#[derive(Default)]
struct TestArrayDecoder<const N: usize>(ArrayDecoder<N>);

impl<const N: usize> Decoder for TestArrayDecoder<N> {
    type Output = TestArray<N>;
    type Error = UnexpectedEofError;

    fn push_bytes(&mut self, bytes: &mut &[u8]) -> Result<DecoderStatus, Self::Error> {
        self.0.push_bytes(bytes)
    }

    fn end(self) -> Result<Self::Output, Self::Error> {
        self.0.end().map(TestArray)
    }

    fn read_limit(&self) -> usize {
        self.0.read_limit()
    }
}

impl<const N: usize> Decode for TestArray<N> {
    type Decoder = TestArrayDecoder<N>;
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
struct WithConsensus(
    #[serde(with = "consensus_encoding::serde_as_consensus")]
    TestArray<4>,
);

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