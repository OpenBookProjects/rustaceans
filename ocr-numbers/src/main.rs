use ocr_numbers as ocr;

fn main(){
    let input = " _ \n".to_string() +
                "| |\n" +
                "|_|\n" +
                "   ";

    println!("{:?}",ocr::convert(&input));

}

