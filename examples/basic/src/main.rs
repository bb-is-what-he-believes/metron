metron_core::def_unit_measure!(some_unit::SomeUnit);
fn main() {
    use some_unit::SomeUnit;
    let lhs = SomeUnit::of(4);
    let rhs = SomeUnit::from(3);
    assert_eq!(&7, (lhs + rhs).num());
    assert_eq!(1, (lhs - rhs).into_num());
    assert_eq!(SomeUnit::new(8), lhs * 2);
    assert!(lhs / 2 < rhs);
}
