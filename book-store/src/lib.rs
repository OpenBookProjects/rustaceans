pub fn lowest_price(books: &[u32]) -> u32 {
    //unimplemented!("Find the lowest price of the bookbasket with books {books:?}")
    let discounted = [0,800,1520,2160,2560,3000];
    let mut baskets: Vec<Vec<u32>> = Vec::new();

    for book in books{
        match baskets
            .iter_mut()
            .filter(|b| !b.contains(book))
            .min_by_key(|b| discounted[b.len()+1]-discounted[b.len()])
        {
            Some(basket) => basket.push(*book),
            None => baskets.push(vec![*book]),
        }
    }
    baskets.iter().map(|b| discounted[b.len()]).sum()

}
