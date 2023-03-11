use std::{path::{PathBuf}, fs::{self}, process::exit};

use clap::Parser;

/// A program to translate a DNA sequence to mRNA or aminoacids
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// The input file for processing, use - for stdin
   #[arg(short, long)]
   file: PathBuf,

   /// The output file, use - for stdout
   #[arg(short, long)]
   output: PathBuf,

   /// Whether to translate only to mRNA
   #[arg(short, long)]
   rna: bool,
}

fn nucleotide_to_rna(nucleotide: char) -> Result<char, std::io::Error>{
    match nucleotide {
        'C' => Ok('G'),
        'G' => Ok('C'),
        'T' => Ok('A'),
        'A' => Ok('U'),
        _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
}

fn transcribe(dna_raw: &str) -> Result<String, std::io::Error> {

    let dna = dna_raw.replace('\n', "");

    if dna.len() % 3 != 0 {
        return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
    }

    let dna_code = dna.to_uppercase();
    let mut rna: String = String::new();

    for nucleotide in dna_code.chars() {
        let rna_nucl = nucleotide_to_rna(nucleotide);
        match rna_nucl {
            Ok(_) => rna.push(rna_nucl.unwrap()),
            Err(_) => return Err(rna_nucl.unwrap_err()),
        }
    }

    Ok(rna)
}

fn codon_to_acid(codon: Vec<char>) -> Result<String, std::io::Error> {
    let codon_str = codon.iter().collect::<String>().to_uppercase();
    match codon_str.as_str() {
        "UUU" => Ok("Phe".into()),
        "UUC" => Ok("Phe".into()),
        "UUA" => Ok("Leu".into()),
        "UUG" => Ok("Leu".into()),
        "CUU" => Ok("Leu".into()),
        "CUC" => Ok("Leu".into()),
        "CUA" => Ok("Leu".into()),
        "CUG" => Ok("Leu".into()),
        "AUU" => Ok("Ile".into()),
        "AUC" => Ok("Ile".into()),
        "AUA" => Ok("Ile".into()),
        "AUG" => Ok("Met".into()),
        "GUU" => Ok("Val".into()),
        "GUC" => Ok("Val".into()),
        "GUA" => Ok("Val".into()),
        "GUG" => Ok("Val".into()),
        "UCU" => Ok("Ser".into()),
        "UCC" => Ok("Ser".into()),
        "UCA" => Ok("Ser".into()),
        "UCG" => Ok("Ser".into()),
        "CCU" => Ok("Pro".into()),
        "CCC" => Ok("Pro".into()),
        "CCA" => Ok("Pro".into()),
        "CCG" => Ok("Pro".into()),
        "ACU" => Ok("Thr".into()),
        "ACC" => Ok("Thr".into()),
        "ACA" => Ok("Thr".into()),
        "ACG" => Ok("Thr".into()),
        "GCU" => Ok("Ala".into()),
        "GCC" => Ok("Ala".into()),
        "GCA" => Ok("Ala".into()),
        "GCG" => Ok("Ala".into()),
        "UAU" => Ok("Tyr".into()),
        "UAC" => Ok("Tyr".into()),
        "UAA" => Ok("-".into()),
        "UAG" => Ok("-".into()),
        "CAU" => Ok("His".into()),
        "CAC" => Ok("His".into()),
        "CAA" => Ok("Gln".into()),
        "CAG" => Ok("Gln".into()),
        "AAU" => Ok("Asn".into()),
        "AAC" => Ok("Asn".into()),
        "AAA" => Ok("Lys".into()),
        "AAG" => Ok("Lys".into()),
        "GAU" => Ok("Asp".into()),
        "GAC" => Ok("Asp".into()),
        "GAA" => Ok("Glu".into()),
        "GAG" => Ok("Glu".into()),
        "UGU" => Ok("Cys".into()),
        "UGC" => Ok("Cys".into()),
        "UGA" => Ok("-".into()),
        "UGG" => Ok("Trp".into()),
        "CGU" => Ok("Arg".into()),
        "CGC" => Ok("Arg".into()),
        "CGA" => Ok("Arg".into()),
        "CGG" => Ok("Arg".into()),
        "AGU" => Ok("Ser".into()),
        "AGC" => Ok("Ser".into()),
        "AGA" => Ok("Arg".into()),
        "AGG" => Ok("Arg".into()),
        "GGU" => Ok("Gly".into()),
        "GGC" => Ok("Gly".into()),
        "GGA" => Ok("Gly".into()),
        "GGG" => Ok("Gly".into()),
        _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
}

fn translate(rna_code: &str) -> Result<String, std::io::Error>{
    let rna = rna_code.replace('\n', "");

    if rna.len() % 3 != 0 || !rna.is_ascii() {
        return Err(std::io::Error::from(std::io::ErrorKind::InvalidData));
    }

    let rna_code = rna.to_uppercase();

    let mut acids_list: Vec<String> = vec![];
    for codon in rna_code.chars().collect::<Vec<char>>().chunks(3){
        let acid = codon_to_acid(codon.to_vec());
        
        match acid {
            Ok(_) => acids_list.push(acid.unwrap()),
            Err(_) => return Err(std::io::Error::from(std::io::ErrorKind::InvalidData))
        }
    }

    Ok(acids_list.iter().cloned().collect())
}


fn get_input(input_file: &str) -> String {
    let mut input = String::new();
    if input_file == "-" {
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error when reading stdin: {}", e);
                exit(-1);
            },
        }

    } else {
        input = match fs::read_to_string(input_file) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error when reading {}: {}", input_file, e);
                exit(-1);
            }
        };
    }
    input
    
}

fn write_output(output_file: &str, output: &str) {
    if output_file == "-" {
        println!("{output}");
            
    } else {
        match fs::write(output_file, output) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error when writing {}: {}", output_file, e)
            },
        }
    }
    
}

fn main() {
    let args = Args::parse();
    let dna_source = get_input(args.file.as_os_str().to_str().unwrap_or_default());
    
    let rna = match transcribe(&dna_source) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error when transcribing DNA: {}", e);
            exit(-1);
        }
    };
    if args.rna {
        write_output(args.output.to_str().unwrap_or_default(), &rna);
    }

    let amino_acids = match translate(&rna) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error when translating mRNA: {}", e);
            exit(-1);
        }
    };
    write_output(args.output.to_str().unwrap_or_default(), &amino_acids)
}

#[cfg(test)]
mod tests {
    use crate::*;


    #[test]
    /// Test the DNA to mRNA transcription function
    fn rna() {
        assert_eq!("GCCAUGCCA", transcribe("cggtacggt").unwrap());
        assert_eq!("GCCAUGCCA", transcribe("CGGTACGGT").unwrap());
        assert!(transcribe("gccgc").is_err());
        assert!(transcribe("abcdef").is_err());
    }

    #[test]
    /// Test the mRNA to amino-acids translation function
    fn acids() {
        assert_eq!("CysAlaGly", translate("UGUGCAGGA").unwrap());
        assert_eq!("CysAlaGly", translate("ugugcagga").unwrap());
        assert!(translate("cggtacggt").is_err());
        assert!(translate("cggtacgg").is_err());
        assert!(translate("abcdef").is_err());
    }
}