use wordy::answer;

fn main(){
    let command = "What is 33 divided by -3?";
    assert_eq!(Some(-11), answer(command));

}


/*
不完全相同。words.next()?.next()?.ok()? 会访问迭代器中的下一个元素两次，即跳过前两个元素，然后访问第三个元素。这个操作可能会引起一些问题，因为在调用两次 next() 方法之前需要先调用一次 next() 方法，以确保迭代器被移动到正确的位置。

另一方面，words.nth(1)?.ok()? 方法是 Rust 标准库中专门为这种情况设计的方法，它直接访问迭代器中的第二个元素，并返回可选值。这个方法比 words.next()?.next()?.ok()? 更为清晰和简洁，因为它直接表达了我们要跳过前两个元素并访问第三个元素的意图。

因此，在这个具体的实现中，使用 words.nth(1)?.ok()? 更为合适。
*/

