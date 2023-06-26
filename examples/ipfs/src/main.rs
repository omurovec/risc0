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

use ipfs_core::Outputs;
use ipfs_methods::SEARCH_JSON_IPFS_ELF;
use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Executor, ExecutorEnv,
};

fn main() {
    let data = include_str!("../res/example.json");
    let key = "obj_field.string_subfield";
    let outputs = search_json(data, key);
    println!();
    println!("  {:?}", outputs.hash);
    println!(
        "provably contains a field {} with value {}",
        key,
        outputs.data
    );
}

fn search_json(data: &str, key: &str) -> Outputs {
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&data).unwrap())
        .add_input(&to_vec(&key).unwrap())
        .build()
        .unwrap();

    let mut exec = Executor::from_elf(env, SEARCH_JSON_IPFS_ELF).unwrap();
    let session = exec.run().unwrap();
    let receipt = session.prove().unwrap();

    from_slice(receipt.get_journal()).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn main() {
        let data = include_str!("../res/example.json");
        let key = "obj_field.string_subfield";
        let outputs = super::search_json(data, key);
        assert_eq!(
            outputs.data, "hello world",
            "Did not find the expected value in the obj_field.string_subfield field"
        );
    }
}
