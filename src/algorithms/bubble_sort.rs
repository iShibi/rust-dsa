pub fn bubble_sort(mut v: Vec<i32>) -> Vec<i32> {
    let len = v.len();
    for i in 0..len - 1 {
        for j in i + 1..len {
            if v[i] > v[j] {
                let temp = v[i];
                v[i] = v[j];
                v[j] = temp;
            }
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn it_bubble_sorts() {
        let v = vec![3, 1, 4, 2];
        let result = bubble_sort(v);
        assert_eq!(result, vec![1, 2, 3, 4]);
    }
}
