use crate::ip::ip_range_bounds::IpRangeBounds;

struct IpRangeIncreases {
    pub range0: u8,
    pub range1: u8,
    pub range2: u8,
    pub range3: u8,
}

#[derive(Copy, Clone)]
pub struct IpRange {
    range0: IpRangeBounds,
    range1: IpRangeBounds,
    range2: IpRangeBounds,
    range3: IpRangeBounds,

    range0_size: u16,
    range1_size: u16,
    range2_size: u16,
    range3_size: u16,

    range0_size_normalized: u64,
    range1_size_normalized: u64,
    range2_size_normalized: u64,
    range3_size_normalized: u64,

    pub total_size: u64
}

impl IpRange {
    pub fn new(range0: IpRangeBounds, range1: IpRangeBounds, range2: IpRangeBounds, range3: IpRangeBounds) -> Self{
        // We add 1 to calculate the sizes since our loops are inclusive in both directions
        // position0..=range0.upper
        // As in; we need BOTH 0 and 255 when generating, meaning 256 options.
        let range0_size = ( range0.upper - range0.lower) as u16 + 1;
        let range1_size = ( range1.upper - range1.lower) as u16 + 1;
        let range2_size = ( range2.upper - range2.lower) as u16 + 1;
        let range3_size = ( range3.upper - range3.lower) as u16 + 1;

        let range3_size_normalized = 1;
        let range2_size_normalized = range3_size as u64 * range3_size_normalized;
        let range1_size_normalized = range2_size as u64 * range2_size_normalized;
        let range0_size_normalized = range1_size as u64 * range1_size_normalized;

        IpRange{
            total_size: range0_size as u64 * range1_size as u64 * range2_size as u64 * range3_size as u64,
            range0_size_normalized,
            range1_size_normalized,
            range2_size_normalized,
            range3_size_normalized,
            range0_size,
            range1_size,
            range2_size,
            range3_size,
            range0,
            range1,
            range2,
            range3
        }
    }

    /**
        Calculates how much each range should be increased, to add n batches of a given size to them
        Does not validate if these increases are valid u8, this should be done before calling
        As in, don't call this function with a number that trigger an increase larger than u8 (if you do, something else is wrong)
    **/
    fn calculate_position_increases(&self, nth_batch: usize, batch_size: usize) -> IpRangeIncreases {
        let mut remainder = (batch_size*nth_batch) as u64;
        let range0 = (remainder as f64/self.range0_size_normalized as f64).floor() as u64;
        if range0 != 0 {
            remainder = remainder - range0*self.range0_size_normalized;
        }

        let range1 = (remainder as f64/self.range1_size_normalized as f64).floor() as u64;
        if range1 != 0 {
            remainder = remainder - range1*self.range1_size_normalized;
        }

        let range2 = (remainder as f64/self.range2_size_normalized as f64).floor() as u64;
        if range2 != 0 {
            remainder = remainder - range2*self.range2_size_normalized;
        }

        IpRangeIncreases {
            range0: range0 as u8,
            range1: range1 as u8,
            range2: range2 as u8,
            range3: remainder as u8,
        }
    }

    /**
        Generates a batch of ips based on a batch size and which batch it is.
    **/
    pub fn generate_nth_batch(&self, nth_batch: usize, batch_size: usize) -> Vec<String> {
        let mut batch: Vec<String> = Vec::new();

        let position_increases = self.calculate_position_increases(nth_batch, batch_size);

        let mut position0 = self.range0.lower+ position_increases.range0;
        let mut position1 = self.range1.lower+ position_increases.range1;
        let mut position2 = self.range2.lower+ position_increases.range2;
        let mut position3 = self.range3.lower+ position_increases.range3;

        // By using simply for loops on the positions and upper bounds, we don't need any if statements involved in incrementing the positional arguments
        for r0 in position0..=self.range0.upper {
            position0 = r0;
            for r1 in position1..=self.range1.upper {
                position1 = r1;
                for r2 in position2..=self.range2.upper {
                    position2 = r2;
                    for r3 in position3..=self.range3.upper {
                        position3 = r3;
                        batch.push(String::from(format!("{}.{}.{}.{}",r0,r1,r2,r3)));

                        if batch.len() == batch_size {
                            return batch;
                        }
                    }

                    // When we have just finished a loop at the bottom level, we check the various positions and resets them if necessary
                    if self.range2.upper == position2 && self.range1.upper == position1 && self.range0.upper == position0 {
                        return batch;
                    } else {
                        position3 = self.range3.lower;
                    }
                }

                position2 = self.range2.lower;
            }

            position1 = self.range1.lower
        }

        return batch;
    }
}