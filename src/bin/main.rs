use std::fs;
use std::process;

use isa::instruction::LabeledInstruction;
use isa::memory_model::MemoryModel;
use isa::memory_model::MemoryModelType;
use isa::memory_model::PSO;
use isa::memory_model::SC;
use isa::memory_model::TSO;
use isa::parser::parse_instruction;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,

    #[arg(short, long)]
    trace: bool,

    #[arg(short, long, default_value = "SC")]
    model: String,

    #[arg(short, long)]
    interactive: bool
}

fn main() {
    let args = Args::parse();

    let file_path = args.file;
    let content = fs::read_to_string(file_path.clone())
        .unwrap_or_else(|err| {
            eprintln!("Error reading file {}: {}", file_path, err);
            process::exit(1);
        });

    let memory_model = match &args.model[..] {
        "SC" => MemoryModelType::SC,
        "TSO" => MemoryModelType::TSO,
        "PSO" => MemoryModelType::PSO,
        _ => {
            eprintln!("Invalid memory model. Choose from: SC, TSO, PSO");
            process::exit(1);
        }
    };

    let mut instructions: Vec<Vec<LabeledInstruction>> = Vec::new();
    let mut current_thread = 0;
    instructions.push(Vec::new());
    for line in content.lines() {
        if line.is_empty() {
          instructions.push(Vec::new());
          current_thread += 1;
          continue;
        }
        let instruction = parse_instruction(line)
            .unwrap_or_else(|err| {
                eprintln!("Error parsing instruction {}: {}", line, err);
                process::exit(1);
            });
        instructions[current_thread].push(instruction);
    }

    match memory_model {
        MemoryModelType::SC => {
            let mut model = SC::new(instructions);
            while model.get_possible_executions().len() > 0 {
                model.random_step(args.trace);
            }
        }
        MemoryModelType::TSO => {
            let mut model = TSO::new(instructions);
            while model.get_possible_executions().len() > 0 {
                model.random_step(args.trace);
            }
        }
        MemoryModelType::PSO => {
            let mut model = PSO::new(instructions);
            while model.get_possible_executions().len() > 0 {
                model.random_step(args.trace);
            }
        }
    };
}