pub fn bubble_sort(list: &mut[i32]) -> &[i32] {
    let mut changed: bool = false;

    for i in 0..list.len()-1 {
        if list[i + 1] < list[i] {
            changed = true;
            let temp = list[i+1];
            list[i+1] = list[i];
            list[i] = temp;
        }
    }

    if changed {
        return bubble_sort(list)
    }

    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_sort_seven_items() {
        let mut list: [i32; 7] = [6,7,0,5,1,3,2];
        let result = bubble_sort(&mut list);
        assert_eq!(result, [0,1,2,3,5,6,7]);
    }

    #[test]
    fn should_sort_ten_items() {
        let mut list: [i32; 10] = [6,7,0,5,1,3,2,97,50,41];
        let result = bubble_sort(&mut list);
        assert_eq!(result, [0,1,2,3,5,6,7,41,50,97]);
    }

    #[test]
    fn should_sort_negative_items() {
        let mut list: [i32; 10] = [6,7,0,5,1,3,2,-50,40,-1];
        let result = bubble_sort(&mut list);
        assert_eq!(result, [-50,-1,0,1,2,3,5,6,7,40]);
    }
}
