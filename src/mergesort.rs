pub fn mergesort(vec: &mut Vec<i32>) {
    let n = vec.len();
    if n <= 1 { // Recursion base case
        return;
    }

    let m = n/2; // Partition
    mergesort(&mut vec[0..m].to_vec());
    mergesort(&mut vec[m..n].to_vec());

    let mut res: Vec<i32> = vec.to_vec();
    merge(&vec[0..m], &vec[m..n], &mut res[..]); // Merge
}

fn merge(one: &[i32], two: &[i32], res: &mut [i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < one.len() && j < two.len() {
        if one[i] < two[j] {
            res[k] = one[i];
            k += 1;
            i += 1;
        } else {
            res[k] = two[j];
            k += 1;
            j += 1;
        }
    }
    if i < one.len() {
        res[k..].copy_from_slice(&one[i..]);
    }
    if j < two.len() {
        res[k..].copy_from_slice(&two[j..]);
    }
}