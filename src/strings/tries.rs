use crate::common;
use std::ptr::NonNull;

const R: usize = 256;

pub struct TrieST<T> {
    root: Option<NonNull<Node<T>>>,
    n: usize,
}

struct Node<T> {
    val: Option<T>,
    next: Vec<Option<NonNull<Node<T>>>>,
}

impl<'a, 'b, T> TrieST<T> {
    pub fn contains(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    pub fn get(&self, key: &'a str) -> Option<&'b T> {
        get_dth(self.root, key, 0)
    }

    pub fn put(&self, key: &str, val: T) {
        unimplemented!()
    }

    pub fn delete(&mut self, key: &str) {
        let root = self.delete_dth(self.root, key, 0);
        self.root = root;
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_empty(&self) -> bool {
        self.n == 0
    }

    fn delete_dth(
        &mut self,
        x: Option<NonNull<Node<T>>>,
        key: &str,
        d: usize,
    ) -> Option<NonNull<Node<T>>> {
        x.and_then(|mut x| {
            if d == key.len() {
                let val = unsafe { x.as_mut().val.take() };
                if val.is_some() {
                    self.n -= 1;
                }
            } else {
                let i = common::util::byte_at(key, d) as usize;
                unsafe {
                    let next = x.as_ref().next[i];
                    x.as_mut().next[i] = self.delete_dth(next, key, d + 1);
                }
            }

            // remove subtrie rooted at x if it is completely empty
            unsafe {
                if x.as_ref().val.is_some() {
                    return Some(x);
                }
                if x.as_ref().next.iter().any(|it| it.is_some()) {
                    return Some(x);
                }

                // x.val is None and next all None, just release x itself
                let _ = Box::from_raw(x.as_ptr());
            }

            None
        })
    }
}

fn get_dth<'a, 'b, T>(p: Option<NonNull<Node<T>>>, key: &'a str, d: usize) -> Option<&'b T> {
    p.and_then(|p| {
        if d == key.len() {
            unsafe { p.as_ref().val.as_ref() }
        } else {
            let i = common::util::byte_at(key, d) as usize;
            let next = unsafe { p.as_ref().next[i] };
            get_dth(next, key, d + 1)
        }
    })
}

impl<T> Default for TrieST<T> {
    fn default() -> Self {
        Self { root: None, n: 0 }
    }
}

impl<T> Node<T> {
    fn new(val: T) -> NonNull<Self> {
        let v = Box::new(Self {
            val: Some(val),
            next: vec![None; R],
        });
        Box::leak(v).into()
    }
}

impl<T> Default for Node<T> {
    fn default() -> Self {
        Self {
            val: None,
            next: vec![None; R],
        }
    }
}

impl<T> Drop for TrieST<T> {
    fn drop(&mut self) {
        fn visitor<T>(p: Option<NonNull<Node<T>>>) {
            if let Some(p) = p {
                let p = unsafe { Box::from_raw(p.as_ptr()) };
                p.next.iter().for_each(|it| visitor(*it));
            }
        }

        let root = self.root.take();
        visitor(root);
        self.n = 0;
    }
}