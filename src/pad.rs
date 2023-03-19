use std::ops::{Bound, RangeBounds};

pub struct Padding<D> {
    data: D,
}

impl<D: Copy> Padding<D> {
    pub fn new(pad: D) -> Self {
        Padding { data: pad }
    }
    pub fn data(&self) -> D {
        self.data
    }
    pub fn get<T: RangeBounds<usize>>(self, range: T) -> PadBounded<D, T> {
        PadBounded {
            range,
            data: self.data,
        }
    }
    pub fn load<L: Iterator<Item = D>>(self, iter: L) -> PadLoaded<D, L> {
        PadLoaded {
            load: iter,
            data: self.data,
        }
    }
}
pub struct PadBounded<D: Copy, R: RangeBounds<usize>> {
    range: R,
    data: D,
}
impl<D: Copy, R: RangeBounds<usize>> PadBounded<D, R> {
    pub fn load<I: Iterator<Item = D>>(self, iterator: I) -> PadBoundedLoad<D, I, R> {
        PadBoundedLoad {
            load: iterator,
            data: self.data,
            range: self.range,
        }
    }
}
impl<D: Copy> Iterator for Padding<D> {
    type Item = D;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.data)
    }
}

pub struct PadBoundedIter<D: Copy, R: RangeBounds<usize>> {
    data: D,
    range: R,
    current_idx: usize,
}

impl<D: Copy, L: RangeBounds<usize>> IntoIterator for PadBounded<D, L> {
    type Item = D;

    type IntoIter = PadBoundedIter<D, L>;

    fn into_iter(self) -> Self::IntoIter {
        let start_bound = self.range.start_bound();
        let current_idx = match start_bound {
            Bound::Unbounded => 0,
            Bound::Excluded(&x) => x + 1,
            Bound::Included(&x) => x,
        };
        PadBoundedIter {
            current_idx,
            data: self.data,
            range: self.range,
        }
    }
}
impl<D: Copy, L: RangeBounds<usize>> Iterator for PadBoundedIter<D, L> {
    type Item = D;

    fn next(&mut self) -> Option<Self::Item> {
        if self.range.contains(&self.current_idx) {
            self.current_idx += 1;
            Some(self.data)
        } else {
            None
        }
    }
}

pub struct PadLoaded<D: Copy, L: Iterator<Item = D>> {
    load: L,
    data: D,
}

impl<D: Copy, L: Iterator<Item = D>> Iterator for PadLoaded<D, L> {
    type Item = D;

    fn next(&mut self) -> Option<Self::Item> {
        let current_value = self.load.next();
        if current_value.is_none() {
            Some(self.data)
        } else {
            current_value
        }
    }
}

pub struct PadBoundedLoad<D: Copy, L: Iterator<Item = D>, R: RangeBounds<usize>> {
    data: D,
    range: R,
    load: L,
}
impl<D, L, R> IntoIterator for PadBoundedLoad<D, L, R>
where
    D: Copy,
    L: Iterator<Item = D>,
    R: RangeBounds<usize>,
{
    type Item = D;

    type IntoIter = PadBoundedLoadIter<D, L, R>;

    fn into_iter(self) -> Self::IntoIter {
        let start_bound = self.range.start_bound();
        let current_idx = match start_bound {
            Bound::Unbounded => 0,
            Bound::Excluded(&x) => x + 1,
            Bound::Included(&x) => x,
        };
        PadBoundedLoadIter {
            current_idx,
            data: self.data,
            load: self.load,
            range: self.range,
        }
    }
}
pub struct PadBoundedLoadIter<D: Copy, L: Iterator<Item = D>, R: RangeBounds<usize>> {
    data: D,
    load: L,
    range: R,
    current_idx: usize,
}

impl<D, L, R> Iterator for PadBoundedLoadIter<D, L, R>
where
    D: Copy,
    L: Iterator<Item = D>,
    R: RangeBounds<usize>,
{
    type Item = D;

    fn next(&mut self) -> Option<Self::Item> {
        let load = &mut self.load;
        let current_value = load.next();
        if self.range.contains(&self.current_idx) == false {
            None
        } else if current_value.is_some() {
            self.current_idx += 1;
            current_value
        } else {
            self.current_idx += 1;
            Some(self.data)
        }
    }
}

#[cfg(test)]
mod test {
    use super::Padding;

    #[test]
    fn test_load() {
        let item1: [u8; 4] = [0, 1, 2, 3];
        // Padding 自身是一个 0 .. infinite 的迭代器
        // get() 函数将其切片，只取某一范围的元素，如 0 到 9（包含）
        // load() 用于将其他迭代器装载到 pad 的指定范围，这样迭代超出范围后，自动使用 pad::new(value) 中的 value 进行填充。
        // Note: 由于 pad 的无限性和同一性，get(0..=9) 和 get(10..=19) 并没有什么区别。但是
        // 在 0..infinite 的不同片段中进行填充时，所表达的含义并不相同。
        // ___________________________________________________
        // pad0 pad1 pad2 pad3 ... pad100 pad101 pad102 pad103
        // a1   a2   padded...     b1     b2     b3     padded
        // ___________________________________________________
        // get(0..=3).load(a)      get(100..=103).load(b)
        // ___________________________________________________
        let padding = Padding::new(&0xff);
        let iterator = item1.iter();
        let pad_item1_with_0xff = padding.get(0..=9).load(iterator);
        for value in pad_item1_with_0xff {
            println!("{value}");
        }
    }
}
