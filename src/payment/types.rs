// LNP/BP Core Library implementing LNPBP specifications & standards
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::fmt::Debug;

use lnpbp::chain::AssetId;
use strict_encoding::{
    self, strict_deserialize, strict_serialize, StrictDecode, StrictEncode,
};

use crate::{channel, extension};

/// Shorthand for representing asset - amount pairs
pub type AssetsBalance = BTreeMap<AssetId, u64>;

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
    Display,
    StrictEncode,
    StrictDecode,
)]
#[display(Debug)]
pub enum ExtensionId {
    /// The channel itself
    Channel,

    Bolt3,
    Eltoo,
    Taproot,

    Htlc,
    Ptlc,
    ShutdownScript,
    AnchorOut,
    Dlc,
    Lightspeed,

    Bip96,
    Rgb,
}

impl Default for ExtensionId {
    fn default() -> Self {
        ExtensionId::Channel
    }
}

impl From<ExtensionId> for u16 {
    fn from(id: ExtensionId) -> Self {
        let mut buf = [0u8; 2];
        buf.copy_from_slice(
            &strict_serialize(&id)
                .expect("Enum in-memory strict encoding can't fail"),
        );
        u16::from_be_bytes(buf)
    }
}

impl TryFrom<u16> for ExtensionId {
    type Error = strict_encoding::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        strict_deserialize(&value.to_be_bytes())
    }
}

impl extension::Nomenclature for ExtensionId {}

#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
    Display,
    StrictEncode,
    StrictDecode,
)]
#[display(Debug)]
#[non_exhaustive]
pub enum TxType {
    HtlcSuccess,
    HtlcTimeout,
    Unknown(u16),
}

impl From<TxType> for u16 {
    fn from(ty: TxType) -> Self {
        match ty {
            TxType::HtlcSuccess => 0x0,
            TxType::HtlcTimeout => 0x1,
            TxType::Unknown(x) => x,
        }
    }
}

impl From<u16> for TxType {
    fn from(ty: u16) -> Self {
        match ty {
            0x00 => TxType::HtlcSuccess,
            0x01 => TxType::HtlcTimeout,
            x => TxType::Unknown(x),
        }
    }
}

impl channel::TxRole for TxType {}

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
#[derive(
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
    Display,
    StrictEncode,
    StrictDecode,
)]
#[display(Debug)]
#[non_exhaustive]
#[repr(u8)]
pub enum Lifecycle {
    Initial,
    Proposed,                 // Sent or got `open_channel`
    Accepted,                 // Sent or got `accept_channel`
    Funding,                  // One party signed funding tx
    Signed,                   // Other peer signed funding tx
    Funded,                   // Funding tx is published but not mined
    Locked,                   // Funding tx mining confirmed by one peer
    Active,                   // Both peers confirmed lock, channel active
    Reestablishing,           // Reestablishing connectivity
    Shutdown,                 // Shutdown proposed but not yet accepted
    Closing { round: usize }, // Shutdown agreed, exchanging `closing_signed`
    Closed,                   // Cooperative closing
    Aborted,                  // Non-cooperative unilateral closing
}

impl Default for Lifecycle {
    fn default() -> Self {
        Lifecycle::Initial
    }
}
