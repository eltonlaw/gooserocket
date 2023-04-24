/// One letter code for amino acids
pub enum AminoAcid1LetterCode {
    A,
    R,
    N,
    D,
    C,
    Q,
    E,
    G,
    H,
    I,
    L,
    K,
    M,
    F,
    P,
    S,
    T,
    W,
    Y,
    V,
}

/// Three letter code for amino acids
pub enum AminoAcid3LetterCode {
    Ala,
    Arg,
    Asn,
    Asp,
    Cys,
    Gln,
    Glu,
    Gly,
    His,
    Ile,
    Leu,
    Lys,
    Met,
    Phe,
    Pro,
    Ser,
    Thr,
    Trp,
    Tyr,
    Val,
}

pub struct AminoAcid {
    pub codon: Vec<&'static str>,
    pub name: &'static str,
    pub code1: AminoAcid1LetterCode,
    pub code3: AminoAcid3LetterCode,
}

pub fn amino_acids() -> Vec<AminoAcid> {
    return vec![
        AminoAcid {
            codon: vec!["GCU", "GCC", "GCA", "GCG"],
            name: "Alanine",
            code1: AminoAcid1LetterCode::A,
            code3: AminoAcid3LetterCode::Ala,
        },
        AminoAcid {
            codon: vec!["CGU", "CGC", "CGA", "CGG", "AGA", "AGG"],
            name: "Arginine",
            code1: AminoAcid1LetterCode::R,
            code3: AminoAcid3LetterCode::Arg,
        },
        AminoAcid {
            codon: vec!["AAU", "AAC"],
            name: "Asparagine",
            code1: AminoAcid1LetterCode::N,
            code3: AminoAcid3LetterCode::Asn,
        },
        AminoAcid {
            codon: vec!["GAU", "GAC"],
            name: "Aspartic acid",
            code1: AminoAcid1LetterCode::D,
            code3: AminoAcid3LetterCode::Asp,
        },
        AminoAcid {
            codon: vec!["UGU", "UGC"],
            name: "Cysteine",
            code1: AminoAcid1LetterCode::C,
            code3: AminoAcid3LetterCode::Cys,
        },
        AminoAcid {
            codon: vec!["CAA", "CAG"],
            name: "Glutamine",
            code1: AminoAcid1LetterCode::Q,
            code3: AminoAcid3LetterCode::Gln,
        },
        AminoAcid {
            codon: vec!["GAA", "GAG"],
            name: "Glutamic acid",
            code1: AminoAcid1LetterCode::E,
            code3: AminoAcid3LetterCode::Glu,
        },
        AminoAcid {
            codon: vec!["GGU", "GGC", "GGA", "GGG"],
            name: "Glycine",
            code1: AminoAcid1LetterCode::G,
            code3: AminoAcid3LetterCode::Gly,
        },
        AminoAcid {
            codon: vec!["CAU", "CAC"],
            name: "Histidine",
            code1: AminoAcid1LetterCode::H,
            code3: AminoAcid3LetterCode::His,
        },
        AminoAcid {
            codon: vec!["AUU", "AUC", "AUA"],
            name: "Isoleucine",
            code1: AminoAcid1LetterCode::I,
            code3: AminoAcid3LetterCode::Ile,
        },
        AminoAcid {
            codon: vec!["CUU", "CUC", "CUA", "CUG", "UUA", "UUG"],
            name: "Leucine",
            code1: AminoAcid1LetterCode::L,
            code3: AminoAcid3LetterCode::Leu,
        },
        AminoAcid {
            codon: vec!["AAA", "AAG"],
            name: "Lysine",
            code1: AminoAcid1LetterCode::K,
            code3: AminoAcid3LetterCode::Lys,
        },
        AminoAcid {
            codon: vec!["AUG"],
            name: "Methionine",
            code1: AminoAcid1LetterCode::M,
            code3: AminoAcid3LetterCode::Met,
        },
        AminoAcid {
            codon: vec!["UUU", "UUC"],
            name: "Phenylalanine",
            code1: AminoAcid1LetterCode::F,
            code3: AminoAcid3LetterCode::Phe,
        },
        AminoAcid {
            codon: vec!["CCU", "CCC", "CCA", "CCG"],
            name: "Proline",
            code1: AminoAcid1LetterCode::P,
            code3: AminoAcid3LetterCode::Pro,
        },
        AminoAcid {
            codon: vec!["UCU", "UCC", "UCA", "UCG", "AGU", "AGC"],
            name: "Serine",
            code1: AminoAcid1LetterCode::S,
            code3: AminoAcid3LetterCode::Ser,
        },
        AminoAcid {
            codon: vec!["ACU", "ACC", "ACA", "ACG"],
            name: "Threonine",
            code1: AminoAcid1LetterCode::T,
            code3: AminoAcid3LetterCode::Thr,
        },
        AminoAcid {
            codon: vec!["UGG"],
            name: "Tryptophan",
            code1: AminoAcid1LetterCode::W,
            code3: AminoAcid3LetterCode::Trp,
        },
        AminoAcid {
            codon: vec!["UAU", "UAC"],
            name: "Tyrosine",
            code1: AminoAcid1LetterCode::Y,
            code3: AminoAcid3LetterCode::Tyr,
        },
        AminoAcid {
            codon: vec!["GUU", "GUC", "GUA", "GUG"],
            name: "Valine",
            code1: AminoAcid1LetterCode::V,
            code3: AminoAcid3LetterCode::Val,
        },
    ];
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
