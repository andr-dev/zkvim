//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use zkvim_lib::{VimEngineRunner, ZkVimPuzzle};

pub fn main() {
    let input = sp1_zkvm::io::read::<String>();
    let vim = sp1_zkvm::io::read::<String>();
    let output = sp1_zkvm::io::read::<String>();

    sp1_zkvm::io::commit_slice(
        &bincode::serialize(
            &ZkVimPuzzle {
                input: input.clone(),
                output: output.clone()
        }).unwrap()
    );

    let mut engine = VimEngineRunner::new(input);

    match engine.run(vim) {
        Ok(final_text) => {
            sp1_zkvm::io::commit_slice(&[(output == final_text) as u8]);
        }
        Err(e) => {
            sp1_zkvm::io::commit_slice(&[0]);
        }
    }
}
