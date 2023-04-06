pub struct RailFence{
    rails: usize
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        //unimplemented!("Construct a new fence with {rails} rails")
        RailFence{rails: rails as usize}
    }


    pub fn encode(&self, text: &str) -> String {
        //unimplemented!("Encode this text: {text}")
        let mut result = vec![Vec::new(); self.rails];
        for (c,i) in text
                .chars()
                .zip(zigzag(self.rails)) {
            result[i].push(c);
        }
        result
            .into_iter()
            .flat_map(|v| v)
            .collect::<String>()
    }

    pub fn decode(&self, cipher: &str) -> String {
        //unimplemented!("Decode this ciphertext: {cipher}")
        let mut indexes: Vec<_> = zigzag(self.rails)
            .zip(1..)
            .take(cipher.len())
            .collect();
        indexes.sort();
        let mut char_with_index:Vec<_> = cipher
            .chars()
            .zip(indexes)
            .map(|(c,(_,i))| (i,c))
            .collect();
        char_with_index.sort();
        char_with_index
            .into_iter()
            .map(|(_,c)| c)
            .collect()
    }

}

fn zigzag(n:usize) -> impl Iterator<Item = usize>{
    (0..n-1).chain((1..n).rev()).cycle()
}

