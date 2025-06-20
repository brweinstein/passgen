use std::collections::HashSet;

#[test]
fn test_random_idx_distribution() {
    let max = 100;
    let mut seen = HashSet::new();

    for _ in 0..10_000 {
        let idx = pgen::random::random_idx(max);
        assert!(idx < max, "Index out of bounds: {}", idx);
        seen.insert(idx);
    }

    let diversity = seen.len();
    println!("Unique values generated: {}", diversity);

    assert!(
        diversity > max * 8 / 10,
        "Random index generation lacks entropy: saw {} unique values out of {}",
        diversity,
        max
    );
}
