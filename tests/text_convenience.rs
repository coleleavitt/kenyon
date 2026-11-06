//! `FlyModel` and `FlyBloom` from text input, via [`kenyon::HashedText`].
//!
//! The paper surfaces take `&[f32]`; the text encoder closes the gap so a
//! downstream caller doesn't have to ship its own text→features step.

use kenyon::{
    BloomParams, Coding, FlyBloom, FlyHash, FlyHashParams, FlyModel, FlyModelParams, HashedText,
    LearningRule, Scratch,
};

const DIM: usize = 128;
const SEED: u64 = 7;

fn encoder() -> HashedText {
    HashedText::new(DIM, SEED)
}

#[test]
fn flybloom_scores_text_and_remembers_it() {
    let encoder = encoder();
    let hasher = FlyHash::new(FlyHashParams::paper(DIM, 8, SEED)).unwrap();
    let mut bloom = FlyBloom::new(hasher, BloomParams::STATIC).unwrap();
    let text = "install the dependencies with bun install, not npm install";

    // First sighting: novel.
    let first = bloom.novelty_text(&encoder, text).unwrap();
    assert!(first > 0.5, "cold bloom reports high novelty: {first}");

    bloom.insert_text(&encoder, text).unwrap();
    let second = bloom.novelty_text(&encoder, text).unwrap();
    assert!(second < first, "same text reads less novel after insert: {second} < {first}");

    // A different piece of text still reads novel.
    let novel = bloom
        .novelty_text(&encoder, "restart the daemon to see new code")
        .unwrap();
    assert!(novel > second, "unrelated text stays novel");
}

#[test]
fn flybloom_observe_text_scores_and_stores_in_one_call() {
    let encoder = encoder();
    let hasher = FlyHash::new(FlyHashParams::paper(DIM, 8, SEED)).unwrap();
    let mut bloom = FlyBloom::new(hasher, BloomParams::STATIC).unwrap();
    let mut scratch = Scratch::default();

    let a = bloom
        .observe_text(&encoder, "a brand new memory to record", &mut scratch)
        .unwrap();
    let b = bloom
        .observe_text(&encoder, "a brand new memory to record", &mut scratch)
        .unwrap();
    assert!(a > b, "first sighting > second: {a} > {b}");
}

#[test]
fn flymodel_trains_on_text_examples() {
    let encoder = encoder();
    let params = FlyModelParams {
        n_inputs: DIM,
        n_kc: 320,
        fan_in: 4,
        n_active: 16,
        n_classes: 2,
        learning_rate: 0.1,
        forgetting: 0.0,
        coding: Coding::Sparse,
        rule: LearningRule::Associative,
        seed: SEED,
    };
    let mut model = FlyModel::new(params).unwrap();
    // Four training examples per class; vocabularies barely overlap.
    let class_a = [
        "install the dependencies with bun install",
        "run the test suite under bun",
        "the bun lockfile pins the deps",
        "bun install never uses npm",
    ];
    let class_b = [
        "restart the daemon to see new code",
        "the supervisor owns worker lifecycle",
        "a daemon restart kills active agents",
        "the supervisor prints lifecycle logs",
    ];
    for t in class_a {
        model.train_text(&encoder, t, 0).unwrap();
    }
    for t in class_b {
        model.train_text(&encoder, t, 1).unwrap();
    }

    assert_eq!(
        model.predict_text(&encoder, "which way do I install").unwrap(),
        0
    );
    assert_eq!(
        model.predict_text(&encoder, "restart the daemon please").unwrap(),
        1
    );

    let scores = model.scores_text(&encoder, "bun install dependencies").unwrap();
    assert_eq!(scores.len(), 2);
    assert!(
        scores[0] > scores[1],
        "class 0 wins on an install-flavoured query: {scores:?}"
    );
}

#[test]
fn flymodel_text_dim_mismatch_errors_cleanly() {
    let wide_encoder = HashedText::new(DIM * 2, SEED);
    let params = FlyModelParams {
        n_inputs: DIM,
        n_kc: 160,
        fan_in: 4,
        n_active: 8,
        n_classes: 2,
        learning_rate: 0.1,
        forgetting: 0.0,
        coding: Coding::Sparse,
        rule: LearningRule::Associative,
        seed: SEED,
    };
    let mut model = FlyModel::new(params).unwrap();
    // The encoder's dim doesn't match n_inputs → the inner `train_one`
    // returns DimensionMismatch. The text convenience method never panics.
    assert!(model.train_text(&wide_encoder, "hello", 0).is_err());
}
