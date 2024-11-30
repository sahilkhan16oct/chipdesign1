use foundation::i_overlay::i_float::bit_pack::BitPackVec;
use foundation::i_overlay::i_float::fix_vec::FixVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct ErrorEdge {
    // start
    pub(super) a: FixVec,
    // end
    pub(super) b: FixVec,
}

impl ErrorEdge {
    pub fn new(a: FixVec, b: FixVec) -> Self {
        if a.bit_pack() <= b.bit_pack() {
            Self { a, b }
        } else {
            Self { a: b, b: a }
        }
    }
}