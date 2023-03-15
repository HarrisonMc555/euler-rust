mod digits;
mod euler;
mod say;

#[macro_use]
extern crate itertools;

use clap::Parser;

/// Run a specific problem from Project Euler
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Which Project Euler problem to run
    #[clap(value_parser)]
    problem: usize,
}

fn main() {
    let args = Args::parse();
    let problem = args.problem;
    let main = match get_main(problem) {
        None => {
            eprintln!("No implementation found for problem {}", problem);
            return;
        }
        Some(main) => main,
    };
    main();
}

fn get_main(problem: usize) -> Option<Box<dyn Fn()>> {
    Some(Box::new(match problem {
        1 => euler::euler1::main,
        2 => euler::euler2::main,
        3 => euler::euler3::main,
        4 => euler::euler4::main,
        5 => euler::euler5::main,
        6 => euler::euler6::main,
        7 => euler::euler7::main,
        8 => euler::euler8::main,
        9 => euler::euler9::main,
        10 => euler::euler10::main,
        11 => euler::euler11::main,
        12 => euler::euler12::main,
        13 => euler::euler13::main,
        14 => euler::euler14::main,
        15 => euler::euler15::main,
        16 => euler::euler16::main,
        17 => euler::euler17::main,
        18 => euler::euler18::main,
        19 => euler::euler19::main,
        20 => euler::euler20::main,
        21 => euler::euler21::main,
        22 => euler::euler22::main,
        23 => euler::euler23::main,
        24 => euler::euler24::main,
        25 => euler::euler25::main,
        26 => euler::euler26::main,
        27 => euler::euler27::main,
        28 => euler::euler28::main,
        29 => euler::euler29::main,
        30 => euler::euler30::main,
        31 => euler::euler31::main,
        32 => euler::euler32::main,
        61 => euler::euler61::main,
        62 => euler::euler62::main,
        63 => euler::euler63::main,
        64 => euler::euler64::main,
        65 => euler::euler65::main,
        67 => euler::euler67::main,
        _ => return None,
    }))
}
