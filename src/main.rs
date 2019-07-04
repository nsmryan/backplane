extern crate clap;

use clap::{App, Arg, ArgMatches};


// TODO
// the main function should take as input two strings, each specifying
// an endpoint. it should then forward bytes from the input to the output.
// potentially it could merge input streams into a single output stream.
fn main() {
    let matches = App::new("backplane")
        .version("0.1")
        .author("Noah Ryan")
        .about("Route bytes between various interfaces")
        .arg(Arg::with_name("INPUT")
                  .help("Input interface")
                  .short("i")
                  .long("input")
                  .required(true)
                  .multiple(false)
                  .empty_values(false))
        .arg(Arg::with_name("OUTPUT")
                  .help("Output interface")
                  .short("o")
                  .long("output")
                  .required(true)
                  .multiple(true)
                  .empty_values(false))
        .get_matches();

    run(matches);
}

fn run(matches: ArgMatches) {
    let input_name = matches.value_of("INPUT").unwrap().to_string();
    let output_names: Vec<&str> = matches.values_of("OUTPUT").unwrap().collect();

    println!("{:?} -> {:?}", input_name, output_names);
}
