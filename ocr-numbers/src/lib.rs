// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
/* use std::collections::HashMap;
const ocr_map:[(&str, char); 10] = [
    (" _ | ||_|", '0'),
    ("     |  |", '1'),
    (" _  _||_ ", '2'),
    (" _  _| _|", '3'),
    ("   |_|  |", '4'),
    (" _ |_  _|", '5'),
    (" _ |_ |_|", '6'),
    (" _   |  |", '7'),
    (" _ |_||_|", '8'),
    (" _ |_| _|", '9'),
];
 */
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    //unimplemented!("Convert the input '{input}' to a string");
    let mut ocr_map = HashMap::<&str, char>::new();
    ocr_map.insert(" _ | ||_|   ", '0');
    ocr_map.insert("     |  |   ", '1');
    ocr_map.insert(" _  _||_    ", '2');
    ocr_map.insert(" _  _| _|   ", '3');
    ocr_map.insert("   |_|  |   ", '4');
    ocr_map.insert(" _ |_  _|   ", '5');
    ocr_map.insert(" _ |_ |_|   ", '6');
    ocr_map.insert(" _   |  |   ", '7');
    ocr_map.insert(" _ |_||_|   ", '8');
    ocr_map.insert(" _ |_| _|   ", '9');
    // ... 可以添加更多的键值对


    let rows = input.lines().collect::<Vec<_>>();
    let row_count = rows.len();

    if row_count % 4 != 0 {
        return Err(Error::InvalidRowCount(row_count));
    } else if rows.iter().any(|row| row.len() % 3 != 0) {
        return Err(Error::InvalidColumnCount(rows[0].len()));
    }

    let mut result = String::with_capacity(input.len()/12);

    for n_row in rows.chunks(4) {
        for col in (0..n_row[0].len()).step_by(3) {
            let num: String = n_row
                .iter()
                .flat_map(|row| row[col..col+3].chars())
                .collect();
            println!("->{}<-", num.clone());
            println!("OCR=> {:?}", *ocr_map.get(num.as_str()).unwrap_or(&'?'));
            
            result.push(*ocr_map.get(num.as_str()).unwrap_or(&'?'));
        }
        result.push(',');
    }
    result.pop();
    Ok(result)

}








