fn main() {}

#[cfg(test)]
fn can_reach_exit(input: &[&str]) -> bool {
    use pathfinding::{directed::bfs, grid::Grid};

    let mut start_position = (0, 0);
    let mut exit = (0, 0);
    let nb_rows = input.len();
    let nb_columns = input.get(0).unwrap().len();
    let mut grid = Grid::new(nb_columns, nb_rows);

    for (row_idx, row) in input.iter().enumerate() {
        for (column_idx, value) in row.chars().enumerate() {
            match value {
                '#' => {
                    grid.add_vertex((column_idx, row_idx));
                }
                '.' => {}
                '@' => start_position = (column_idx, row_idx),
                'E' => exit = (column_idx, row_idx),
                _ => unreachable!(),
            }
        }
    }

    grid.invert();

    bfs::bfs(
        &start_position,
        |coord| {
            let plop = grid.neighbours(*coord);
            println!("coord={coord:?} plop={plop:?}");
            plop
        },
        |(x, y)| *x == exit.0 && *y == exit.1,
    )
    .is_some()
}

#[test]
fn test_function() {
    assert_eq!(can_reach_exit(&["@..", ".#E", "..."]), true);
    assert_eq!(can_reach_exit(&["@#E"]), false);
    assert_eq!(can_reach_exit(&["@.#.", "..#E", "####"]), false);
    assert_eq!(can_reach_exit(&["@...", ".###", "...E"]), true);
}
