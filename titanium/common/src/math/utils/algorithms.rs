// /// Sorts a vector using the quicksort algorithm
// pub fn quicksort<T,S>(arr: &mut S) where
//     T: Eq + PartialEq + Clone + PartialOrd,
//     S: IntoIterator<Item=T>
// {
//     let mut arr = arr.into_iter().collect::<Vec<T>>();
//     let slice = arr.as_mut_slice();
//     internals::quicksort(slice);
// }

// mod internals {
//     pub fn quicksort<T: PartialOrd>(v: &mut [T]) {
//         if !v.is_empty() {
//             let mut sep = 0;
//             for i in 1..v.len() {
//                 if v[i] < v[0] {
//                     sep += 1;
//                     v.swap(sep, i);
//                 }
//             }
        
//             v.swap(0, sep);
//             quicksort(&mut v[..sep]);
//             quicksort(&mut v[(sep + 1)..]);
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_quicksort() {
//         let mut v = vec![5, 4, 3, 2, 1];
//         quicksort(&mut v);
//         assert_eq!(v, vec![1, 2, 3, 4, 5]);
//     }
// }
