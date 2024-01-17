fn some_function(num: Option<usize>) -> Option<usize> {
    Some(num? * 5)
}

fn practice(nums: Vec<usize>, index: usize) -> usize {
    return nums.get(index).unwrap_or(&index) * 5;
}

fn main() {
    some_function(Some(5));
}
