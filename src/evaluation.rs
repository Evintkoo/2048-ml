//! Reproducible descriptive score statistics for held-out game evaluations.

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

#[derive(Clone, Debug, PartialEq)]
pub struct ScoreSummary {
    pub n: usize,
    pub mean: f64,
    pub sample_std_dev: f64,
    pub median: f64,
    pub percentile_90: f64,
    pub percentile_99: f64,
    pub min: u64,
    pub max: u64,
    pub games_above_2048: usize,
    pub games_above_4096: usize,
    pub games_above_8192: usize,
    pub mean_ci_95: (f64, f64),
}

pub fn summarize_scores(
    scores: &[u64],
    seed: u64,
    bootstrap_replicates: usize,
) -> Option<ScoreSummary> {
    if scores.is_empty() || bootstrap_replicates == 0 {
        return None;
    }
    let mut ordered = scores.to_vec();
    ordered.sort_unstable();
    let ordered_f64: Vec<f64> = ordered.iter().map(|&score| score as f64).collect();
    let mean = scores.iter().map(|&score| score as f64).sum::<f64>() / scores.len() as f64;
    let variance = if scores.len() > 1 {
        scores
            .iter()
            .map(|&score| (score as f64 - mean).powi(2))
            .sum::<f64>()
            / (scores.len() - 1) as f64
    } else {
        0.0
    };
    let median = quantile_sorted(&ordered_f64, 0.5);
    let percentile_90 = quantile_sorted(&ordered_f64, 0.9);
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let mut means = Vec::with_capacity(bootstrap_replicates);
    for _ in 0..bootstrap_replicates {
        let sample_mean = (0..scores.len())
            .map(|_| scores[rng.gen_range(0..scores.len())] as f64)
            .sum::<f64>()
            / scores.len() as f64;
        means.push(sample_mean);
    }
    means.sort_by(f64::total_cmp);
    Some(ScoreSummary {
        n: scores.len(),
        mean,
        sample_std_dev: variance.sqrt(),
        median,
        percentile_90,
        percentile_99: quantile_sorted(&ordered_f64, 0.99),
        min: *ordered.first().unwrap(),
        max: *ordered.last().unwrap(),
        games_above_2048: scores.iter().filter(|&&score| score >= 2048).count(),
        games_above_4096: scores.iter().filter(|&&score| score >= 4096).count(),
        games_above_8192: scores.iter().filter(|&&score| score >= 8192).count(),
        mean_ci_95: (
            quantile_sorted(&means, 0.025),
            quantile_sorted(&means, 0.975),
        ),
    })
}

pub fn bootstrap_mean_difference_ci(
    first: &[u64],
    second: &[u64],
    seed: u64,
    bootstrap_replicates: usize,
) -> Option<(f64, f64)> {
    if first.is_empty() || second.is_empty() || bootstrap_replicates == 0 {
        return None;
    }
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let mut differences = Vec::with_capacity(bootstrap_replicates);
    for _ in 0..bootstrap_replicates {
        let first_mean = (0..first.len())
            .map(|_| first[rng.gen_range(0..first.len())] as f64)
            .sum::<f64>()
            / first.len() as f64;
        let second_mean = (0..second.len())
            .map(|_| second[rng.gen_range(0..second.len())] as f64)
            .sum::<f64>()
            / second.len() as f64;
        differences.push(first_mean - second_mean);
    }
    differences.sort_by(f64::total_cmp);
    Some((
        quantile_sorted(&differences, 0.025),
        quantile_sorted(&differences, 0.975),
    ))
}

/// Two-sided Mann–Whitney U test with average ranks, tie correction, and a
/// normal approximation. Suitable for the large held-out game counts in plans.
pub fn mann_whitney_u_pvalue(first: &[u64], second: &[u64]) -> Option<f64> {
    if first.is_empty() || second.is_empty() {
        return None;
    }
    let mut combined: Vec<(u64, bool)> = first
        .iter()
        .copied()
        .map(|score| (score, true))
        .chain(second.iter().copied().map(|score| (score, false)))
        .collect();
    combined.sort_unstable_by_key(|(score, _)| *score);
    let n1 = first.len() as f64;
    let n2 = second.len() as f64;
    let n = n1 + n2;
    let mut rank_sum_first = 0.0;
    let mut tie_term = 0.0;
    let mut begin = 0;
    while begin < combined.len() {
        let mut end = begin + 1;
        while end < combined.len() && combined[end].0 == combined[begin].0 {
            end += 1;
        }
        let tie_len = end - begin;
        let average_rank = ((begin + 1 + end) as f64) / 2.0;
        rank_sum_first += combined[begin..end]
            .iter()
            .filter(|(_, is_first)| *is_first)
            .count() as f64
            * average_rank;
        let t = tie_len as f64;
        tie_term += t.powi(3) - t;
        begin = end;
    }
    let u1 = rank_sum_first - n1 * (n1 + 1.0) / 2.0;
    let mean_u = n1 * n2 / 2.0;
    let tie_factor = 1.0 - tie_term / (n * (n - 1.0));
    let variance = n1 * n2 * (n + 1.0) * tie_factor / 12.0;
    if variance <= 0.0 {
        return Some(1.0);
    }
    let z = ((u1 - mean_u).abs() - 0.5).max(0.0) / variance.sqrt();
    Some((2.0 * (1.0 - normal_cdf(z))).clamp(0.0, 1.0))
}

