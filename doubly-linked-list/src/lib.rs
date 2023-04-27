// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
use std::ptr::NonNull;

mod pre_implemented;
//use std::{marker::PhantomData, ptr::NonNull};

pub struct LinkedList<T>{
    front: Link<T>,
    back: Link<T>,
    len: usize,
}

struct Node<T> {
    next: Link<T>,
    prev: Link<T>,
    elem: T,
}

type Link<T> = Option<NonNull<Node<T>>>;


pub struct Cursor<'a, T>{
    cur: Link<T>,
    list: &'a mut LinkedList<T>,
}


pub struct Iter<'a, T>(&'a Link<T>);


impl<T> LinkedList<T> {
    pub fn new() -> Self {
        //unimplemented!()
        Self {
            front: None,
            back: None,
            len: 0,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        //unimplemented!()
        self.len == 0
    }

    pub fn len(&self) -> usize {
        //unimplemented!()
        self.len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        //unimplemented!()
        Cursor{
            cur: self.front,
            list: self,
        }
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        //unimplemented!()
        Cursor{
            cur: self.back,
            list: self,
        }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        //unimplemented!()
        Iter(&self.front)
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        //unimplemented!()
        let mut cursor = self.cursor_front();
        while let Some(element) = cursor.take() {
            drop(element);
        };
    }
}

#[cfg(feature = "advanced")]
unsafe impl<T: Send> Send for LinkedList<T> {}

#[cfg(feature = "advanced")]
unsafe impl<T: Sync> Sync for LinkedList<T> {}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        //unimplemented!()
        unsafe{
            self.cur.map(|cur| &mut (*cur.as_ptr()).elem)
        }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        //unimplemented!()
        self.cur = self.cur.and_then(|cur| unsafe{ 
            (*cur.as_ptr()).next
        });
        self.peek_mut()
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        //unimplemented!()
        self.cur = self.cur.and_then(|cur| unsafe{ 
            (*cur.as_ptr()).prev
        });
        self.peek_mut()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        //unimplemented!()
        self.cur.map(|cur| unsafe{
            if self.cur == self.list.front{
                self.list.front = (*cur.as_ptr()).next;
            }
            if self.cur == self.list.back{
                self.list.back = (*cur.as_ptr()).prev;
            }
            if let Some(prex) = (*cur.as_ptr()).prev{
                (*prex.as_ptr()).next = (*cur.as_ptr()).next;
            }
            if let Some(next) = (*cur.as_ptr()).next{
                (*next.as_ptr()).prev = (*cur.as_ptr()).prev;
            }
            self.list.len -= 1;
            self.cur = (*cur.as_ptr()).next.or((*cur.as_ptr()).prev);
            let node = Box::from_raw(cur.as_ptr());
            node.elem

        })

    }

    pub fn insert_after(&mut self, _element: T) {
        //unimplemented!()
        unsafe{
            let node = NonNull::new_unchecked(
                Box::into_raw(Box::new(Node{
                    next: None,
                    prev: None,
                    elem: _element,
                }))
            );
            if let Some(cur) = self.cur{
                let next = (*cur.as_ptr()).next;
                (*node.as_ptr()).next = next;
                (*node.as_ptr()).prev = self.cur;
                (*cur.as_ptr()).next = Some(node);
                if let Some(next_node) = next{
                    (*next_node.as_ptr()).prev = (*cur.as_ptr()).next;
                }
                if self.list.back == self.cur{
                    self.list.back = (*cur.as_ptr()).next;
                }
            }else{
                self.cur = Some(node);
                self.list.front = self.cur;
                self.list.back = self.cur;
            }
            self.list.len += 1;
        }

    }

    pub fn insert_before(&mut self, _element: T) {
        //unimplemented!()
        unsafe{
            let node = NonNull::new_unchecked(
                Box::into_raw(Box::new(Node{
                    elem: _element,
                    next: None,
                    prev: None,
                }))
            );
            if let Some(cur) = self.cur{
                let prev = (*cur.as_ptr()).prev;
                (*node.as_ptr()).prev = prev;
                (*node.as_ptr()).next = self.cur;
                (*cur.as_ptr()).prev = Some(node);

                if let Some(prev_node) = prev{
                    (*prev_node.as_ptr()).next = (*cur.as_ptr()).prev;
                }
                if self.list.front == self.cur{
                    self.list.front = (*cur.as_ptr()).prev;
                }
            }else{
                self.cur = Some(node);
                self.list.front = self.cur;
                self.list.back = self.cur;
            }
            self.list.len += 1;
        }
    }
}


impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        //unimplemented!()
        self.0.map(|cur| unsafe{
            self.0 = &(*cur.as_ptr()).next;
            &(*cur.as_ptr()).elem
        })
    }
}







