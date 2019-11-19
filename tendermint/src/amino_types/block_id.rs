use std::convert::TryFrom;

use super::validate::{ConsensusMessage, ValidationError, ValidationErrorKind::*};
use crate::block::parts;
use crate::{
    block,
    error::Error,
    hash,
    hash::{Hash, SHA256_HASH_SIZE},
};

#[derive(Clone, PartialEq, Message)]
pub struct BlockId {
    #[prost(bytes, tag = "1")]
    pub hash: Vec<u8>,
    #[prost(message, tag = "2")]
    pub parts_header: Option<PartsSetHeader>,
}

impl BlockId {
    pub fn new(hash: Vec<u8>, parts_header: Option<PartsSetHeader>) -> Self {
        BlockId { hash, parts_header }
    }
}

impl block::ParseId for BlockId {
    fn parse_block_id(&self) -> Result<block::Id, Error> {
        let hash = Hash::new(hash::Algorithm::Sha256, &self.hash)?;
        let parts_header = self
            .parts_header
            .as_ref()
            .and_then(PartsSetHeader::parse_parts_header);
        Ok(block::Id::new(hash, parts_header))
    }
}

impl TryFrom<&block::Id> for BlockId {
    type Error = ();
    fn try_from(bid: &block::Id) -> Result<Self, ()> {
        match bid.hash.as_bytes() {
            None => Err(()),
            Some(bid_hash) => match &bid.parts {
                Some(parts) => Ok(BlockId::new(
                    bid_hash.to_vec(),
                    PartsSetHeader::try_from(parts).ok(),
                )),
                None => Ok(BlockId::new(bid_hash.to_vec(), None)),
            },
        }
    }
}

impl ConsensusMessage for BlockId {
    fn validate_basic(&self) -> Result<(), ValidationError> {
        // Hash can be empty in case of POLBlockID in Proposal.
        if !self.hash.is_empty() && self.hash.len() != SHA256_HASH_SIZE {
            return Err(InvalidHashSize.into());
        }
        self.parts_header
            .as_ref()
            .map_or(Ok(()), ConsensusMessage::validate_basic)
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct CanonicalBlockId {
    #[prost(bytes, tag = "1")]
    pub hash: Vec<u8>,
    #[prost(message, tag = "2")]
    pub parts_header: Option<CanonicalPartSetHeader>,
}

impl block::ParseId for CanonicalBlockId {
    fn parse_block_id(&self) -> Result<block::Id, Error> {
        let hash = Hash::new(hash::Algorithm::Sha256, &self.hash)?;
        let parts_header = self
            .parts_header
            .as_ref()
            .and_then(CanonicalPartSetHeader::parse_parts_header);
        Ok(block::Id::new(hash, parts_header))
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct PartsSetHeader {
    #[prost(int64, tag = "1")]
    pub total: i64,
    #[prost(bytes, tag = "2")]
    pub hash: Vec<u8>,
}

impl PartsSetHeader {
    pub fn new(total: i64, hash: Vec<u8>) -> Self {
        PartsSetHeader { total, hash }
    }
}

impl TryFrom<&parts::Header> for PartsSetHeader {
    type Error = ();
    fn try_from(parts: &parts::Header) -> Result<Self, ()> {
        parts.hash.as_bytes().map_or(Err(()), |bytes| {
            Ok(PartsSetHeader::new(parts.total as i64, bytes.to_vec()))
        })
    }
}

impl PartsSetHeader {
    fn parse_parts_header(&self) -> Option<block::parts::Header> {
        Hash::new(hash::Algorithm::Sha256, &self.hash)
            .map(|hash| block::parts::Header::new(self.total as u64, hash))
            .ok()
    }
}

impl ConsensusMessage for PartsSetHeader {
    fn validate_basic(&self) -> Result<(), ValidationError> {
        if self.total < 0 {
            return Err(NegativeTotal.into());
        }
        // Hash can be empty in case of POLBlockID.PartsHeader in Proposal.
        if !self.hash.is_empty() && self.hash.len() != SHA256_HASH_SIZE {
            return Err(InvalidHashSize.into());
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct CanonicalPartSetHeader {
    #[prost(bytes, tag = "1")]
    pub hash: Vec<u8>,
    #[prost(int64, tag = "2")]
    pub total: i64,
}

impl CanonicalPartSetHeader {
    fn parse_parts_header(&self) -> Option<block::parts::Header> {
        Hash::new(hash::Algorithm::Sha256, &self.hash)
            .map(|hash| block::parts::Header::new(self.total as u64, hash))
            .ok()
    }
}
