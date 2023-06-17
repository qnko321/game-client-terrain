extern crate core;

use anyhow::anyhow;

#[derive(Clone, Debug)]
pub(crate) struct BufferManager {
    free_regions: Vec<BufferRegion>,
    used_regions: Vec<BufferRegion>,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub(crate) struct BufferRegion {
    offset: usize,
    size: usize,
}

impl BufferRegion {
    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn end(&self) -> usize {
        self.offset + self.size
    }
}

impl BufferManager {
    pub(crate) fn new() -> Self {
        Self {
            free_regions: vec![],
            used_regions: vec![],
        }
    }

    pub(crate) fn clear(&mut self) {
        self.free_regions = vec![];
        self.used_regions = vec![];
    }

    pub(crate) fn use_free_region(&mut self, size: usize) -> anyhow::Result<usize> {
        let mut best_fit: Option<usize> = None;
        let mut best_fit_size: Option<usize> = None;
        for i in 0..self.free_regions.len() {
            let region = *self.free_regions.get(i).unwrap();

            if region.size >= size {
                if region.size == size {
                    best_fit = Some(i);
                    best_fit_size = Some(region.size);
                    break;
                } else if best_fit.is_none() {
                    best_fit = Some(i);
                    best_fit_size = Some(region.size);
                } else if region.size as isize - size as isize > 0 && (region.size as isize - size as isize) < best_fit_size.unwrap() as isize - size as isize {
                    best_fit = Some(i);
                    best_fit_size = Some(region.size);
                }
            }
        }

        if best_fit.is_none() {
            return Err(anyhow!("Not enough free memory on buffer!"));
        }

        let region = *self.free_regions.get(best_fit.unwrap()).unwrap();

        if region.size > size {
            self.free_regions.push(BufferRegion {
                offset: region.offset + size,
                size: region.size - size,
            });
        }

        self.used_regions.push(BufferRegion {
            offset: region.offset,
            size,
        });

        self.free_regions.remove(best_fit.unwrap());

        Ok(region.offset)
    }

    pub(crate) fn add_free_region(&mut self, offset: usize, size: usize) {
        self.free_regions.push(BufferRegion {
            offset,
            size,
        });
    }

    pub(crate) fn add_used_region(&mut self, offset: usize, size: usize) {
        self.used_regions.push(BufferRegion {
            offset,
            size,
        });
    }

    pub(crate) fn free_used_region(&mut self, index: usize) {
        let used_region = *self.used_regions.get(index).unwrap();
        self.used_regions.push(BufferRegion {
            offset: used_region.offset,
            size: used_region.size,
        });
        self.used_regions.remove(index);
    }

    fn free_segment(&mut self, region: BufferRegion) {
        let mut done = true;

        for i in 0..self.used_regions.len() {
            let used_region = match self.used_regions.get(i) {
                None => break,
                Some(value) => *value
            };

            if region.end() > used_region.offset && used_region.end() - 1 > region.offset {
                done = false;
                if region.offset > used_region.offset {
                    self.used_regions.push(BufferRegion {
                        offset: used_region.offset,
                        size: region.offset - used_region.offset,
                    });
                }
                if region.end() < used_region.end() {
                    self.used_regions.push(BufferRegion {
                        offset: region.end(),
                        size: used_region.end() - region.end(),
                    });
                }
                self.used_regions.remove(i);
            }


        }

        if !done {
            self.free_segment(region);
        } else {
            self.free_regions.push(region);
        }
    }



