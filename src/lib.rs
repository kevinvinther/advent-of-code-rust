pub mod utils;
pub mod year2023;
pub mod year2024;

#[macro_export]
macro_rules! register_solutions {
    ($map:expr, $($year:ident::$day:ident),* $(,)?) => {
        $(
            $map.insert(
                (stringify!($year).trim_start_matches("year"), stringify!($day).trim_start_matches("day")),
                ($year::$day::part1, $year::$day::part2),
            );
        )*
    };
}
