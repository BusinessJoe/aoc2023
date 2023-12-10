#[derive(Debug)]
pub struct Race {
    pub time: u64,
    pub dist: u64,
}

fn score_race(race: &Race) -> u64 {
    let t: f64 = race.time as f64;
    let d: f64 = race.dist as f64;
    let disc: f64 = (t * t - 4.0 * d).sqrt();
    let min = (0.5 * (t - disc)).floor();
    let max = (0.5 * (t + disc)).ceil();

    max as u64 - min as u64 - 1
}

pub fn solution1(races: &[Race]) -> u64 {
    races.iter().map(score_race).product()
}

pub fn solution2(race: &Race) -> u64 {
    score_race(race)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let races = vec![
            Race { time: 7, dist: 9 },
            Race { time: 15, dist: 40 },
            Race {
                time: 30,
                dist: 200,
            },
        ];

        let p1 = solution1(&races);
        assert_eq!(288, p1);
    }
}