    pub fn merge_contiguous_free_regions(&mut self) {
        self.free_regions.sort_by(|a, b| a.offset.cmp(&b.offset));

        let mut i = 0;
        while i < self.free_regions.len() - 1 {
            let current_region = self.free_regions[i];
            let next_region = self.free_regions[i + 1];

            if current_region.end() == next_region.offset {
                self.free_regions[i].size += next_region.size;

                self.free_regions.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }

    pub fn merge_contiguous_used_regions(&mut self) {
        self.used_regions.sort_by(|a, b| a.offset.cmp(&b.offset));

        let mut i = 0;
        while i < self.used_regions.len() - 1 {
            let current_region = self.used_regions[i];
            let next_region = self.used_regions[i + 1];

            if current_region.end() == next_region.offset {
                self.used_regions[i].size += next_region.size;

                self.used_regions.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_use_free_region_same_size() {
            let mut buffer_manager = BufferManager::new();
            buffer_manager.add_free_region(0, 20);
            buffer_manager.add_free_region(20, 30);
            buffer_manager.add_free_region(50, 40);

            // Attempt to use a region of size 40.
            let result = buffer_manager.use_free_region(40);
            assert!(result.is_ok());
            let offset = result.unwrap();

            // Since the first two regions are smaller, the last region is used.
            assert_eq!(offset, 50);

            // There should be 2 free regions left.
            assert_eq!(buffer_manager.free_regions.len(), 2);
            assert_eq!(buffer_manager.used_regions.len(), 1);
        }

        #[test]
        fn test_use_free_region_success() {
            let mut buffer_manager = BufferManager::new();
            buffer_manager.add_free_region(0, 10);
            buffer_manager.add_free_region(10, 20);
            buffer_manager.add_free_region(30, 10);

            // Attempt to use a region of size 15.
            let result = buffer_manager.use_free_region(15);
            assert!(result.is_ok());
            let offset = result.unwrap();
            assert_eq!(offset, 10);

            // Ensure that the remaining free region of size 5 starts at offset 25.
            let remaining_region = buffer_manager.free_regions.iter().find(|&r| r.offset == 25).unwrap();
            assert_eq!(remaining_region.size, 5);
        }

        #[test]
        fn test_use_free_region_not_enough_memory() {
            let mut buffer_manager = BufferManager::new();
            buffer_manager.add_free_region(0, 10);
            buffer_manager.add_free_region(10, 10);
            buffer_manager.add_free_region(20, 10);

            // Attempt to use a region of size 30.
            let result = buffer_manager.use_free_region(30);
            println!("{:?}", result);
            assert!(result.is_err());

            // Ensure that the free regions have not changed.
            assert_eq!(buffer_manager.free_regions.len(), 3);
            assert_eq!(buffer_manager.used_regions.len(), 0);
        }
    }

    #[test]
    fn test_merge_contiguous_free_regions() {
        let mut buffer_manager = BufferManager::new();
        for i in 0..=1000 {
            if i != 500 {
                buffer_manager.add_free_region(i * 10, 10);
            } else {
                buffer_manager.add_free_region(i * 10, 9);
            }
        }
        buffer_manager.merge_contiguous_free_regions();
        assert_eq!(buffer_manager.free_regions, vec![BufferRegion { offset: 0, size: 5009}, BufferRegion {offset: 5010, size: 5000}]);
        assert_eq!(buffer_manager.used_regions, vec![]);
    }

    #[test]
    fn test_merge_contiguous_used_regions() {
        let mut buffer_manager = BufferManager::new();
        for i in 0..=1000 {
            if i != 500 {
                buffer_manager.add_used_region(i * 10, 10);
            } else {
                buffer_manager.add_used_region(i * 10, 9);
            }
        }
        buffer_manager.merge_contiguous_used_regions();
        assert_eq!(buffer_manager.used_regions, vec![BufferRegion { offset: 0, size: 5009}, BufferRegion {offset: 5010, size: 5000}]);
        assert_eq!(buffer_manager.free_regions, vec![]);
    }

    #[test]
    fn test_new_buffer_manager() {
        let buffer_manager = BufferManager::new();
        assert!(buffer_manager.free_regions.is_empty());
        assert!(buffer_manager.used_regions.is_empty());
    }

    #[test]
    fn test_add_free_region() {
        let mut buffer_manager = BufferManager::new();
        buffer_manager.add_free_region(5, 10);

        assert_eq!(buffer_manager.free_regions.len(), 1);
        assert_eq!(buffer_manager.free_regions[0], BufferRegion { offset: 5, size: 10 });
    }

    #[test]
    fn test_add_used_region() {
        let mut buffer_manager = BufferManager::new();
        buffer_manager.add_used_region(10, 20);

        assert_eq!(buffer_manager.used_regions.len(), 1);
        assert_eq!(buffer_manager.used_regions[0], BufferRegion { offset: 10, size: 20 });
    }

    #[test]
    fn test_use_free_region() {
        let mut buffer_manager = BufferManager::new();
        buffer_manager.add_free_region(0, 10);
        let res = buffer_manager.use_free_region(5);

        assert!(res.is_ok());
        assert_eq!(buffer_manager.free_regions.len(), 1);
        assert_eq!(buffer_manager.used_regions.len(), 1);
        assert_eq!(buffer_manager.free_regions[0], BufferRegion { offset: 5, size: 5 });
        assert_eq!(buffer_manager.used_regions[0], BufferRegion { offset: 0, size: 5 });
    }

    #[test]
    fn test_free_segment_no_overlap() {
        let mut buffer_manager = BufferManager::new();
        buffer_manager.add_used_region(10, 20);
        buffer_manager.free_segment(BufferRegion { offset: 30, size: 10 });

        assert_eq!(buffer_manager.free_regions.len(), 1);
        assert_eq!(buffer_manager.free_regions[0], BufferRegion { offset: 30, size: 10 });
        assert_eq!(buffer_manager.used_regions.len(), 1);
        assert_eq!(buffer_manager.used_regions[0], BufferRegion { offset: 10, size: 20 });
    }

    #[test]
    fn test_free_segment_overlap() {
        let mut buffer_manager = BufferManager::new();
        buffer_manager.add_used_region(10, 20);
        buffer_manager.free_segment(BufferRegion { offset: 15, size: 10 });

        assert_eq!(buffer_manager.free_regions.len(), 1);
        assert_eq!(buffer_manager.free_regions[0], BufferRegion { offset: 15, size: 10 });
        assert_eq!(buffer_manager.used_regions.len(), 2);
        assert_eq!(buffer_manager.used_regions[0], BufferRegion { offset: 10, size: 5 });
        assert_eq!(buffer_manager.used_regions[1], BufferRegion { offset: 25, size: 5 });
    }


    #[test]
    fn test_free_segment_exact_match() {
        let mut buffer_manager = BufferManager::new();
        buffer_manager.add_used_region(10, 20);
        buffer_manager.free_segment(BufferRegion { offset: 10, size: 20 });

        assert_eq!(buffer_manager.free_regions.len(), 1);
        assert_eq!(buffer_manager.free_regions[0], BufferRegion { offset: 10, size: 20 });
        assert_eq!(buffer_manager.used_regions.len(), 0);
    }

    #[test]
    fn test_free_segment_complex() {
        let mut buffer_manager = BufferManager::new();
        buffer_manager.add_used_region(0, 10);
        buffer_manager.add_used_region(20, 10);
        buffer_manager.add_used_region(40, 10);

        buffer_manager.free_segment(BufferRegion { offset: 5, size: 30 });


        assert_eq!(buffer_manager.used_regions.len(), 2);
        assert_eq!(buffer_manager.used_regions.contains(&BufferRegion { offset: 0, size: 5 }), true);
        assert_eq!(buffer_manager.used_regions.contains(&BufferRegion { offset: 40, size: 10 }), true);

        assert_eq!(buffer_manager.free_regions.len(), 1);
        assert_eq!(buffer_manager.free_regions[0], BufferRegion { offset: 5, size: 30 });
    }
}