//! Schulze against the literature and against an independent reference.
//!
//! The reference enumerates every simple path (no Floyd-Warshall) and takes the best weakest edge,
//! so a mistake in the library's widest-path loop cannot hide in the reference as well.
mod common;
use common::*;
use fs_tally::{Ballot, Method, evaluate};

/// Example of Schulze's paper (45 voters, five candidates): E wins; the order is E, A, C, B, D.
#[test]
fn literature_example_with_45_voters() {
    let c = ids(&["A", "B", "C", "D", "E"]);
    let mut b = Vec::new();
    for (n, r) in [
        (5, ["A", "C", "B", "E", "D"]),
        (5, ["A", "D", "E", "C", "B"]),
        (8, ["B", "E", "D", "A", "C"]),
        (3, ["C", "A", "B", "E", "D"]),
        (7, ["C", "A", "E", "B", "D"]),
        (2, ["C", "B", "A", "D", "E"]),
        (7, ["D", "C", "E", "B", "A"]),
        (8, ["E", "B", "A", "D", "C"]),
    ] {
        b.extend(rankings(n, &r));
    }
    assert_eq!(b.len(), 45);
    let t = evaluate(&Method::Schulze, &c, &b).unwrap();
    assert_eq!(winners(&t), ["E"]);
    assert_eq!(order(&t), ["E", "A", "C", "B", "D"]);
}

/// Deterministic generator (no extra dependency, same sets on every run).
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> usize {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        usize::try_from(self.0 >> 33).unwrap()
    }
}

fn random_ranking(rng: &mut Lcg, n: usize) -> Vec<usize> {
    let mut all: Vec<usize> = (0..n).collect();
    for i in (1..n).rev() {
        all.swap(i, rng.next() % (i + 1));
    }
    all.truncate(1 + rng.next() % n); // partial: the rest is unranked
    all
}

fn prefs(n: usize, rankings: &[Vec<usize>]) -> Vec<Vec<i64>> {
    let mut d = vec![vec![0_i64; n]; n];
    for r in rankings {
        let pos = |c: usize| r.iter().position(|&x| x == c).unwrap_or(n);
        for i in 0..n {
            for j in 0..n {
                if pos(i) < pos(j) {
                    d[i][j] += 1;
                }
            }
        }
    }
    d
}

fn best_path(d: &[Vec<i64>], at: usize, to: usize, seen: &mut Vec<usize>, so_far: i64) -> i64 {
    if at == to {
        return so_far;
    }
    let mut best = 0;
    for next in 0..d.len() {
        if seen.contains(&next) || d[at][next] <= d[next][at] {
            continue;
        }
        seen.push(next);
        best = best.max(best_path(d, next, to, seen, so_far.min(d[at][next])));
        seen.pop();
    }
    best
}

fn reference_winners(n: usize, rankings: &[Vec<usize>]) -> Vec<usize> {
    let d = prefs(n, rankings);
    let p = |i: usize, j: usize| best_path(&d, i, j, &mut vec![i], i64::MAX);
    (0..n)
        .filter(|&i| (0..n).all(|j| j == i || p(i, j) >= p(j, i)))
        .collect()
}

#[test]
fn agrees_with_the_path_enumeration_reference() {
    let names = ["A", "B", "C", "D", "E"];
    let mut rng = Lcg(0x5eed);
    for case in 0..300 {
        let n = 3 + case % 3; // 3, 4, 5 choices
        let voters = 1 + rng.next() % 12;
        let rankings: Vec<Vec<usize>> = (0..voters).map(|_| random_ranking(&mut rng, n)).collect();
        let ballots: Vec<Ballot> = rankings
            .iter()
            .map(|r| Ballot::Ranking(r.iter().map(|&i| names[i].to_owned()).collect()))
            .collect();
        let choices = ids(&names[..n]);
        let got = evaluate(&Method::Schulze, &choices, &ballots).unwrap();
        let want: Vec<&str> = reference_winners(n, &rankings)
            .into_iter()
            .map(|i| names[i])
            .collect();
        assert_eq!(winners(&got), want, "case {case}: {rankings:?}");
    }
}

#[test]
fn a_condorcet_winner_is_the_only_winner() {
    // B beats A 3:2 and C 4:1; A beats C 3:2.
    let mut b = rankings(2, &["A", "B", "C"]);
    b.extend(rankings(2, &["B", "C", "A"]));
    b.extend(rankings(1, &["B", "A", "C"]));
    let t = evaluate(&Method::Schulze, &ids(&["A", "B", "C"]), &b).unwrap();
    assert_eq!(winners(&t), ["B"]);
}

#[test]
fn a_perfect_cycle_is_a_tie_of_all() {
    let mut b = rankings(1, &["A", "B", "C"]);
    b.extend(rankings(1, &["B", "C", "A"]));
    b.extend(rankings(1, &["C", "A", "B"]));
    let t = evaluate(&Method::Schulze, &ids(&["A", "B", "C"]), &b).unwrap();
    assert_eq!(winners(&t), ["A", "B", "C"]);
}
