use isogram::check;

fn main(){
    println!("isogram: {}", check("isograms"));
//下面是一些使用 .all() 方法的示例:
//检查一个字符串中的所有字符是否都是数字:
let s = "123456";
let result = s.chars().all(|c| c.is_numeric());
    println!("result: {}", result);

//检查一个字符串中的所有单词是否都长度不超过 5 个字符:
let s = "The quick brown fox";
let result = s.split_whitespace().all(|word| word.len() <= 5);
    println!("result: {}", result);

//检查一个字符串中的所有字符都是大写
let s = "HELLO";
let result = s.chars().all(|c| c.is_uppercase());
    println!("result: {}", result);

//检查一个数组中的所有元素是否都是偶数
let arr = [2, 4, 6, 8, 10];
let result = arr.iter().all(|x| x % 2 == 0);
    println!("result: {}", result);

//请注意, 如果空字符串或空数组， .all() 会返回 true.



}