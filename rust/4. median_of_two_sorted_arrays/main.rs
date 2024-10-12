impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut combined: Vec<i32> = Vec::new();

        let mut i_1: usize = 0;
        let mut i_2: usize = 0;
    
        let median_idx: usize = ((nums1.len() + nums2.len()) as f32 / 2.0).ceil() as usize;

        let mut nums1_end = false;
        let mut nums2_end = false;

        if nums1.len() == 0 {
            nums1_end = true;
        }
        if nums2.len() == 0 {
            nums2_end = true;
        }

        while combined.len() < median_idx + 1 {
            if !nums1_end && (nums2_end || nums1[i_1] < nums2[i_2]) {
                combined.push(nums1[i_1]);
                if i_1 == nums1.len() - 1 {
                    nums1_end = true;
                } else {
                    i_1 += 1;
                }
            } else if !nums2_end && (nums1_end || nums1[i_1] >= nums2[i_2]) {
                combined.push(nums2[i_2]);
                if i_2 == nums2.len() - 1 {
                    nums2_end = true;
                } else {
                    i_2 += 1;
                }
            } else {
                break;
            }
        };
        if combined.len() == 1 {
            combined[0] as f64
        }
        else if (nums1.len() + nums2.len()) % 2 == 1 {
            combined[combined.len() - 2] as f64
        } else {
            (combined[combined.len() - 2] + combined[combined.len() - 1]) as f64 / 2.0
        }
    }
}