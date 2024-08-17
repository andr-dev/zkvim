use alloy_sol_types::SolType;
use clap::Parser;
use zkvim_lib::{VimEngineRunner, ZkVimPuzzle};
use sp1_sdk::{ProverClient, SP1Stdin};

pub const ZKVIM_ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(long)]
    input: String,

    #[clap(long)]
    vim: String,

    #[clap(long)]
    output: String,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::new();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&args.input);
    stdin.write(&args.vim);
    stdin.write(&args.output);

    println!("vim: {}", args.vim);

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(ZKVIM_ELF, stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read the output.
        let decoded: ZkVimPuzzle = bincode::deserialize(output.as_slice()).unwrap();
        let ZkVimPuzzle { input, output } = decoded;
        println!("input: {}", input);
        println!("output: {}", output);

        let expected_output = {
            let mut engine = VimEngineRunner::new(input);

            engine.run(args.vim).unwrap()
        };
        
        assert_eq!(output, expected_output);
        println!("Values are correct!");

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(ZKVIM_ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, stdin)
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
