use std::iter::{self,FromIterator};// for iter self
//use std::iter::FromIterator;

type Link<T> = Option<Box<Node<T>>>;

pub struct SimpleLinkedList<T> {
    // Delete this field
    // dummy is needed to avoid unused parameter error during compilation
    //dummy: ::std::marker::PhantomData<T>,
    head: Link<T>,
}

struct Node<T>{
    elem:T,
    next: Link<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        //unimplemented!()
        SimpleLinkedList{head: None}
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        //unimplemented!()
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        //unimplemented!()
        let mut current_node = &self.head;
        let mut size = 0;
        while let Some(x) = current_node{
            size +=1 ;
            current_node=&x.next;
        }
        size
    }

    pub fn push(&mut self, _element: T) {
        //unimplemented!()
        let crt_node:Box<Node<T>> = Box::new(Node{
            elem: _element,
            next: self.head.take(),
        });
        self.head = Some(crt_node)
    }

    pub fn pop(&mut self) -> Option<T> {
        //unimplemented!()
        self.head.take().map(|node: Box<Node<T>>|{
            self.head = node.next;
            node.elem // need echo this?
        })
    }

    pub fn peek(&self) -> Option<&T> {
        //unimplemented!()
        self.head.as_ref().map(|node: &Box<Node<T>>|{ // point Box not take out
            &node.elem
        })
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        //unimplemented!()
        let mut ret = SimpleLinkedList::new();
        while let Some(x) = self.pop(){
            ret.push(x);
        }
        ret
    }
}
/* 
// 创建一个空的 SimpleLinkedList
let mut commands = SimpleLinkedList::new();

// 添加一些命令
commands.push("打开文件");
commands.push("编写代码");
commands.push("运行程序");

// 按照添加的顺序执行命令
while let Some(command) = commands.pop() {
    execute(command);
}

usage peek
let list = SimpleLinkedList::from_iter(vec![1, 2, 3, 4, 5]);
if let Some(head) = list.peek() {
    println!("The first element is: {}", head);
} else {
    println!("The list is empty!");
}

and usage FromIterator

let vec = vec![1, 2, 3];
let linked_list: SimpleLinkedList<i32> = vec.into_iter().collect();
 */

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        //unimplemented!()
        _iter.into_iter().fold(SimpleLinkedList::new(),|mut l,e|{
            l.push(e);
            l // same echo needed
        })
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        //unimplemented!()
        let mut vec = iter::from_fn(|| _linked_list.pop()).collect::<Vec<_>>();
        vec.reverse(); // if there had rpop, can canncel this
        vec
    }
}
/*


and usage from for Vec

let linked_list = SimpleLinkedList::from_iter(vec![1, 2, 3]);
let vec = Vec::from(linked_list.into_iter());
 */
