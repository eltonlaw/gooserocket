pub struct AminoAcid {
    pub codon: Vec<&'static str>,
    pub name: &'static str,
    pub code1: char,
    pub code3: &'static str,
}

pub fn amino_acids() -> Vec<AminoAcid> {
    return vec![
        AminoAcid {
            codon: vec!["GCU", "GCC", "GCA", "GCG"],
            name: "Alanine",
            code1: 'A',
            code3: "Ala",
        },
        AminoAcid {
            codon: vec!["CGU", "CGC", "CGA", "CGG", "AGA", "AGG"],
            name: "Arginine",
            code1: 'R',
            code3: "Arg",
        },
        AminoAcid {
            codon: vec!["AAU", "AAC"],
            name: "Asparagine",
            code1: 'N',
            code3: "Asn",
        },
        AminoAcid {
            codon: vec!["GAU", "GAC"],
            name: "Aspartic acid",
            code1: 'D',
            code3: "Asp",
        },
        AminoAcid {
            codon: vec!["UGU", "UGC"],
            name: "Cysteine",
            code1: 'C',
            code3: "Cys",
        },
        AminoAcid {
            codon: vec!["CAA", "CAG"],
            name: "Glutamine",
            code1: 'Q',
            code3: "Gln",
        },
        AminoAcid {
            codon: vec!["GAA", "GAG"],
            name: "Glutamic acid",
            code1: 'E',
            code3: "Glu",
        },
        AminoAcid {
            codon: vec!["GGU", "GGC", "GGA", "GGG"],
            name: "Glycine",
            code1: 'G',
            code3: "Gly",
        },
        AminoAcid {
            codon: vec!["CAU", "CAC"],
            name: "Histidine",
            code1: 'H',
            code3: "His",
        },
        AminoAcid {
            codon: vec!["AUU", "AUC", "AUA"],
            name: "Isoleucine",
            code1: 'I',
            code3: "Ile",
        },
        AminoAcid {
            codon: vec!["CUU", "CUC", "CUA", "CUG", "UUA", "UUG"],
            name: "Leucine",
            code1: 'L',
            code3: "Leu",
        },
        AminoAcid {
            codon: vec!["AAA", "AAG"],
            name: "Lysine",
            code1: 'K',
            code3: "Lys",
        },
        AminoAcid {
            codon: vec!["AUG"],
            name: "Methionine",
            code1: 'M',
            code3: "Met",
        },
        AminoAcid {
            codon: vec!["UUU", "UUC"],
            name: "Phenylalanine",
            code1: 'F',
            code3: "Phe",
        },
        AminoAcid {
            codon: vec!["CCU", "CCC", "CCA", "CCG"],
            name: "Proline",
            code1: 'P',
            code3: "Pro",
        },
        AminoAcid {
            codon: vec!["UCU", "UCC", "UCA", "UCG", "AGU", "AGC"],
            name: "Serine",
            code1: 'S',
            code3: "Ser",
        },
        AminoAcid {
            codon: vec!["ACU", "ACC", "ACA", "ACG"],
            name: "Threonine",
            code1: 'T',
            code3: "Thr",
        },
        AminoAcid {
            codon: vec!["UGG"],
            name: "Tryptophan",
            code1: 'W',
            code3: "Trp",
        },
        AminoAcid {
            codon: vec!["UAU", "UAC"],
            name: "Tyrosine",
            code1: 'Y',
            code3: "Tyr",
        },
        AminoAcid {
            codon: vec!["GUU", "GUC", "GUA", "GUG"],
            name: "Valine",
            code1: 'V',
            code3: "Val",
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
