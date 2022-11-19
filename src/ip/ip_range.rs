use crate::ip::ip_range_bounds::IpRangeBounds;

pub struct IpRange {
    range0: IpRangeBounds,
    range1: IpRangeBounds,
    range2: IpRangeBounds,
    range3: IpRangeBounds,

    position0: u8,
    position1: u8,
    position2: u8,
    position3: u8
}

impl IpRange {
    pub fn new(range0: IpRangeBounds, range1: IpRangeBounds, range2: IpRangeBounds, range3: IpRangeBounds) -> Self{
        IpRange{
            position0: range0.lower,
            position1: range1.lower,
            position2: range2.lower,
            position3: range3.lower,
            range0,
            range1,
            range2,
            range3
        }
    }

    pub fn has_more_batches(&self) -> bool {
        self.position3 != self.range3.upper || self.position2 != self.range2.upper || self.position1 != self.range1.upper || self.position0 != self.range0.upper
    }

    pub fn generate_next_batch(&mut self, size: usize) -> Vec<String> {
        let mut batch: Vec<String> = Vec::new();

/*
        // Other way of generating, generates at around 5900 ips pr milli
        for n in 0..size {
            if self.has_more_batches() == false {
                return batch;
            }

            batch.push(String::from(format!("{}.{}.{}.{}",self.position0,self.position1,self.position2,self.position3)));

            self.increment_position();
        }

        return batch;*/

        // By using simply for loops on the positions and upper bounds, we don't need any if statements involved in incrementing the positional arguments
        // Faster way - generates at around 6200  ips pr. milli
        for r0 in self.position0..=self.range0.upper
        {
            self.position0 = r0;
            for r1 in self.position1..=self.range1.upper {
                self.position1 = r1;
                for r2 in self.position2..=self.range2.upper {
                    self.position2 = r2;
                    for r3 in self.position3..=self.range3.upper {
                        self.position3 = r3;
                        batch.push(String::from(format!("{}.{}.{}.{}",r0,r1,r2,r3)));

                        if batch.len() == size {
                            return batch;
                        }
                    }

                    // When we have just finished a loop at the bottom level, we check the various positions and resets them if necessary
                    if self.has_more_batches() == false {
                        return batch;
                    } else {
                        self.position3 = self.range3.lower;
                    }
                }

                self.position2 = self.range2.lower;
            }

            self.position1 = self.range1.lower
        }

        return batch;
    }

    // Increments the current positional index
    // Return false if there is no remaining ranges to increment to
    fn increment_position(&mut self) -> bool{
        // We only need to check the upper ranges position, if the last position handled was the end of the lower range bounds.
        if self.position3 != self.range3.upper {
            self.position3 = self.position3 + 1;
        } else if self.position2 != self.range2.upper {
            self.position3 = self.range3.lower;
            self.position2 = self.position2 + 1;
        } else if self.position1 != self.range1.upper {
            self.position3 = self.range3.lower;
            self.position2 = self.range2.lower;
            self.position1 = self.position1 + 1;
        } else if self.position0 != self.range0.upper {
            self.position3 = self.range3.lower;
            self.position2 = self.range2.lower;
            self.position1 = self.range1.lower;
            self.position0 = self.position0 + 1;
        } else {
            return false;
        }

        return true;
    }
}