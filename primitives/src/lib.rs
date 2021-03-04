// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019  BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

//! Primitives used by the Parachains Tick, Trick and Track.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use core::convert::TryFrom;

use sp_runtime::{
	generic,
	traits::{BlakeTwo256, IdentifyAccount, Verify},
	MultiSignature, RuntimeDebug,
};
use sp_std::vec::Vec;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;

/// Opaque block header type.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Opaque block type.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// Opaque block identifier type.
pub type BlockId = generic::BlockId<Block>;
/// An index to a block.
pub type BlockNumber = u64;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;

/// Balance of an account.
pub type Balance = u128;
pub type Amount = i128;

/// Index of a transaction in the chain.
pub type Index = u64;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Digest item type.
pub type DigestItem = generic::DigestItem<Hash>;

#[derive(Encode, Decode, Eq, PartialEq, Copy, Clone, RuntimeDebug, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CurrencyId {
	LAMI = 0,
	AUSD,
	DOT,
	FEUR,
	FJPY,
	FBTC,
	FETH,
	FAUD,
	FCAD,
	FCHF,
	FXAU,
	FOIL,
	KILT,
}

impl TryFrom<Vec<u8>> for CurrencyId {
	type Error = ();
	fn try_from(v: Vec<u8>) -> Result<CurrencyId, ()> {
		match v.as_slice() {
			b"LAMI" => Ok(CurrencyId::LAMI),
			b"AUSD" => Ok(CurrencyId::AUSD),
			b"DOT" => Ok(CurrencyId::DOT),
			b"FEUR" => Ok(CurrencyId::FEUR),
			b"FJPY" => Ok(CurrencyId::FJPY),
			b"FBTC" => Ok(CurrencyId::FBTC),
			b"FETH" => Ok(CurrencyId::FETH),
			b"FAUD" => Ok(CurrencyId::FAUD),
			b"FCAD" => Ok(CurrencyId::FCAD),
			b"FCHF" => Ok(CurrencyId::FCHF),
			b"FXAU" => Ok(CurrencyId::FXAU),
			b"FOIL" => Ok(CurrencyId::FOIL),
			b"KILT" => Ok(CurrencyId::KILT),
			_ => Err(()),
		}
	}
}
