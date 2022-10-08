/// Counts the number of paths thru a grid of the given size
/// if the paths can only move right or down at each point
pub fn count_paths(grid_size: usize) -> u128 {
    let mut paths_count: Vec<Vec<Option<u128>>> = vec![];
    for _i in 0..=grid_size {
        let mut row = vec![];
        for _j in 0..=grid_size {
            row.push(None);
        }
        paths_count.push(row);
    }

    return paths_to_point(&mut paths_count, grid_size, grid_size);
}

fn paths_to_point(paths_count: &mut Vec<Vec<Option<u128>>>, i: usize, j: usize) -> u128 {
    if let Some(count) = paths_count[i][j] {
        return count;
    }

    let count;
    if i == 0 && j == 0 {
        count = 1;
    } else if i == 0 {
        count = paths_to_point(paths_count, i, j - 1);
    } else if j == 0 {
        count = paths_to_point(paths_count, i - 1, j);
    } else {
        count = paths_to_point(paths_count, i - 1, j) + paths_to_point(paths_count, i, j - 1);
    }
    paths_count[i][j] = Some(count);

    return count;
}

#[cfg(test)]
mod tests {
    use super::count_paths;

    #[test]
    fn test_count_paths() {
        assert_eq!(6, count_paths(2));
    }
}
