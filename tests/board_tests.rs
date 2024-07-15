use std::collections::{HashMap, HashSet};

use ts1_game_2048_rust::component::tile::Tile;
use ts1_game_2048_rust::component::board::Board;


#[test]
fn test_board_new() {
    let board = Board::new();
    assert_eq!(board.tiles.len(), 2);
    assert_eq!(board.score, 0);
}

#[test]
fn test_board_spawn_tile() {
    let mut board = Board::new();
    board.spawn_tile();
    assert_eq!(board.count_tiles(), 3);
}

#[test]
fn test_board_add_tile() {
    let mut board = Board::new();
    let tile = Tile::new(2, 0, 0);
    board.add_tile(tile);
    assert_eq!(board.count_tiles(), 3);
    assert_eq!(board.get_empty_positions().len(), 13);
}

#[test]
fn test_random_tile_generation() {
    let iterations = 1000;
    let mut value_counts = HashMap::new();
    let mut position_counts = HashMap::new();

    for _ in 0..iterations {
        let mut new_board = Board::new(); // 매 반복마다 새 보드 생성
        if let Some(tile) = new_board.spawn_tile() {
            *value_counts.entry(tile.value).or_insert(0) += 1;
            *position_counts.entry((tile.position_x, tile.position_y)).or_insert(0) += 1;
        }
    }

    // 값 분포 테스트
    assert!(value_counts.contains_key(&2), "2 값의 타일이 생성되지 않았습니다");
    assert!(value_counts.contains_key(&4), "4 값의 타일이 생성되지 않았습니다");

    let count_2 = value_counts.get(&2).unwrap_or(&0);
    let count_4 = value_counts.get(&4).unwrap_or(&0);

    let expected_2 = (iterations as f64 * 0.6) as i32;
    let expected_4 = (iterations as f64 * 0.4) as i32;
    let tolerance = (iterations as f64 * 0.02) as i32; // 2% 오차 허용

    assert!(
        (count_2 - expected_2).abs() < tolerance,
        "2 값의 타일 생성 비율이 예상 범위를 벗어났습니다. 예상: {}, 실제: {}",
        expected_2,
        count_2
    );
    assert!(
        (count_4 - expected_4).abs() < tolerance,
        "4 값의 타일 생성 비율이 예상 범위를 벗어났습니다. 예상: {}, 실제: {}",
        expected_4,
        count_4
    );

    // 위치 분포 테스트
    assert_eq!(position_counts.len(), 16, "일부 위치에 타일이 생성되지 않았습니다");

    let min_count = iterations / 20; // 각 위치에 최소 5% 이상 생성되어야 함
    for count in position_counts.values() {
        assert!(count > &min_count, "특정 위치에 타일이 거의 생성되지 않았습니다");
    }
}

#[test]
fn test_consecutive_tile_generations() {
    let board = Board::new();
    let mut generated_positions = HashSet::new();

    // 초기 보드 상태 확인 (2개의 타일이 이미 생성되어 있어야 함)
    assert_eq!(board.count_tiles(), 2, "초기 보드에 2개의 타일이 생성되지 않았습니다");

    // 초기에 생성된 2개 타일의 위치를 기록
    for tile in board.get_tiles() {
        generated_positions.insert((tile.position_x, tile.position_y));
    }

    let mut board = board; // 가변 참조로 변경

    // 14번 추가로 타일 생성 (보드를 가득 채움)
    for _ in 0..14 {
        if let Some(tile) = board.spawn_tile() {
            let pos = (tile.position_x, tile.position_y);
            assert!(!generated_positions.contains(&pos), "같은 위치에 타일이 중복 생성되었습니다");
            generated_positions.insert(pos);
        } else {
            panic!("보드가 가득 차기 전에 타일 생성이 멈췄습니다");
        }
    }

    assert_eq!(generated_positions.len(), 16, "모든 위치에 타일이 생성되지 않았습니다");
    assert!(board.spawn_tile().is_none(), "가득 찬 보드에 새 타일이 생성되었습니다");
}