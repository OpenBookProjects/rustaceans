#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

fn get_odd_nucleotide(strand:&str,nucleotides:&str)->Option<usize>{
    strand
        .chars()
        .position(|c| !nucleotides.contains(c))
}

//const NUC: &str = "ACGT";

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        /* unimplemented!("Construct new Dna from '{dna}' string. If string contains invalid nucleotides return index of first invalid nucleotide"); */
        get_odd_nucleotide(dna,"ACGT")
            .map_or(Ok(Dna(dna.to_string())), |i| Err(i))
    }

    pub fn into_rna(self) -> Rna {
        //unimplemented!("Transform Dna {self:?} into corresponding Rna");
        let rna = self
            .0
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => c,
            })
            .collect();
        Rna(rna)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
       /*  unimplemented!("Construct new Rna from '{rna}' string. If string contains invalid nucleotides return index of first invalid nucleotide"); */

        get_odd_nucleotide(rna,"ACGU")
            .map_or(Ok(Rna(rna.to_string())), |i| Err(i))
    }
}

