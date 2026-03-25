mod hubbard;

use std::env;
use hubbard::*;

fn usage(program_name: &str) {
    eprintln!("Usage: {} -n <n_sites> [-m <m_orbitals>] [-t <hopping>] [-u <interaction>] TREXIO_FILE", program_name);
    eprintln!("  -n <n_sites>     : Number of x sites)");
    eprintln!("  -m <m_orbitals>  : Number of y sites (default 1)");
    eprintln!("  -t <hopping>     : Hopping parameter (default: 1.0)");
    eprintln!("  -u <interaction> : Interaction parameter (default: 1.0)");
    eprintln!("");
    eprintln!("Examples:");
    eprintln!("  {} -n 2 -m 3 -t 1.0 -u 2.0 2x3_t1_u2.h5", program_name);
    eprintln!("  {} -n 4 -m 2 4x2.h5", program_name);  // uses defaults for t and u
    eprintln!("  {} -n 1 -m 1 -t 0.5 out.h5 ", program_name);  // uses default for u
    std::process::exit(1);
}




fn main() {

    let mut args: Vec<String> = env::args().collect();

    let prog_name = args.remove(0);

    if args.len() < 2 {
        usage(&prog_name);
    }

    let mut n = 0;
    let mut m = 1;
    let mut t = 1.0;
    let mut u = 1.0;

    while args.len()>1 {
        match args[0].as_str() {
            "-n" => n = args[1].parse::<usize>().expect("n must be a positive integer"),
            "-m" => m = args[1].parse::<usize>().expect("m must be a positive integer"),
            "-t" => t = args[1].parse::<f64>().expect("t must be a number"),
            "-u" => u = args[1].parse::<f64>().expect("u must be a number"),
            _ => usage(&prog_name),
        }
        let _ = args.remove(0);
        let _ = args.remove(0);
    }

    if args.len() == 0 {
        usage(&prog_name)
    };

    let f = &args[0];

    if n == 0 {
        eprintln!("Error: -n is required");
        usage(&prog_name);
    }


    println!("Hubbard Model Integrals");
    println!("");
    println!("  Sites : {} x {}", n, m);
    println!("  t = {}", t);
    println!("  U = {}", u);
    println!("");
    println!("  File: {}", f);
    println!("");

    let model = HubbardModel::new(n,m,t,u);
    model.write(f);

}
