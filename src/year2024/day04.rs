pub fn part1(input: &str) -> String {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = input.len();
    let cols = input[0].len();

    let res: i32 = (0..rows)
        .into_iter()
        .map(|row| {
            let mut local_res = 0;
            for col in 0..cols {
                if input[row][col] == 'X' {
                    local_res += find_xmas(&input, row, col, rows, cols);
                }
            }
            local_res
        })
        .sum();

    res.to_string()
}

fn find_xmas(input: &Vec<Vec<char>>, row: usize, col: usize, rows: usize, cols: usize) -> i32 {
    let mut res = 0;
    // Prepare for spaghetti
    if col + 3 < cols // ->
        && input[row][col + 1] == 'M'
        && input[row][col + 2] == 'A'
        && input[row][col + 3] == 'S'
    {
        res += 1;
    }
    if col >= 3 // <-
        && input[row][col - 1] == 'M'
        && input[row][col - 2] == 'A'
        && input[row][col - 3] == 'S'
    {
        res += 1;
    }
    if row + 3 < rows // Up
        && input[row + 1][col] == 'M'
        && input[row + 2][col] == 'A'
        && input[row + 3][col] == 'S'
    {
        res += 1;
    }
    if row >= 3 // Down
        && input[row - 1][col] == 'M'
        && input[row - 2][col] == 'A'
        && input[row - 3][col] == 'S'
    {
        res += 1;
    }
    // Onto diagonals
    if row + 3 < rows // Up right
        && col + 3 < cols
        && input[row + 1][col + 1] == 'M'
        && input[row + 2][col + 2] == 'A'
        && input[row + 3][col + 3] == 'S'
    {
        res += 1;
    }
    if row >= 3 // Down right
        && col + 3 < cols
        && input[row - 1][col + 1] == 'M'
        && input[row - 2][col + 2] == 'A'
        && input[row - 3][col + 3] == 'S'
    {
        res += 1;
    }
    if row + 3 < rows // Up left
        && col >= 3
        && input[row + 1][col - 1] == 'M'
        && input[row + 2][col - 2] == 'A'
        && input[row + 3][col - 3] == 'S'
    {
        res += 1;
    }

    if row >= 3 // Down left
        && col >= 3
        && input[row - 1][col - 1] == 'M'
        && input[row - 2][col - 2] == 'A'
        && input[row - 3][col - 3] == 'S'
    {
        res += 1;
    }

    res
}

pub fn part2(input: &str) -> String {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = input.len();
    let cols = input[0].len();

    let res: i32 = (1..rows - 1)
        .into_iter()
        .map(|row| {
            let mut local_res = 0;
            for col in 1..cols - 1 {
                if input[row][col] == 'A' {
                    local_res += find_x_mas(&input, row, col, rows, cols);
                }
            }
            local_res
        })
        .sum();

    res.to_string()
}

fn find_x_mas(input: &Vec<Vec<char>>, row: usize, col: usize, _rows: usize, _cols: usize) -> i32 {
    let mut res = 0;

    let topleft = input[row - 1][col - 1];
    let topright = input[row - 1][col + 1];
    let bottomleft = input[row + 1][col - 1];
    let bottomright = input[row + 1][col + 1];

    let diag1_matches =
        (topleft == 'M' && bottomright == 'S') || (topleft == 'S' && bottomright == 'M');

    let diag2_matches =
        (topright == 'M' && bottomleft == 'S') || (topright == 'S' && bottomleft == 'M');

    if diag1_matches && diag2_matches {
        res += 1;
    }

    res
}
