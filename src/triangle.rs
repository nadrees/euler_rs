use std::cmp::max;

/// Finds the maximum sum when traversing the triangle from top
/// to bottom. Ex:
///
/// By starting at the top of the triangle below and moving to adjacent numbers on the row below, the maximum total from top to bottom is 23.
///
/// 3 <br/>
/// 7 4 <br/>
/// 2 4 6 <br/>
/// 8 5 9 3 <br/>
///
/// That is, 3 + 7 + 4 + 9 = 23.
pub fn maximum_sum_path(lines: &Vec<Vec<u8>>) -> u128 {
    let mut max_sums: Vec<Vec<Option<u128>>> = vec![];
    for i in 0..lines.len() {
        let mut line = vec![];
        for _j in 0..lines[i].len() {
            line.push(None);
        }
        max_sums.push(line);
    }
    let last_row_index = lines.len() - 1;
    let mut max_sum = 0;
    for i in 0..lines[last_row_index].len() {
        let sum = calculate_maximum_sum_path_to(lines, &mut max_sums, last_row_index, i);
        max_sum = max(max_sum, sum);
    }
    return max_sum;
}

fn calculate_maximum_sum_path_to(
    lines: &Vec<Vec<u8>>,
    max_sums: &mut Vec<Vec<Option<u128>>>,
    i: usize,
    j: usize,
) -> u128 {
    if let Some(max_sum) = max_sums[i][j] {
        return max_sum;
    }

    let sum: u128;
    let local_value: u128 = lines[i][j].into();
    if i == 0 {
        sum = local_value;
    } else if j == 0 {
        sum = calculate_maximum_sum_path_to(lines, max_sums, i - 1, j) + local_value;
    } else if j == lines[i].len() - 1 {
        sum = calculate_maximum_sum_path_to(lines, max_sums, i - 1, j - 1) + local_value;
    } else {
        sum = max(
            calculate_maximum_sum_path_to(lines, max_sums, i - 1, j),
            calculate_maximum_sum_path_to(lines, max_sums, i - 1, j - 1),
        ) + local_value;
    }
    max_sums[i][j] = Some(sum);
    return sum;
}

#[cfg(test)]
mod tests {
    use super::maximum_sum_path;

    #[test]
    fn test_maximum_sum_path() {
        let triangle = vec![vec![3], vec![7, 4], vec![2, 4, 6], vec![8, 5, 9, 3]];
        assert_eq!(23, maximum_sum_path(&triangle));
    }
}
