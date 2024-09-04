use std::{
    borrow::Borrow,
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Bound, Deref, Range, RangeBounds},
    rc::Rc,
};

#[derive(Clone)]
pub struct ShareStr {
    data: Rc<str>,
    range: Range<usize>,
}

impl ShareStr {
    pub fn new(s: &str) -> Self {
        ShareStr {
            data: s.into(),
            range: 0..s.len(),
        }
    }

    pub fn recognize(&self, rec: &str) -> Option<Self> {
        let rec_ptr = rec.as_ptr() as usize;
        let data_ptr = self.data.as_ptr() as usize;

        let start = rec_ptr.checked_sub(data_ptr)?;
        let end = start + rec.len();
        let range = start..end;

        if start < self.range.start || end > self.range.end {
            return None;
        }

        debug_assert!(self.data.get(range.clone()).is_some());

        Some(Self {
            data: self.data.clone(),
            range,
        })
    }

    pub fn as_str(&self) -> &str {
        &self.data[self.range.clone()]
    }

    pub fn clone_range<T>(&self, range: T) -> Self
    where
        T: RangeBounds<usize>,
    {
        let mut this = self.clone();

        this.range.start += match range.start_bound() {
            Bound::Excluded(&b) => b.checked_add(1).expect("exceed usize bound"),
            Bound::Included(&b) => b,
            Bound::Unbounded => 0,
        };

        this.range.end = match range.end_bound() {
            Bound::Excluded(&b) => self.range.start + b,
            Bound::Included(&b) => self.range.start + b.checked_add(1).expect("exceed usize bound"),
            Bound::Unbounded => self.range.end,
        };

        assert!(
            self.data.get(this.range.clone()).is_some(),
            "range is invalid"
        );
        this
    }
}

impl Deref for ShareStr {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Debug for ShareStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ShareStr({:?})", self.as_str())
    }
}

impl Display for ShareStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Hash for ShareStr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state)
    }
}

impl<T> PartialEq<T> for ShareStr
where
    str: PartialEq<T>,
{
    fn eq(&self, other: &T) -> bool {
        (**self).eq(other)
    }
}

impl PartialEq<Self> for ShareStr {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl Eq for ShareStr {}

impl Borrow<str> for ShareStr {
    fn borrow(&self) -> &str {
        self
    }
}
