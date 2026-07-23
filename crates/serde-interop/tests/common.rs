use consensus_encoding::{
    ArrayDecoder, ArrayEncoder, Decode, Decoder, DecoderStatus, Encode,
    UnexpectedEofError,
};
use serde::{Deserialize, Serialize};

/// A fixed-size byte array that implements consensus encoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestArray<const N: usize>(pub [u8; N]);

impl<const N: usize> Encode for TestArray<N> {
    type Encoder<'e> = ArrayEncoder<N>
    where
        Self: 'e;

    fn encoder(&self) -> Self::Encoder<'_> {
        ArrayEncoder::without_length_prefix(self.0)
    }
}

#[derive(Default)]
pub struct TestArrayDecoder<const N: usize>(ArrayDecoder<N>);

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
pub struct WithConsensus(
    #[serde(with = "consensus_encoding::serde_as_consensus")]
    pub TestArray<4>,
);
