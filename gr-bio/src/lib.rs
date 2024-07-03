use bio::io::fasta;
use std::io;

pub fn analyze(file: &String) {
    println!("Reading: {}", file);
    let mut reader = fasta::Reader::from_file(file).expect("Error during fasta file opening");

    let mut nb_reads = 0;
    let mut nb_bases = 0;

    for result in reader.records() {
        let record = result.expect("Error during fasta record parsing");
        println!("{}", record.id());

        nb_reads += 1;
        nb_bases += record.seq().len();
    }

    println!("Number of reads: {}", nb_reads);
    println!("Number of bases: {}", nb_bases);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let aa = AMINO_ACIDS.first().unwrap();
        assert_eq!('A', aa.code1);
    }
}
