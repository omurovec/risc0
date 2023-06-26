// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]

use json::parse;
use ipfs_core::Outputs;
use risc0_zkvm::{
    guest::env,
};
use cid::multihash::{Code, MultihashDigest};
use cid::Cid;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let data: String = env::read();
    let key: String = env::read();
    let hash = Code::Sha2_256.digest(data.as_bytes());
    let cid = Cid::new_v1(0x55, hash);
    let mut data = parse(&data).unwrap();

    for key_fragment in key.split('.') {
        data = data[key_fragment].clone();
    }

    let proven_val = data.to_string();
    let out = Outputs {
        data: proven_val,
        hash: cid.to_string(),
    };
    env::commit(&out);
}
