use std::{iter::Peekable, rc::Rc};


struct IterRouter<'a, I: Iterator> {
    pub source: Option<&'a mut I>,
}

impl<'a, I: Iterator> Iterator for IterRouter<'a, I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.as_mut().unwrap().next()
    }
}

struct Rec<I: Iterator, F: Fn(&I::Item) -> Peekable<I>> {
    source: Option<Box<Rec<I, F>>>,
    graph: Rc<F>,
    buffer: Peekable<I>,
}

impl<I: Iterator, F: Fn(&I::Item) -> Peekable<I>> Iterator for Rec<I, F> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.buffer.next().or_else(|| {
            if let Some(source) = self.source.as_mut() {
                if let Some(x) = source.next() {

                    // If x has children, add a level of recursion
                    let mut next_level = (self.graph)(&x);
                    if next_level.peek().is_some() {
                        let current_source = self.source.take();
                        self.source = Some(Box::new(Rec {
                            source: current_source,
                            graph: Rc::clone(&self.graph),
                            buffer: next_level,
                        }));
                    }

                    return Some(x)
                }
            }
            None
        })
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::*;

    #[test]
    fn test_router() {
        let mut r = IterRouter::<Range<i32>> {
            source: None
        };

        let mut c1 = 0..4;
        let mut c2 = 4..8;

        let mut r1 = &mut c1;
        let mut r2 = &mut c2;

        r.source = Some(r1);
        assert_eq!(r.next(), Some(0));
        assert_eq!(r.next(), Some(1));

        r1 = r.source.replace(r2).unwrap();
        assert_eq!(r.next(), Some(4));
        assert_eq!(r.next(), Some(5));

        r2 = r.source.replace(r1).unwrap();
        assert_eq!(r.next(), Some(2));
        assert_eq!(r.next(), Some(3));
        assert_eq!(r.next(), None);

        r.source = Some(&mut r2);
        assert_eq!(r.next(), Some(6));
        assert_eq!(r.next(), Some(7));
        assert_eq!(r.next(), None);
    }

    #[test]
    fn test_recurse() {
        let start: Range<i32> = 1..2;
        let neighbors = |x: &i32| (10*x..std::cmp::min(10*x + 2, 1000)).peekable();
        let neighbors = Rc::new(neighbors);

        let recurse = Rec {
            source: Some(Box::new(Rec {
               source: None,
               graph: Rc::clone(&neighbors),
               buffer: start.clone().peekable(),
            })),
            graph: Rc::clone(&neighbors),
            buffer: (0..0).peekable(),
        };

        let result: Vec<_> = recurse.collect();
        dbg!(result);
    }
}
