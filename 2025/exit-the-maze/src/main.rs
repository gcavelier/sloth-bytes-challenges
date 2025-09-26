fn main() {}

#[cfg(test)]
fn can_exit(input: &[&[usize]]) -> bool {
    use pathfinding::{directed::bfs, grid::Grid};

    let nb_rows = input.len();
    let nb_columns = input.get(0).unwrap().len();
    let mut grid = Grid::new(nb_columns, nb_rows);

    for (row_idx, row) in input.iter().enumerate() {
        for (column_idx, value) in row.iter().enumerate() {
            if *value == 0 {
                grid.add_vertex((column_idx, row_idx));
            }
        }
    }

    bfs::bfs(
        &(0, 0),
        |coord| grid.neighbours(*coord),
        |(x, y)| *x == grid.width - 1 && *y == grid.height - 1,
    )
    .is_some()
}

#[test]
fn test_can_exit() {
    assert_eq!(
        can_exit(&[
            &[0, 1, 1, 1, 1, 1, 1],
            &[0, 0, 1, 1, 0, 1, 1],
            &[1, 0, 0, 0, 0, 1, 1],
            &[1, 1, 1, 1, 0, 0, 1],
            &[1, 1, 1, 1, 1, 0, 0]
        ]),
        true
    );

    assert_eq!(
        can_exit(&[
            &[0, 1, 1, 1, 1, 1, 1],
            &[0, 0, 1, 0, 0, 1, 1],
            &[1, 0, 0, 0, 0, 1, 1],
            &[1, 1, 0, 1, 0, 0, 1],
            &[1, 1, 0, 0, 1, 1, 1]
        ]),
        false
    );

    assert_eq!(
        can_exit(&[
            &[0, 1, 1, 1, 1, 0, 0],
            &[0, 0, 0, 0, 1, 0, 0],
            &[1, 1, 1, 0, 0, 0, 0],
            &[1, 1, 1, 1, 1, 1, 0],
            &[1, 1, 1, 1, 1, 1, 1]
        ]),
        false
    );

    assert_eq!(
        can_exit(&[
            &[0, 1, 1, 1, 1, 0, 0],
            &[0, 0, 0, 0, 1, 0, 0],
            &[1, 1, 1, 0, 0, 0, 0],
            &[1, 0, 0, 0, 1, 1, 0],
            &[1, 1, 1, 1, 1, 1, 0]
        ]),
        true
    );
}
