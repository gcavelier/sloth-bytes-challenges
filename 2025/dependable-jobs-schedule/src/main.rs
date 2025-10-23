fn main() {}

#[cfg(test)]
fn finish_all(nb_jobs: usize, deps: &[(usize, usize)]) -> bool {
    for i in 0..nb_jobs {
        if let Some(job_dep) = deps
            .iter()
            .find_map(|(src, dst)| if *src == i { Some(*dst) } else { None })
        {
            // We found a dependency, checking if the reverse dependency exists
            if deps
                .iter()
                .find(|(src_item, dst_item)| *src_item == job_dep && *dst_item == i)
                .is_some()
            {
                return false;
            }
        }
    }
    true
}

#[test]
fn test_function() {
    assert_eq!(finish_all(2, &vec![(1, 0), (0, 1)]), false);
    assert_eq!(finish_all(3, &vec![(1, 0), (2, 1)]), true);
    assert_eq!(finish_all(1, &vec![]), true);
    assert_eq!(
        finish_all(
            11,
            &vec![
                (6, 10),
                (4, 3),
                (9, 2),
                (2, 3),
                (6, 1),
                (2, 8),
                (10, 1),
                (10, 2),
                (5, 3),
                (0, 10),
                (7, 4),
                (6, 1)
            ]
        ),
        true
    );
}
