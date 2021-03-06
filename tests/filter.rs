#![allow(
    clippy::precedence,
    clippy::type_complexity,
    clippy::float_cmp,
    clippy::len_zero,
    clippy::double_neg,
    clippy::clippy::many_single_char_names
)]
#![allow(dead_code)]

extern crate fundsp;

use fundsp::hacker::*;

#[test]
fn test_filter() {
    let mut rnd = AttoRand::new(1);

    // Test follow().
    for _ in 0..200 {
        // Bias testing toward smaller lengths to cut testing time shorter.
        let samples = round(xerp(1.0, 100_000.0, squared(rnd.gen_01::<f64>())));
        let sample_rate = xerp(10.0, 100_000.0, rnd.gen_01::<f64>());
        let mut x = follow(samples / sample_rate);
        x.reset(Some(sample_rate));
        let goal = lerp(-100.0, 100.0, rnd.gen_01::<f64>());
        for _ in 0..samples as usize {
            x.filter_mono(goal);
        }
        // Promise was 0.5% accuracy between 1 and 100k samples.
        let response = x.value() / goal;
        assert!(response >= 0.495 && response <= 0.505);
    }

    // Test afollow().
    for _ in 0..200 {
        // Bias testing toward smaller lengths to cut testing time shorter.
        let attack_samples = round(xerp(1.0, 100_000.0, squared(rnd.gen_01::<f64>())));
        let release_samples = round(xerp(1.0, 100_000.0, squared(rnd.gen_01::<f64>())));
        let sample_rate = xerp(10.0, 100_000.0, rnd.gen_01::<f64>());
        let goal = lerp(-100.0, 100.0, rnd.gen_01::<f64>());
        let mut x = followa(attack_samples / sample_rate, release_samples / sample_rate);
        x.reset(Some(sample_rate));
        for _ in 0..(if goal > 0.0 {
            attack_samples
        } else {
            release_samples
        }) as usize
        {
            x.filter_mono(goal);
        }
        // Promise was 0.5% accuracy between 1 and 100k samples.
        let response = x.value() / goal;
        assert!(response >= 0.495 && response <= 0.505);
    }
}
