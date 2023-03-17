use std::ops::{Bound, RangeBounds};

#[derive(Clone, Copy)]
pub struct Padding<D: Copy> {
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
        PadBounded { range, pad: self }
    }
}
pub struct PadBounded<D: Copy, R: RangeBounds<usize>> {
    range: R,
    pad: Padding<D>,
}
impl<D: Copy, R: RangeBounds<usize>> PadBounded<D, R> {
    pub fn load<I: Iterator<Item = D>>(self, iterator: I) -> PadBoundedLoad<D, I, R> {
        PadBoundedLoad {
            pad: self,
            load: iterator,
        }
    }
}
impl<D: Copy> Iterator for Padding<D> {
    type Item = D;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.data)
    }
}

pub struct PadBoundedIter<D: Copy, T: RangeBounds<usize>> {
    bounded_pad: PadBounded<D, T>,
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
            bounded_pad: self,
            current_idx,
        }
    }
}
impl<D: Copy, L: RangeBounds<usize>> Iterator for PadBoundedIter<D, L> {
    type Item = D;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bounded_pad.range.contains(&self.current_idx) {
            self.current_idx += 1;
            Some(self.bounded_pad.pad.data)
        } else {
            None
        }
    }
}

pub struct PadLoaded<D: Copy, L: Iterator<Item = D>> {
    pad: Padding<D>,
    load: L,
}

impl<D: Copy, L: Iterator<Item = D>> Iterator for PadLoaded<D, L> {
    type Item = D;

    fn next(&mut self) -> Option<Self::Item> {
        let current_value = self.load.next();
        if current_value.is_none() {
            Some(self.pad.data)
        } else {
            current_value
        }
    }
}

pub struct PadBoundedLoad<D: Copy, L: Iterator<Item = D>, R: RangeBounds<usize>> {
    pad: PadBounded<D, R>,
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
        let start_bound = self.pad.range.start_bound();
        let current_idx = match start_bound {
            Bound::Unbounded => 0,
            Bound::Excluded(&x) => x + 1,
            Bound::Included(&x) => x,
        };
        PadBoundedLoadIter {
            loaded_bounded_pad: self,
            current_idx,
        }
    }
}
pub struct PadBoundedLoadIter<D: Copy, L: Iterator<Item = D>, R: RangeBounds<usize>> {
    loaded_bounded_pad: PadBoundedLoad<D, L, R>,
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
        let load = &mut self.loaded_bounded_pad.load;
        let current_value = load.next();
        if current_value.is_some() {
            self.current_idx += 1;
            current_value
        } else if self
            .loaded_bounded_pad
            .pad
            .range
            .contains(&self.current_idx)
        {
            self.current_idx += 1;
            Some(self.loaded_bounded_pad.pad.pad.data)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::Padding;

    #[test]
    fn test_load() {
        let item: [u8; 4] = [0, 1, 2, 3];
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
        let pad_item1_with_0xff = padding.get(0..=9).load(item.iter());
        let pad_item2_with_0x55 = padding.get(20..=29).load(item.iter());
        for value in pad_item1_with_0xff
            .into_iter()
            .chain(pad_item2_with_0x55.into_iter())
        {
            println!("{value}");
        }
    }
}
