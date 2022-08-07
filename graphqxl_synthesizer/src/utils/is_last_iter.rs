pub(crate) fn is_last_iter<T, I: Iterator<Item = T>>(iterable: I) -> Vec<(bool, T)> {
    let mut result = Vec::new();
    for item in iterable {
        result.push((false, item));
    }
    if let Some(last) = result.last_mut() {
        last.0 = true;
    }
    result
}
