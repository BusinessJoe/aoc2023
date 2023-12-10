use day6::{solution1, solution2, Race};

fn main() {
    let races = vec![
        Race {
            time: 48,
            dist: 296,
        },
        Race {
            time: 93,
            dist: 1928,
        },
        Race {
            time: 85,
            dist: 1236,
        },
        Race {
            time: 95,
            dist: 1391,
        },
    ];

    let p1 = solution1(&races);
    let p2 = solution2(&Race {
        time: 48_93_85_95,
        dist: 296_1928_1236_1391,
    });
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
