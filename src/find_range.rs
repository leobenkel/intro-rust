use core::ops::RangeInclusive;

#[test]
fn range_inclusive_expression() {
    let mut range: RangeInclusive<i32> = 1..=3;

    let found = range.find(|&x| {
        println!("x: {}", x);
        x == 3
    });

    assert_eq!(found, Some(3));
}
