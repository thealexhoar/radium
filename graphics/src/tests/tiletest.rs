use Tile;
use Color;

#[test]
fn test_overlay() {
    let tile1 = Tile::new(None, None, 1);
    let tile2 = Tile::new(Some(Color::red()), None, 2);
    let tile3 = Tile::new(None, Some(Color::blue()), 3);
    let tile4 = Tile::new(Some(Color::green()), Some(Color::yellow()), 4);

    assert_eq!(tile1.overlay(tile2), Tile::new(Some(Color::red()), None, 2));
    assert_eq!(tile1.overlay(tile3), Tile::new(None, Some(Color::blue()), 1));
    assert_eq!(tile1.overlay(tile4), tile4);

    assert_eq!(tile2.overlay(tile1), Tile::new(Some(Color::red()), None, 2));
    assert_eq!(
        tile2.overlay(tile3),
        Tile::new(Some(Color::red()), Some(Color::blue()), 2)
    );
    assert_eq!(tile2.overlay(tile4), tile4);

    assert_eq!(tile3.overlay(tile1), Tile::new(None, Some(Color::blue()), 3));
    assert_eq!(
        tile3.overlay(tile2), 
        Tile::new(Some(Color::red()), Some(Color::blue()), 2)
    );
    assert_eq!(
        tile3.overlay(tile4),
        tile4
    );

    assert_eq!(tile4.overlay(tile1), tile4);
    assert_eq!(
        tile4.overlay(tile2), 
        Tile::new(Some(Color::red()), Some(Color::yellow()), 2)
    );
    assert_eq!(
        tile4.overlay(tile3), 
        Tile::new(Some(Color::green()), Some(Color::blue()), 4)
    );
}
    