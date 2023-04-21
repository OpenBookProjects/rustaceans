// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
const SHARPS: &[&str] = &[
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];
const FLATS: &[&str] = &[
    "C", "Db", "D", "Eb", "E","F", "Gb", "G", "Ab", "A", "Bb", "B" 
];

#[derive(Debug)]
pub enum Error{
    InvalidTonic,
    InvalidInterval
}

pub struct Scale{
    notes: Vec<String>,
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        //unimplemented!("Construct a new scale with tonic {tonic} and intervals {intervals}")
        let chromatic_scale = match tonic {
            "C" | "a" | "G" | "D" | "A" | "E" | "B" | "F#" | "e" | "b" | "f#" | "c#" | "g#" | "d#" => SHARPS,
            "F" | "Bb" | "Eb" | "Ab" | "Db" | "Gb" | "d" | "g" | "c" | "f" | "bb" | "eb" => FLATS,
            _ => return Err(Error::InvalidTonic),
        };
        let mut pos = chromatic_scale
            .iter()
            .position(|&x| x.to_uppercase() == tonic.to_uppercase())
            .unwrap();
        let mut notes = vec![chromatic_scale[pos].to_string()];
        for interval in intervals.chars() {
            match interval {
                'm' => pos += 1,
                'M' => pos += 2,
                'A' => pos += 3,
                _ => return Err(Error::InvalidInterval),
            };
            notes
                .push(chromatic_scale[pos % chromatic_scale.len()]
                    .to_string());
        }
        Ok(Scale { notes })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        //unimplemented!("Construct a new chromatic scale with tonic {tonic}")
        Scale::new(tonic, "mmmmmmmmmmmm")
    }

    pub fn enumerate(&self) -> Vec<String> {
        //unimplemented!()
        self.notes.clone()
    }
}








