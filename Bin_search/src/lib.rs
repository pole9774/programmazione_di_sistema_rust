// soluzione ricorsiva con genrics
pub fn find<T, U>(array: U, key: T) -> Option<usize> 
where 
    // T deve essere confrontabile
    T: PartialEq + PartialOrd,
    // U deve essere un array di T da cui è possibile prendere un ref con as_ref()
    U: AsRef<[T]> {
    
    let array = array.as_ref();

    match array.len() {
        0 => None,
        1 => {
            if array[0] == key {
                Some(0)
            } else {
                None
            }
        },
        _ => {
            let mid = array.len() / 2;
            if array[mid] == key {
                Some(mid)
            } else if array[mid] > key {
                find(&array[..mid], key)
            } else {
                Some(mid + find(&array[mid..], key)?)
            }
        }
    }
}

// soluzione iterativa
pub fn find_iter(array: &[i32], key: i32) -> Option<usize> {
    let mut start: i32 = 0;
    let mut end: i32 = array.len() as i32 - 1;

    loop {
        if start > end {
            return None;
        }

        let mid = (start + end) / 2;
        let v = array[mid as usize];
        if v == key {
            return Some(mid as usize);
        } else if v > key {
            end = mid - 1;
        } else {
            start = mid + 1;
        }
    }
}
