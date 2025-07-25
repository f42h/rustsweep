#[macro_export]
macro_rules! correct_range {
    ($from:expr, $to:expr) => {
        $from < $to
    };
}