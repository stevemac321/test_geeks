extern crate test_geeks;

#[test]
fn test_rotate_grid() {
    let mut grid = [
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
        [13, 14, 15, 16],
    ];
    test_geeks::rotate_grid::rotate_grid(&mut grid);
    assert_eq!(grid, [
        [13, 9, 5, 1],
        [14, 10, 6, 2],
        [15, 11, 7, 3],
        [16, 12, 8, 4],
    ]);
}