/// Exact two-sided sign test on matched game seeds, ignoring ties.
/// This makes no symmetric-difference assumption required by Wilcoxon.
pub fn paired_sign_test_pvalue(first: &[u64], second: &[u64]) -> Option<f64> {
    if first.len() != second.len() || first.is_empty() {
        return None;
    }
    let (wins, losses) = first
        .iter()
        .zip(second)
        .fold((0_u64, 0_u64), |(wins, losses), (a, b)| match a.cmp(b) {
            std::cmp::Ordering::Greater => (wins + 1, losses),
            std::cmp::Ordering::Less => (wins, losses + 1),
            std::cmp::Ordering::Equal => (wins, losses),
        });
    let n = wins + losses;
    if n == 0 {
        return Some(1.0);
    }
    let smaller = wins.min(losses);
    let lower_tail = (0..=smaller)
        .map(|k| binomial_probability(n, k))
        .sum::<f64>();
    Some((2.0 * lower_tail).min(1.0))
}

fn binomial_probability(n: u64, k: u64) -> f64 {
    if k > n {
        return 0.0;
    }
    let k = k.min(n - k);
    let mut probability = 0.5_f64.powi(n as i32);
    for index in 1..=k {
        probability *= (n - k + index) as f64 / index as f64;
    }
    probability
}

/// Holm step-down adjustment, returned in the same order as the input p-values.
pub fn holm_adjust(p_values: &[f64]) -> Vec<f64> {
    let mut order: Vec<usize> = (0..p_values.len()).collect();
    order.sort_by(|&a, &b| p_values[a].total_cmp(&p_values[b]));
    let mut adjusted = vec![0.0; p_values.len()];
    let mut running_max: f64 = 0.0;
    for (rank, &index) in order.iter().enumerate() {
        let multiplier = (p_values.len() - rank) as f64;
        running_max = running_max.max((multiplier * p_values[index]).clamp(0.0, 1.0));
        adjusted[index] = running_max;
    }
    adjusted
}

pub fn cohens_d(first: &[u64], second: &[u64]) -> Option<f64> {
    if first.len() < 2 || second.len() < 2 {
        return None;
    }
    let first_mean = first.iter().map(|&x| x as f64).sum::<f64>() / first.len() as f64;
    let second_mean = second.iter().map(|&x| x as f64).sum::<f64>() / second.len() as f64;
    let first_var = first
        .iter()
        .map(|&x| (x as f64 - first_mean).powi(2))
        .sum::<f64>()
        / (first.len() - 1) as f64;
    let second_var = second
        .iter()
        .map(|&x| (x as f64 - second_mean).powi(2))
        .sum::<f64>()
        / (second.len() - 1) as f64;
    let pooled = (((first.len() - 1) as f64 * first_var + (second.len() - 1) as f64 * second_var)
        / (first.len() + second.len() - 2) as f64)
        .sqrt();
    (pooled > 0.0).then_some((first_mean - second_mean) / pooled)
}

fn normal_cdf(value: f64) -> f64 {
    let x = value / std::f64::consts::SQRT_2;
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.3275911 * x);
    let erf = 1.0
        - (((((1.061405429 * t - 1.453152027) * t + 1.421413741) * t - 0.284496736) * t
            + 0.254829592)
            * t)
            * (-x * x).exp();
    0.5 * (1.0 + sign * erf)
}

fn quantile_sorted(values: &[f64], probability: f64) -> f64 {
    if values.len() == 1 {
        return values[0];
    }
    let position = probability * (values.len() - 1) as f64;
    let lower = position.floor() as usize;
    let upper = position.ceil() as usize;
    let weight = position - lower as f64;
    values[lower] * (1.0 - weight) + values[upper] * weight
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_summary_and_bootstrap_are_reproducible() {
        let scores = [10, 20, 30, 40, 50];
        let a = summarize_scores(&scores, 42, 1000).unwrap();
        let b = summarize_scores(&scores, 42, 1000).unwrap();
        assert_eq!(a, b);
        assert_eq!(a.mean, 30.0);
        assert_eq!(a.median, 30.0);
        assert_eq!(a.max, 50);
        assert!(a.mean_ci_95.0 <= a.mean && a.mean <= a.mean_ci_95.1);
    }

    #[test]
    fn nonparametric_test_and_holm_adjustment_are_sensible() {
        let low = [1, 2, 3, 4, 5, 6, 7, 8];
        let high = [20, 21, 22, 23, 24, 25, 26, 27];
        assert!(mann_whitney_u_pvalue(&low, &high).unwrap() < 0.01);
        let adjusted = holm_adjust(&[0.01, 0.04, 0.03]);
        assert!(adjusted.iter().all(|p| *p <= 1.0));
        assert_eq!(adjusted[0], 0.03);
        assert_eq!(adjusted[1], 0.06);
        assert_eq!(adjusted[2], 0.06);
        assert!(cohens_d(&low, &high).unwrap() < 0.0);
        assert_eq!(
            paired_sign_test_pvalue(&[1, 3, 2, 4], &[2, 2, 3, 5]),
            Some(0.625)
        );
    }
}
