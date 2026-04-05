use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use life::prelude::*;

fn load_beehive() -> World {
    let cells = Cells::try_from(Pattern::Glider.cells_str()).expect("require valid pattern");
    World::from(cells)
}

fn bench_next_generation(c: &mut Criterion) {
    let mut world = load_beehive();

    c.bench_function("world_next_generation", |b| {
        b.iter(|| {
            (0..1000).for_each(|_| world.next_generation());
            black_box(&world);
        });
    });
}

fn bench_generations_iterator(c: &mut Criterion) {
    let initial_world = load_beehive();

    c.bench_function("generations_iterator", |b| {
        b.iter(|| {
            let mut gens = Generations::new(initial_world.clone());
            let mut count = 0u32;

            while let Some(_) = gens.next_generation() {
                count += 1;
                if count >= 100 {
                    break;
                }
            }
            black_box(count);
        });
    });
}

criterion_group!(benches, bench_next_generation, bench_generations_iterator);
criterion_main!(benches);
