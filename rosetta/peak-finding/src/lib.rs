pub fn d1_peak<T: PartialOrd + Copy>(list: &[T]) -> T {
    let len = list.len();
    if list[len/2] < list[len/2 - 1] {
        for i in 1..(len/2) {
            if list[0] > list[1] {
                return list[0]
            }
            if list[i] > list[i - 1] && list[i] > list[i + 1] {
                return list[i]
            }
        }
    } else if list[len/2] < list[len/2 + 1] {
        for i in (len/2 + 1)..(len) {
            if i == (len - 1) && (list[i] > list[i-1]) {
                return list[i]
            }
            if list[i] > list[i - 1] && list[i] > list[i + 1] {
                return list[i]
            }
        }
    }
    return list[len/2];
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let v = [7, 6, 4, 3, 2, 1, 5];
        let peak = d1_peak(&v);
        assert_eq!(peak, 7);
    }
}
