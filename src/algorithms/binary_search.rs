pub fn binary_search(v: Vec<i32>, key: i32) -> usize {
    let len = v.len();
    let (mut left, mut mid, mut right) = (0, 0, len);
    while left <= right {
        mid = left + ((right - left) / 2);
        if key == v[mid] {
            return mid;
        } else if key > v[mid] {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    mid
}

#[cfg(test)]
mod tests {
    use super::binary_search;

    #[test]
    fn it_binary_searches() {
        let v = vec![1, 2, 3, 4, 5];
        let key = 3;
        let result = binary_search(v, key);
        assert_eq!(result, 2);
    }
}
