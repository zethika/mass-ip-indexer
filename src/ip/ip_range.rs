use crate::ip::ip_range_bounds::IpRangeBounds;

pub struct IpRange {
    range0: IpRangeBounds,
    range1: IpRangeBounds,
    range2: IpRangeBounds,
    range3: IpRangeBounds,
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