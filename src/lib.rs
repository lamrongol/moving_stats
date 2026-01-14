use std::collections::VecDeque;

#[derive(Debug)]
/// Fixed size queue, which calculate moving median when you add new value.
/// sampling size must be odd number because this can be used for all Ord(and Clone) types
/// computational complexity: O(n) for calculating median, O(nlog(n)) only for first calculation because it simply sorts and finds median.
pub struct MovingMedian<T: Ord+Clone> {
    //must be odd for all Ord type(like string)
    odd_sampling_size: usize,
    last_put_val: Option<T>,
    last_ejected_val: Option<T>,

    queue: VecDeque<T>,
    is_full: bool,
    median: Option<T>,

    //stats
    // min: Option<T>,
    // max: Option<T>,
    // average: T, //TODO for numeric type
}
//TODO MovingStats with average, min and max, too; for numeric which allow even integer sampling size

impl<T: Ord+Clone> MovingMedian<T> {
    pub fn add(&mut self, val: T) {
        self.queue.push_back(val.to_owned());
        self.last_put_val = Some(val.to_owned());

        if !self.is_full {
            if self.queue.len() == self.odd_sampling_size {
                self.is_full = true;

                let mut copy = self.queue.clone();
                copy .make_contiguous().sort();
                self.median = Some(copy[self.odd_sampling_size / 2].to_owned());
            }
        } else {
            self.last_ejected_val = Some(self.queue.pop_front().unwrap());

            if ((self.last_ejected_val < self.median) && (self.last_put_val < self.median))
                || ((self.last_ejected_val == self.median) && (self.last_put_val == self.median))
                || ((self.last_ejected_val > self.median) && (self.last_put_val > self.median))
            {
                //do nothing if last_ejected_val and last_put_val are larger, smaller, equals to previous median
            } else if self.last_put_val > self.median {
                let mut higher_count = 0;
                let mut min_over_median = None;
                for v in self.queue.iter() {
                    if *v > self.median.to_owned().unwrap() {
                        higher_count += 1;
                        if min_over_median.is_none() || *v <= min_over_median.to_owned().unwrap() {
                            min_over_median = Some(v.to_owned());
                        }
                    }
                }
                if higher_count > (self.odd_sampling_size / 2) {
                    self.median = min_over_median;
                }
            } else {
                let mut lower_count = 0;
                let mut max_under_median = None;
                for v in self.queue.iter() {
                    if *v < self.median.to_owned().unwrap() {
                        lower_count += 1;
                        if max_under_median.is_none() || *v >= max_under_median.to_owned().unwrap() {
                            max_under_median = Some(v.to_owned());
                        }
                    }
                }
                if lower_count > (self.odd_sampling_size / 2) {
                    self.median = max_under_median;
                }
            }
        }
    }

    ///idx allow minus integer, minus means last index
    pub fn get(&self, idx: isize) -> Result<T, &'static str> {
        if idx.abs()>= self.queue.len() as isize {
            Err("Index out of bounds")
        }else{
            if idx>=0{
                Ok(self.queue[idx as usize].to_owned())
            }else{
                Ok(self.queue[self.queue.len()-(idx.abs() as usize)].to_owned())
            }
        }
    }

    pub fn odd_sampling_size(&self) -> usize {self.odd_sampling_size}
    pub fn is_full(&self) -> bool {self.is_full}

    ///None if `is_full()==false`(added values count is smaller than `odd_sampling_size`)  
    pub fn median(&self) -> Option<T> {self.median.to_owned()}
}

pub fn new<T:Ord+Clone>(odd_sampling_size: usize) -> Result<MovingMedian<T>,  &'static str> {
    if odd_sampling_size % 2 == 1 {//this also means odd_sampling_siz  >0
        Ok(MovingMedian {
            odd_sampling_size,
            is_full: false,
            last_put_val: None,
            last_ejected_val: None,
            queue: VecDeque::with_capacity(odd_sampling_size),
            median: None,
        })
    }else{
        Err("sampling size must be odd because this is used for all Ord type(e.g. str), which can't be divided by 2")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(new::<i32>(6).is_err());
        assert!(new::<i32>(0).is_err());
        let mut queue = new::<i32>(3).unwrap();
        assert_eq!(queue.odd_sampling_size(), 3);
        queue.add(3);
        queue.add(7);
        assert_eq!(queue.is_full(), false);
        assert_eq!(queue.median(), None);
        queue.add(6);
        assert_eq!(queue.is_full(), true);
        assert_eq!(queue.median(), Some(6));
        queue.add(9);
        assert_eq!(queue.median(), Some(7));
        queue.add(1);
        assert_eq!(queue.median(), Some(6));
        assert_eq!(queue.get(0), Ok(6));
        assert_eq!(queue.get(2), Ok(1));
        assert_eq!(queue.get(3).is_err(), true);
        assert_eq!(queue.get(-3).is_err(), true);
        assert_eq!(queue.get(-1), Ok(1));
        assert_eq!(queue.get(-2), Ok(9));


        let mut str_queue = new::<String>(3).unwrap();
        str_queue.add(String::from("abc"));
        str_queue.add(String::from("def"));
        str_queue.add(String::from("adc"));
        assert_eq!(str_queue.is_full(), true);
        assert_eq!(str_queue.median(), Some(String::from("adc")));
        str_queue.add(String::from("bbe"));
        assert_eq!(str_queue.median(), Some(String::from("bbe")));

    }
}
