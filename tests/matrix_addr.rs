use kaleidoscope_internal::matrix_addr::MatrixAddr;

type MA1 = MatrixAddr<7, 14>;
type MA2 = MatrixAddr<7, 8>;
type MA3 = MatrixAddr<7, 5>;
type MA4 = MatrixAddr<4, 12>;
    
fn test_indexed_access<const R: u8, const C: u8>(mut ma: MatrixAddr<R, C>) {
    let row = if R > 1 { 1 } else { 0 };
    let col = if C > 3 { 3 } else if C > 1 { C - 1 } else { 0 };

    ma.set_row(1);
    ma.set_col(col);
    assert_eq!(ma, MatrixAddr::<R, C>::create(row, col));

    let row = R - 1;
    ma.set_row(row);
    assert_eq!(ma.row(), row);
    assert_eq!(ma.col(), col);
    assert_eq!(ma, MatrixAddr::<R, C>::create(row, col));

    let col = C - 1;
    ma.set_col(col);
    assert_eq!(ma.row(), row);
    assert_eq!(ma.col(), col);
    assert_eq!(ma, MatrixAddr::<R, C>::create(row, col));

    let row = if R > 3 { 3 } else { 0 };
    let ma3 = MatrixAddr::<R, C>::create(row, col);
    assert_eq!(ma3.row(), row);
    assert_eq!(ma3.col(), col);
}

fn test_relation<const R1: u8, const C1: u8, const R2: u8, const C2: u8>(
    _ma1: MatrixAddr<R1, C1>,
    _ma2: MatrixAddr<R2, C2>,
) {
    let row1 = if R1 > 4 { R1 - 4 } else { R1 - 1 };
    let row2 = if R2 > 5 { R2 - 5 } else if R2 > 2 { R2 - 2 } else { 0 };

    let col1 = C1 - 1;
    let col2 = C2 - 1;

    let maddr1 = MatrixAddr::<R1, C1>::create(row1, col1);
    let maddr2 = MatrixAddr::<R2, C2>::create(row2, col2);

    let maddr3 = MatrixAddr::<R1, C1>::default();

    assert!(maddr1.is_valid());
    assert!(!maddr3.is_valid());

    assert_ne!(maddr1, maddr2);

    let maddr1_copy = MatrixAddr::<R2, C2>::from_addr(maddr1);
    if R1 <= R2 && C1 <= C2 {
        assert_eq!(maddr1_copy, maddr1);
    } else if R1 >= R2 && C1 <= C2 {
        assert!(maddr1_copy <= maddr1);
    } else {
        assert!(maddr1_copy >= maddr1);
    }

    assert!(maddr2 < maddr1);
    assert!(maddr1 > maddr2);

    assert!(maddr2 <= maddr1);
    assert!(maddr1 >= maddr2);

    assert!(maddr1 <= maddr1);
    assert!(maddr2 >= maddr2);
}

#[test]
fn test_indexed_access_runner() {
    test_indexed_access(MA1::default());
    test_indexed_access(MA2::default());
    test_indexed_access(MA3::default());
    test_indexed_access(MA4::default());
}

#[test]
fn test_relation_runner() {
    test_relation(MA1::default(), MA1::default());
    test_relation(MA2::default(), MA2::default());
    test_relation(MA3::default(), MA3::default());
    test_relation(MA4::default(), MA4::default());

    test_relation(MA1::default(), MA2::default());
    test_relation(MA1::default(), MA2::default());
    test_relation(MA1::default(), MA4::default());

    test_relation(MA2::default(), MA1::default());
    test_relation(MA2::default(), MA3::default());
    test_relation(MA2::default(), MA4::default());

    test_relation(MA3::default(), MA1::default());
    test_relation(MA3::default(), MA2::default());
    test_relation(MA3::default(), MA4::default());

    test_relation(MA4::default(), MA1::default());
    test_relation(MA4::default(), MA2::default());
    test_relation(MA4::default(), MA3::default());
}
