use std::collections::HashMap;

pub struct CodonsInfo<'a>(HashMap<&'a str,&'a str>);

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        /* unimplemented!(
            "Return the protein name for a '{codon}' codon or None, if codon string is invalid"
        ); */
        self.0.get(codon).cloned()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        /* unimplemented!("Return a list of protein names that correspond to the '{rna}' RNA string or None if the RNA string is invalid");
    } */
    rna.chars()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|c| self.name_for(
                &c.iter().collect::<String>()))
        .take_while(|opt| opt.filter(
                |name| *name=="stop condon").is_none())
        .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    /* 
    unimplemented!("Construct a new CodonsInfo struct from given pairs: {pairs:?}"); 
    */
    CodonsInfo( pairs.into_iter().collect() )
}
