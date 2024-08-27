pub fn binary_search(list: &[i32], target: i32) -> bool {
    let size = list.len();
    let half_index = size / 2;
    let half_item = list[half_index];

    if target == half_item {
        return true;
    }

    if size <= 1 {
        return false;
    }

    if target > half_item {
        return binary_search(&list[half_index .. size], target);
    }

    if target < half_item {
        return binary_search(&list[0 .. half_index], target);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_number() {
        let list: [i32; 6] = [5,6,3,4,1,10];
        let found = binary_search(&list, 4);
        assert!(found, "Number not found");
    }

    #[test]
    fn should_NOT_find_number() {
        let list: [i32; 6] = [1,2,3,4,5,6];
        let found = binary_search(&list, 7);
        assert!(!found, "Number found, call an exorcist");
    }
}
