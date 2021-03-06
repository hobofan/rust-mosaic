// Copyright 2018 OpenST Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This module provides an API to interact with blockchains, e.g. Ethereum.

use self::types::address::Address;

mod ethereum;
pub mod types;

/// Kind only represents what kind a blockchain is without any implementation.
pub enum Kind {
    Eth,
}

/// A blockchain is a connection to a blockchain.
pub enum Blockchain {
    Eth(ethereum::Ethereum),
}

impl Blockchain {
    /// Creates a new blockchain of the given kind pointing to the given address.
    ///
    /// # Arguments
    ///
    /// * `kind` - The kind that the blockchain shall be.
    /// * `address` - The address of a node of the blockchain.
    pub fn new(kind: &Kind, address: &str) -> Self {
        match *kind {
            Kind::Eth => Blockchain::Eth(ethereum::Ethereum::new(&address)),
        }
    }

    /// Returns all accounts on this blockchain.
    pub fn get_accounts(&self) -> Vec<Address> {
        match self {
            Blockchain::Eth(ethereum) => ethereum.get_accounts(),
        }
    }
}
