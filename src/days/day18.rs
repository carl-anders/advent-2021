use itertools::Itertools;
use smallvec::SmallVec;

use super::day::Day;

#[derive(Clone, Copy, Debug)]
pub struct SN {
    val: i32,
    depth: usize,
}
#[allow(clippy::use_self)]
impl SN {
    #[allow(dead_code)]
    fn print(nums: &[SN]) {
        let mut depth = 0;
        for num in nums {
            while num.depth > depth {
                depth += 1;
                print!("[");
            }
            while num.depth < depth {
                depth -= 1;
                print!("]");
            }
            print!(" {} ", num.val);
        }
        while 0 < depth {
            depth -= 1;
            print!("]");
        }
        println!();
    }
    fn add(nums: &mut SmallVec<[SN; 32]>, mut to_add: SmallVec<[SN; 32]>) {
        nums.append(&mut to_add);
        nums.iter_mut().for_each(|sn| sn.depth += 1);
        while Self::explode(nums) || Self::split(nums) {}
    }
    fn explode(nums: &mut SmallVec<[SN; 32]>) -> bool {
        for first in 0..nums.len() {
            unsafe {
                if nums.get_unchecked(first).depth > 4 {
                    if first > 0 {
                        nums.get_unchecked_mut(first - 1).val += nums.get_unchecked(first).val;
                    }
                    let second = first + 1;
                    if second < (nums.len() - 1) {
                        nums.get_unchecked_mut(second + 1).val += nums.get_unchecked(second).val;
                    }
                    nums.get_unchecked_mut(second).val = 0;
                    nums.get_unchecked_mut(second).depth -= 1;
                    nums.remove(first);
                    return true;
                }
            }
        }
        false
    }

    fn split(nums: &mut SmallVec<[SN; 32]>) -> bool {
        for i in 0..nums.len() {
            unsafe {
                if nums.get_unchecked(i).val >= 10 {
                    let val = nums.get_unchecked(i).val;
                    nums.get_unchecked_mut(i).val = val / 2;
                    nums.get_unchecked_mut(i).depth += 1;
                    nums.insert(
                        i + 1,
                        SN {
                            val: (val + 1) / 2,
                            depth: nums.get_unchecked(i).depth,
                        },
                    );
                    return true;
                }
            }
        }
        false
    }

    fn magnitude(nums: &[SN]) -> i32 {
        let mut nums = nums.to_owned();
        for depth in (0..=4).rev() {
            let mut new_nums = Vec::new();
            let mut i = 0;
            while i < nums.len() {
                if nums[i].depth == depth {
                    if i + 1 < nums.len() && nums[i].depth == nums[i + 1].depth {
                        new_nums.push(SN {
                            depth: depth.saturating_sub(1),
                            val: nums[i].val * 3 + nums[i + 1].val * 2,
                        });
                        i += 1;
                    } else {
                        new_nums.push(SN {
                            depth: depth.saturating_sub(1),
                            val: nums[i].val,
                        });
                    }
                } else {
                    new_nums.push(nums[i]);
                }
                i += 1;
            }
            nums = new_nums;
        }
        nums[0].val
    }
}
pub struct Day18;
impl Day for Day18 {
    type Parsed = Vec<SmallVec<[SN; 32]>>;

    fn parse(input: impl Iterator<Item = String>) -> Self::Parsed {
        input
            .map(|line| {
                let mut depth = 0;
                let mut nums = SmallVec::<[SN; 32]>::new();
                for c in line.chars() {
                    match c {
                        '[' => depth += 1,
                        ']' => depth -= 1,
                        ',' => {}
                        n => nums.push(SN {
                            val: n.to_digit(10).unwrap() as i32,
                            depth,
                        }),
                    }
                }
                nums
            })
            .collect()
    }
    fn first(mut nums: Self::Parsed) -> String {
        while nums.len() > 1 {
            let mut new = nums.remove(0);
            SN::add(&mut new, nums.remove(0));
            nums.insert(0, new);
        }
        let val = SN::magnitude(&nums[0]);

        val.to_string()
    }
    fn second(nums: Self::Parsed) -> String {
        let mut max = 0;
        for group in nums.iter().permutations(2) {
            let mut new = group[0].clone();
            SN::add(&mut new, group[1].clone());
            max = max.max(SN::magnitude(&new));
        }
        max.to_string()
    }
}
