use super::*;

pub struct Selectors {
    code: SegmentSelector,
    tss: SegmentSelector
}

impl Selectors {
    pub fn new(code: SegmentSelector, tss: SegmentSelector) -> Selectors {
        Selectors { code, tss }
    }

    pub fn tss(&self) -> SegmentSelector { self.tss }
    pub fn code(&self) -> SegmentSelector { self.code }
}
