#[macro_export]
macro_rules! gridmap {
    ($fn_name:ident, $static_name:ident, $grid:expr) => {
        static $static_name: OnceLock<HashMap<char, (isize, isize)>> = OnceLock::new();

        fn $fn_name() -> &'static HashMap<char, (isize, isize)> {
            $static_name.get_or_init(|| {
                $grid
                    .iter()
                    .enumerate()
                    .flat_map(|(row, row_data)| {
                        row_data
                            .iter()
                            .enumerate()
                            .map(move |(col, &ch)| (ch, (row as isize, col as isize)))
                    })
                    .collect()
            })
        }
    };
}
