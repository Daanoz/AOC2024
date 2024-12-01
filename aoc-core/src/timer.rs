#[macro_export]
macro_rules! timed {
    ($x: expr) => {{
        let start = std::time::Instant::now();
        let result = $x;
        (result, start.elapsed())
    }};
}

#[macro_export]
macro_rules! print_timed {
    ($x: expr) => {
        $crate::print_timed!($x, $x)
    };
    ($label: expr, $x: expr) => {{
        let (result, time) = $crate::timed!($x);
        println!("{}: {:.2?}", stringify!($label), time);
        result
    }};
}
