/// Takes as input a sequence of structs, and converts them to a series of
/// bytes. All traits that implement `Bytes` can be automatically converted to
/// bytes in this manner.
#[macro_export]
macro_rules! to_bytes {
    ($($x:expr),*) => ({
        let mut buf = vec![];
        {$crate::push_to_vec!(buf, $($x),*)}.map(|_| buf)
    });
}

#[doc(hidden)]
#[macro_export]
macro_rules! push_to_vec {
    ($buf:expr, $y:expr, $($x:expr),*) => ({
        {
            use ark_serialize::CanonicalSerialize;
            $y.serialize_compressed(&mut $buf)
        }.and({$crate::push_to_vec!($buf, $($x),*)})
    });

    ($buf:expr, $x:expr) => ({
        use ark_serialize::CanonicalSerialize;
        $x.serialize_compressed(&mut $buf)
    })
}
