
pub fn accumulate<T>(nums: &[i32], f: T) -> Vec<i32>  where T: Fn(&i32) -> i32  {
    let mut res = Vec::with_capacity(nums.len());
    for i in nums {
        res.push(f(i));
    }
    res
}

#[cfg(test)]
mod tests {

    #[test]
    fn func_single() {
        let input = vec![2];
        let expected = vec![4];
        assert_eq!(super::accumulate(&input,|x| { x * x}), expected);
    }
}