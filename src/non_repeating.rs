
pub fn non_repeating(v: Vec<i32>) -> Vec<i32> {
    let mut rv = Vec::new();

    // Iterate over each element in the vector `v`
    for &num in &v {
        // `position` returns `Some(index)` if `num` is found in `rv`, otherwise `None`
        if let Some(pos) = rv.iter().position(|&x| x == num) {
            // If `num` is found in `rv`, remove it
            rv.remove(pos);
        } else {
            // If `num` is not found in `rv`, add it
            rv.push(num);
        }
    }

    // Sort the resulting vector `rv`
    rv.sort();
    rv
}

