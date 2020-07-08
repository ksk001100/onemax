mod ga;

use ga::*;
use seahorse::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("onemax [flag]")
        .flag(
            Flag::new("generation", FlagType::Int)
                .alias("gen")
                .usage("--generation, -gen <int>"),
        )
        .flag(
            Flag::new("gene_length", FlagType::Int)
                .alias("g_len")
                .usage("--gene_length, -g_len <int>"),
        )
        .flag(
            Flag::new("individual_length", FlagType::Int)
                .alias("i_len")
                .usage("--individual_length, -i_len <int>"),
        )
        .flag(
            Flag::new("mutate_rate", FlagType::Float)
                .alias("m_rate")
                .usage("--mutate_rate, -m_rate <float>"),
        )
        .flag(
            Flag::new("elite_rate", FlagType::Float)
                .alias("e_rate")
                .usage("--elite_rate, -e_rate <float>"),
        )
        .action(action);

    app.run(args);
}

fn action(c: &Context) {
    let generation = c.int_flag("generation").unwrap_or_else(|_| 100);
    let gene_length = c.int_flag("gene_length").unwrap_or_else(|_| 100) as usize;
    let individual_length = c.int_flag("individual_length").unwrap_or_else(|_| 10) as usize;
    let mutate_rate = c.float_flag("mutate_rate").unwrap_or_else(|_| 0.6);
    let elite_rate = c.float_flag("elite_rate").unwrap_or_else(|_| 0.2);

    println!("Generations       : {}", generation);
    println!("Gene length       : {}", gene_length);
    println!("Individual length : {}", individual_length);
    println!("Mutate rate       : {}", mutate_rate);
    println!("Elite rate        : {}\n", elite_rate);

    let mut pop = Population::new(gene_length, individual_length, mutate_rate, elite_rate);

    pop.evaluate();
    println!("Generation        : 0");
    println!("Max               : {}", pop.max().rank);
    println!("Min               : {}", pop.min().rank);
    println!("-----------------------------");

    for gen in 1..generation {
        pop.evolution();
        println!("Generation        : {}", gen);
        println!("Max               : {}", pop.max().rank);
        println!("Min               : {}", pop.min().rank);
        println!("-----------------------------");
    }
}
