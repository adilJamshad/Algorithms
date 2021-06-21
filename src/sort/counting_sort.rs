// Works best when the range of the values (i.e. max_value - min_value) is small.
//
// - Time complexity: O(n+maxVal-maxVal)
pub fn counting_sort<T: num_traits::PrimInt>(v: &mut [T]) {
    let mut min = T::min_value();
    let mut max = T::max_value();
    for &n in v.iter() {
        if n < min {
            min = n;
        } else if n > max {
            max = n;
        }
    }

    let sz = (max - min + T::one()).to_usize().unwrap();
    // `occurence[i]` stores how many times `i` occured in the vector to be sorted;
    // later, we can then build the sorted vector by reading the `occurence` from
    // left to right, pushing the value `i` to the sorted vector `occurence[i]` times.
    let mut occurence = vec![0; sz];
    for m in v.iter().map(|&n| (n - min).to_usize().unwrap()) {
        occurence[m] += 1;
    }
    let mut k = 0;
    for idx in 0..sz {
        let i = T::from(idx).unwrap() + min;
        for _ in 0..occurence[idx] {
            v[k] = i;
            k += 1;
        }
    }
}
