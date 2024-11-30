pub(super) struct RailScanner {
    pub(super) sum: u16,
    len: u16,
    pos: u16,
    start: Cursor,
    end: Cursor,
}

struct Cursor {
    index: u16,
    segment: BitSegment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct BitSegment {
    pub(super) start: u16,
    pub(super) end: u16,
}

impl BitSegment {
    pub(super) fn new(start: u16, end: u16) -> Self {
        Self { start, end: end - 1 }
    }

    fn values(&self) -> (usize, usize) {
        (self.start as usize, self.end as usize)
    }
}

#[derive(Debug, Clone)]
pub(super) struct Rail {
    pub(super) bit_segments: Vec<BitSegment>,
}

impl Rail {
    pub(super) fn empty() -> Self {
        Self { bit_segments: vec![] }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.bit_segments.is_empty()
    }

    pub(super) fn start(&self, width: u16) -> RailScanner {
        if self.bit_segments.is_empty() {
            return RailScanner {
                sum: 0,
                len: 0,
                pos: 0,
                start: Cursor { index: 0, segment: BitSegment { start: 0, end: 0 } },
                end: Cursor { index: 0, segment: BitSegment { start: 0, end: 0 } },
            };
        }

        let mut s = 0;
        let mut i = 0;
        let w = width;
        while i < self.bit_segments.len() {
            let seg = &self.bit_segments[i];
            if w <= seg.end {
                if w >= seg.start {
                    s += w - seg.start;
                }
                break;
            }

            s += seg.end - seg.start + 1;

            i += 1;
        }

        if i >= self.bit_segments.len() {
            i = self.bit_segments.len() - 1;
        }

        RailScanner {
            sum: s,
            len: width,
            pos: 0,
            start: Cursor { index: 0, segment: self.bit_segments[0] },
            end: Cursor { index: i as u16, segment: self.bit_segments[i] },
        }
    }

    pub(super) fn append(&mut self, segment: BitSegment) {
        if let Some(last) = self.bit_segments.last_mut() {
            if last.end + 1 >= segment.start {
                last.end = segment.end;
            } else {
                self.bit_segments.push(segment);
            }
        } else {
            self.bit_segments.push(segment);
        }
    }

    pub(super) fn merge(&mut self, rail: &Rail) {
        if rail.is_empty() {
            return;
        }

        if self.is_empty() {
            self.bit_segments.extend_from_slice(&rail.bit_segments);
            return;
        }

        let mut result = Vec::with_capacity(self.bit_segments.len());

        let mut i = 0;
        let mut j = 0;
        let mut end = 0;
        let mut start = 0;
        let mut merge = false;

        let (mut i_start, mut i_end) = self.bit_segments[0].values();
        let (mut j_start, mut j_end) = rail.bit_segments[0].values();

        while i < self.bit_segments.len() || j < rail.bit_segments.len() {
            if merge {
                if end + 1 >= j_start {
                    end = end.max(j_end);
                    j += 1;
                    (j_start, j_end) = rail.next_value(j);
                } else if end + 1 >= i_start {
                    end = end.max(i_end);
                    i += 1;
                    (i_start, i_end) = self.next_value(i);
                } else {
                    result.push(BitSegment { start: start as u16, end: end as u16 });
                    merge = false;
                }
            } else {
                if i_end + 1 <= j_start {
                    start = i_start;
                    end = i_end;
                    i += 1;
                    (i_start, i_end) = self.next_value(i);
                } else if j_end + 1 <= i_start {
                    start = j_start;
                    end = j_end;
                    j += 1;
                    (j_start, j_end) = rail.next_value(j);
                } else {
                    start = i_start.min(j_start);
                    end = i_end.max(j_end);
                }
                merge = true;
            }
        }

        if merge {
            result.push(BitSegment { start: start as u16, end: end as u16 });
        }

        self.bit_segments = result;
    }

