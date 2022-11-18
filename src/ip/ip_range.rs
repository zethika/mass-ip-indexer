use crate::ip::ip_range_bounds::IpRangeBounds;

#[derive(Debug)]
pub struct IpRange {
    pub range0: IpRangeBounds,
    pub range1: IpRangeBounds,
    pub range2: IpRangeBounds,
    pub range3: IpRangeBounds,
}

impl IpRange {
    pub fn new(range0: IpRangeBounds, range1: IpRangeBounds, range2: IpRangeBounds, range3: IpRangeBounds) -> Self{
        IpRange{
            range0,
            range1,
            range2,
            range3
        }
    }
}