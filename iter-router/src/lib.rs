
struct IterRouter<'a, I: Iterator> {
    pub source: Option<&'a mut I>,
}

impl<'a, I: Iterator> Iterator for IterRouter<'a, I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.as_mut().unwrap().next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_router() {
        let mut r = IterRouter::<std::ops::Range<i32>> {
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
}
