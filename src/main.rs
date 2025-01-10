use std::io::{self, Read};
use discombobugraph::Discombobugraph;

fn main() -> io::Result<()> {
    let mut buffer = Vec::new();
    io::stdin().read_to_end(&mut buffer)?;

    let analyzer = Discombobugraph::new();
    let results = analyzer.run(buffer);

    println!("Shannon : {:.6}", results[0]);
    println!("Freq    : {:.6}", results[1]);
    println!("Runs    : {:.6}", results[2]);
    println!("Pairs   : {:.6}", results[3]);
    println!("χ2      : {:.6}", results[4]);
    println!("AC (1)  : {:.6}", results[5]);
    println!("AC (4)  : {:.6}", results[6]);
    println!("AC (8)  : {:.6}", results[7]);
    println!("AC (10%): {:.6}", results[8]);
    println!("AC (25%): {:.6}", results[9]);
    println!("AC (50%): {:.6}", results[10]);

    Ok(())
}
