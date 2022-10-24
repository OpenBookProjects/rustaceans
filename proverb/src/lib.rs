pub fn build_proverb(list: &[&str]) -> String {
    //unimplemented!("build a proverb from this list of items: {:?}", list)

    let mut proverb = String::new();
    let mut buffer: String;
    let last = match list.first() {
        Some(first) => format!("And all for the want of a {}.", first),
        None => String::new(),
    };

    if !list.is_empty() {
        for index in 0..list.len() - 1 {
            buffer = format!(
                "For want of a {} the {} was lost.\n",
                list[index],
                list[index + 1]
            );
            proverb.push_str(&buffer);
        }
    }
    proverb.push_str(&last);

    proverb
}
