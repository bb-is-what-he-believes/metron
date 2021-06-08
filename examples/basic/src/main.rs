use metron_core::def_unit_measure;
def_unit_measure!(some_unit::SomeUnit);
fn main() {
    use some_unit::SomeUnit;
    let lhs = SomeUnit::of(4);
    let rhs = SomeUnit::of(3);
    assert_eq!(&7, (lhs + rhs).num());
    assert_eq!(1, (lhs - rhs).into_num());
    assert_eq!(SomeUnit::of(8), lhs * 2);
    assert_eq!(true, lhs / 2 < rhs);
}
