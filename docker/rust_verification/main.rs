use verification_annotations as verifier;

fn main() {
    let a = verifier::abstract_value::<u32>();
    let b = verifier::abstract_value::<u32>();
    verifier::assume(1 <= a && a <= 1000);
    verifier::assume(1 <= b && b <= 1000);
    if verifier::is_replay() {
        eprintln!("Test values: a = {}, b = {}", a, b);
    }
    let r = a*b;
    assert!(1 <= r && r <= 1000000);
}
