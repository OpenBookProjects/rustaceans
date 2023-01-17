use std::collections::HashMap;

const DNA:[char;4] = ['A','C','G','T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    /* unimplemented!(
        "How much of nucleotide type '{}' is contained inside DNA string '{}'?",
        nucleotide,
        dna
    ); */

    if !DNA.contains(&nucleotide){
        return Err(nucleotide);
    }
    
    if let Some(c) = dna
                        .chars()
                        .find(|x| !DNA.contains(x)){
                            return Err(c);
                        }
    
    Ok(dna
        .chars()
        .filter(|&x| x == nucleotide)
        .count() as usize
    )

}


pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
/*     unimplemented!(
        "How much of every nucleotide type is contained inside DNA string '{}'?",
        dna
    ); */
    let mut result: HashMap<char,usize> = HashMap::new();
    for c in DNA{
        result
            .insert(c, count(c,dna)?);
    }
    Ok(result)
}
