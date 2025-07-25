use criterion::{criterion_group, criterion_main, Criterion};
use win_hotkeys::state::KeyboardState;

fn bench_key_ops(c: &mut Criterion) {
    let mut state = KeyboardState::new();
    c.bench_function("key_ops", |b| {
        b.iter(|| {
            for key in 0..256 {
                state.keydown(key);
                state.is_down(key);
                state.keyup(key);
            }
        })
    });
}

criterion_group!(benches, bench_key_ops);
criterion_main!(benches);
