// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the snarkOS library.

// The snarkOS library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkOS library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkOS library. If not, see <https://www.gnu.org/licenses/>.

use crate::storage::{DataMap, Map, Storage};
use snarkvm::dpc::prelude::*;

use anyhow::{anyhow, Result};
use std::path::Path;

#[derive(Debug)]
pub struct ProverState<N: Network> {
    /// The coinbase records of the prover in storage.
    coinbase: CoinbaseState<N>,
}

impl<N: Network> ProverState<N> {
    ///
    /// Opens a new writable instance of `ProverState` from the given storage path.
    ///
    pub fn open_writer<S: Storage, P: AsRef<Path>>(path: P) -> Result<Self> {
        // Open storage.
        let context = N::NETWORK_ID;
        let is_read_only = false;
        let storage = S::open(path, context, is_read_only)?;

        // Initialize the prover.
        let prover = Self {
            coinbase: CoinbaseState::open(storage)?,
        };

        // let value = storage.export()?;
        // println!("{}", value);
        // let storage_2 = S::open(".ledger_2", context)?;
        // storage_2.import(value)?;

        info!("Prover successfully initialized");
        Ok(prover)
    }

    /// Returns `true` if the given commitment exists in storage.
    pub fn contains_coinbase_record(&self, commitment: &N::Commitment) -> Result<bool> {
        self.coinbase.contains_record(commitment)
    }

    /// Returns the coinbase record for a given commitment.
    pub fn get_coinbase_record(&self, commitment: &N::Commitment) -> Result<Record<N>> {
        self.coinbase.get_record(commitment)
    }

    /// Adds the given coinbase record to storage.
    pub fn add_coinbase_record(&self, record: &Record<N>) -> Result<()> {
        self.coinbase.add_record(record)
    }

    /// Removes the given record from storage.
    pub fn remove_coinbase_record(&self, commitment: &N::Commitment) -> Result<()> {
        self.coinbase.remove_record(commitment)
    }
}

#[derive(Clone, Debug)]
#[allow(clippy::type_complexity)]
struct CoinbaseState<N: Network> {
    records: DataMap<N::Commitment, Record<N>>,
}

impl<N: Network> CoinbaseState<N> {
    /// Initializes a new instance of `CoinbaseState`.
    fn open<S: Storage>(storage: S) -> Result<Self> {
        Ok(Self {
            records: storage.open_map("records")?,
        })
    }

    /// Returns `true` if the given commitment exists in storage.
    fn contains_record(&self, commitment: &N::Commitment) -> Result<bool> {
        self.records.contains_key(commitment)
    }

    /// Returns the record for a given commitment.
    fn get_record(&self, commitment: &N::Commitment) -> Result<Record<N>> {
        match self.records.get(commitment)? {
            Some(record) => Ok(record),
            None => return Err(anyhow!("Record with commitment {} does not exist in storage", commitment)),
        }
    }

    /// Adds the given record to storage.
    fn add_record(&self, record: &Record<N>) -> Result<()> {
        // Ensure the record does not exist.
        let commitment = record.commitment();
        if self.records.contains_key(&commitment)? {
            Err(anyhow!("Record with commitment {} already exists in storage", commitment))
        } else {
            // Insert the record.
            self.records.insert(&commitment, &record)?;
            Ok(())
        }
    }

    /// Removes the given record from storage.
    fn remove_record(&self, commitment: &N::Commitment) -> Result<()> {
        // Remove the record entry.
        self.records.remove(commitment)?;
        Ok(())
    }
}