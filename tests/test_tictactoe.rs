extern crate test_geeks;

#[test]
fn test_player_a_wins() {
    let grid = [
        [-1, -1, -1],
        [0, 1, 0],
        [1, 1, 0],
    ];
    assert_eq!(test_geeks::tictactoe::check_winner(grid), -1);
}

#[test]
fn test_player_b_wins() {
    let grid = [
        [1, 1, 1],
        [-1, -1, 0],
        [0, 0, 0],
    ];
    assert_eq!(test_geeks::tictactoe::check_winner(grid), 1);
}

#[test]
fn test_tie() {
    let grid = [
        [1, -1, 1],
        [-1, 1, -1],
        [-1, 1, -1],
    ];
    assert_eq!(test_geeks::tictactoe::check_winner(grid), 2);
}

#[test]
fn test_no_winner_yet() {
    let grid = [
        [1, -1, 1],
        [-1, 0, -1],
        [-1, 1, -1],
    ];
    assert_eq!(test_geeks::tictactoe::check_winner(grid), 0);
}
