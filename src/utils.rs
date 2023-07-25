/// Splits the slice into a slice of `N`-element arrays,
/// starting at the beginning of the slice,
/// and a remainder slice with length strictly less than `N`.
///
/// This a port of the unstable `slice_as_chunks` feature.
///
/// # Safety
/// N must be non-zero.
///
/// # Examples
///
/// ```
/// use base16384::utils::slice_as_chunks;
///
/// let slice = ['l', 'o', 'r', 'e', 'm'];
/// let (chunks, remainder) = unsafe { slice_as_chunks(&slice) };
/// assert_eq!(chunks, &[['l', 'o'], ['r', 'e']]);
/// assert_eq!(remainder, &['m']);
/// ```
pub unsafe fn slice_as_chunks<T, const N: usize>(arr: &[T]) -> (&[[T; N]], &[T]) {
    let len = arr.len() / N;
    let (multiple_of_n, remainder) = arr.split_at(len * N);
    let array_slice = core::slice::from_raw_parts(multiple_of_n.as_ptr().cast(), len);
    (array_slice, remainder)
}

/// Splits the slice into a slice of `N`-element arrays,
/// starting at the beginning of the slice.
///
/// This a port of the unstable `slice_as_chunks` feature.
///
/// # Safety
/// N must be non-zero and the length of the slice must be a multiple of `N`.
///
/// # Examples
/// ```
/// use base16384::utils::slice_as_chunks_exact;
///
/// let slice = ['l', 'o', 'r', 'e'];
/// let chunks = unsafe { slice_as_chunks_exact(&slice) };
/// assert_eq!(chunks, &[['l', 'o'], ['r', 'e']]);
/// ```
pub unsafe fn slice_as_chunks_exact<T, const N: usize>(arr: &[T]) -> &[[T; N]] {
    let len = arr.len() / N;
    core::slice::from_raw_parts(arr.as_ptr().cast(), len)
}
