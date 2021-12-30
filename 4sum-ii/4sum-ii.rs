impl Solution {
    pub fn four_sum_count(
        mut nums1: Vec<i32>,
        mut nums2: Vec<i32>,
        mut nums3: Vec<i32>,
        mut nums4: Vec<i32>,
    ) -> i32 {
        nums1.sort_unstable();
        nums2.sort_unstable();
        nums3.sort_unstable();
        nums4.sort_unstable();

        four_sum_count(nums1, nums2, nums3, nums4)
    }
}

fn four_sum_count(
        mut nums1: Vec<i32>,
        mut nums2: Vec<i32>,
        mut nums3: Vec<i32>,
        mut nums4: Vec<i32>,
    ) -> i32 {
    let n = nums1.len();

    let mut count = 0;

    let mut count1 = 0;
    let lower = nums2[0] + nums3[0] + nums4[0];
    let upper = nums2[n - 1] + nums3[n - 1] + nums4[n - 1];
    for i in 0..n {
        if i != 0 && nums1[i] == nums1[i - 1] {
            count += count1;
            continue;
        }
        count1 = 0;
        if lower > -nums1[i] || upper < -nums1[i] {
            continue;
        }
        let mut count2 = 0;
        let lower = lower - nums2[0];
        let upper = upper - nums2[n - 1];
        for j in 0..n {
            if j != 0 && nums2[j] == nums2[j - 1] {
                count1 += count2;
                continue;
            }
            count2 = 0;
            if lower > -nums1[i] - nums2[j] || upper < -nums1[i] - nums2[j] {
                continue;
            }
            let mut count3 = 0;
            let lower = nums4[0];
            let upper = nums4[n - 1];
            for k in 0..n {
                if k != 0 && nums3[k] == nums3[k - 1] {
                    count2 += count3;
                    continue;
                }
                count3 = 0;
                if lower > -nums1[i] - nums2[j] - nums3[k]
                    || upper < -nums1[i] - nums2[j] - nums3[k]
                {
                    continue;
                }
                for l in 0..n {
                    if nums1[i] + nums2[j] + nums3[k] + nums4[l] == 0 {
                        count3 += 1;
                    }
                }
                count2 += count3;
            }
            count1 += count2;
        }
        count += count1;
    }
    count
}
