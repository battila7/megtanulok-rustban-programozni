use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut bottom: i32 = 0;
    let mut top: i32 = (array.len() as i32) - 1;

    while bottom <= top {
        let index = ((top - bottom) / 2) + bottom;

        match array[index as usize].cmp(&key) {
            Ordering::Less => bottom = index + 1,
            Ordering::Greater => top = index - 1,
            _ => return Some(index as usize),
        }
    }
    
    None
}