    fn next_value(&self, index: usize) -> (usize, usize) {
        if index < self.bit_segments.len() {
            self.bit_segments[index].values()
        } else {
            (usize::MAX - 1, usize::MAX - 1)
        }
    }
}

impl RailScanner {
    pub(super) fn iterate(&mut self, rail: &Rail) -> usize {
        if rail.is_empty() {
            return 0
        }

        let old_pos = self.pos;
        let new_end = old_pos + self.len;

        let mut s = self.sum;

        if self.start.segment.start <= old_pos && old_pos <= self.start.segment.end {
            s -= 1;
        } else if old_pos > self.start.segment.end {
            self.start.index += 1;
            let si = self.start.index as usize;
            if si < rail.bit_segments.len() {
                self.start.segment = rail.bit_segments[si];
            } else {
                self.start.segment = BitSegment { start: u16::MAX, end: u16::MAX }
            }
        }

        if self.end.segment.start <= new_end && new_end <= self.end.segment.end {
            s += 1;
        } else if new_end > self.end.segment.end {
            self.end.index += 1;
            let si = self.end.index as usize;
            if si < rail.bit_segments.len() {
                self.end.segment = rail.bit_segments[si];
            } else {
                self.end.segment = BitSegment { start: u16::MAX, end: u16::MAX }
            }
        }

        self.pos = old_pos + 1;
        self.sum = s;

        s as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::solution::rail::{BitSegment, Rail};

    impl Rail {
        fn with_array(array: &[u8]) -> Self {
            let mut segments = Vec::new();
            let mut a0 = 0;
            let mut n = 0;
            let mut i = 0;
            for &ai in array.iter() {
                n += ai as u16;
                if a0 == 1 && ai == 0 {
                    segments.push(BitSegment { start: i - n, end: i - 1 });
                    n = 0;
                }
                a0 = ai;
                i += 1;
            }
            if n > 0 {
                segments.push(BitSegment { start: i - n, end: i - 1 });
            }

            Self { bit_segments: segments }
        }
    }

    fn sum(start: u16, width: u16, array: &[u8]) -> u16 {
        let mut i = start as usize;
        let end = (start + width) as usize;
        let mut s = 0;
        while i < end {
            s += array[i] as u16;
            i += 1
        }
        s
    }

    #[test]
    fn test_with_array_0() {
        assert_eq!(vec![
            BitSegment { start: 2, end: 2 }
        ], Rail::with_array(&[0, 0, 1]).bit_segments);
        assert_eq!(vec![
            BitSegment { start: 0, end: 0 },
            BitSegment { start: 2, end: 2 },
        ], Rail::with_array(&[1, 0, 1]).bit_segments);
        assert_eq!(vec![
            BitSegment { start: 1, end: 1 }
        ], Rail::with_array(&[0, 1, 0]).bit_segments);
        assert_eq!(vec![
            BitSegment { start: 2, end: 3 }
        ], Rail::with_array(&[0, 0, 1, 1, 0]).bit_segments);
        assert_eq!(vec![
            BitSegment { start: 0, end: 1 }
        ], Rail::with_array(&[1, 1, 0, 0, 0]).bit_segments);
        assert_eq!(vec![
            BitSegment { start: 0, end: 0 }
        ], Rail::with_array(&[1]).bit_segments);
        assert_eq!(vec![
            BitSegment { start: 0, end: 2 }
        ], Rail::with_array(&[1, 1, 1]).bit_segments);
        assert_eq!(true, Rail::with_array(&[0]).bit_segments.is_empty());
        assert_eq!(true, Rail::with_array(&[0, 0, 0]).bit_segments.is_empty());
    }

    #[test]
    fn test_sum_0() {
        test_sum(&[1, 1, 0, 0], 1);
    }

    #[test]
    fn test_sum_1() {
        test_sum(&[1, 1, 0, 1], 1);
    }

    #[test]
    fn test_sum_2() {
        test_sum(&[1, 0, 1, 1], 1);
    }

    #[test]
    fn test_sum_3() {
        test_sum(&[1, 0, 0, 0], 1);
    }

    #[test]
    fn test_sum_4() {
        test_sum(&[1, 0, 1, 1, 0, 1], 2);
    }

    #[test]
    fn test_sum_rand() {
        let mut w = 1;

        while w < 5 {
            for a in 1..=255u8 {
                let array = to_bit_array(a);
                test_sum(&array, w);
            }
            w += 1;
        }
    }

    fn test_sum(array: &[u8], w: u16) {
        let rail = Rail::with_array(array);
        let mut scanner = rail.start(w);

        let mut s0 = scanner.sum;
        let mut s1 = sum(0, w, array);
        assert_eq!(s0, s1);

        let last = array.len() - w as usize;

        for i in 1..last {
            scanner.iterate(&rail);
            s0 = scanner.sum;
            s1 = sum(i as u16, w, &array);
            assert_eq!(s0, s1);
        }
    }

    fn to_bit_array(value: u8) -> [u8; 8] {
        let mut arr = [0u8; 8];
        for j in 0..8 {
            arr[7 - j] = (value >> j) & 1;
        }
        arr
    }

    #[test]
    fn test_merge_0() {
        test_merge(&[1, 0, 0, 1], &[1, 0, 0, 1]);
    }

    #[test]
    fn test_merge_1() {
        test_merge(&[1, 0, 1, 0], &[0, 1, 0, 1]);
    }

    #[test]
    fn test_merge_2() {
        test_merge(&[1, 0, 0, 0], &[0, 0, 0, 1]);
    }

    #[test]
    fn test_merge_3() {
        test_merge(&[1, 1, 0, 0], &[0, 0, 1, 1]);
    }

    #[test]
    fn test_merge_4() {
        test_merge(&[0, 1, 1, 1], &[1, 1, 1, 0]);
    }

    #[test]
    fn test_merge_rand() {
        for a in 1..=255u8 {
            let arr_a = to_bit_array(a);
            for b in 1..=255u8 {
                let arr_b = to_bit_array(b);
                test_merge(&arr_a, &arr_b);
            }
        }
    }

    fn test_merge(array_0: &[u8], array_1: &[u8]) {
        let mut rail_0 = Rail::with_array(array_0);
        let rail_1 = Rail::with_array(array_1);

        rail_0.merge(&rail_1);

        let mut array = array_0.to_vec();
        for i in 0..array_1.len() {
            if i < array.len() {
                if array[i] == 0 && array_1[i] == 1 {
                    array[i] = 1;
                }
            } else {
                array.push(array_1[i]);
            }
        }

        let rail = Rail::with_array(&array);

        assert_eq!(rail_0.bit_segments, rail.bit_segments);
    }
}