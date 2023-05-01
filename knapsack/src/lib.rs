pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(_max_weight: u32, _items: &[Item]) -> u32 {
    //unimplemented!("Solve the knapsack exercise");
    let mut maxval = 0;
    for (i,item) in _items.iter().enumerate() {
        if item.weight <= _max_weight {
            let value = item.value+maximum_value(
                _max_weight-item.weight, 
                &_items[i+1..]
            );
            if value > maxval {
                maxval = value;
            }
        }
    }
    maxval
}
