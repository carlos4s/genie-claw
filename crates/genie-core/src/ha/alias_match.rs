pub(crate) fn best_alias_score(query_words: &[&str], query_lower: &str, aliases: &[String]) -> f32 {
    aliases
        .iter()
        .map(|alias| fuzzy_score(query_words, query_lower, alias))
        .fold(0.0, f32::max)
}

pub(crate) fn fuzzy_score(query_words: &[&str], query_lower: &str, candidate: &str) -> f32 {
    let candidate_words: Vec<&str> = candidate.split_whitespace().collect();
    let mut score = 0.0;

    if candidate == query_lower {
        score += 1.0;
    } else if candidate.contains(query_lower) || query_lower.contains(candidate) {
        score += 0.75;
    }

    if !query_words.is_empty() {
        let matching = query_words
            .iter()
            .filter(|query_word| {
                candidate_words.iter().any(|candidate_word| {
                    candidate_word.contains(*query_word) || query_word.contains(candidate_word)
                })
            })
            .count();
        score += (matching as f32 / query_words.len() as f32) * 0.35;
    }

    if candidate.starts_with(query_lower) {
        score += 0.15;
    }

    score.min(1.0)
}

/// Outcome of selecting a single best candidate from a list of scores.
pub(crate) enum Match {
    /// Exactly one candidate holds the top score above the threshold.
    Unique(usize),
    /// Two or more distinct candidates tie for the top score; the caller must
    /// disambiguate rather than pick one arbitrarily.
    Ambiguous(Vec<usize>),
    /// No candidate cleared the threshold.
    None,
}

/// Pick the single highest-scoring candidate whose score exceeds `threshold`.
///
/// The previous resolver kept the first candidate at the maximum score with a
/// strict `>` comparison, so when several distinct devices tied (shared domain
/// synonyms make a bare "lamp" score 1.0 against every light) the first in graph
/// order silently won and an arbitrary device was actuated. Returning
/// [`Match::Ambiguous`] on a tie lets the caller decline and disambiguate.
/// Scores within `EPSILON` of the maximum count as tied.
pub(crate) fn select_unique(scores: &[f32], threshold: f32) -> Match {
    const EPSILON: f32 = 1e-4;

    let best = scores
        .iter()
        .copied()
        .filter(|score| *score > threshold)
        .fold(None, |acc: Option<f32>, score| {
            Some(acc.map_or(score, |current| current.max(score)))
        });
    let Some(best) = best else {
        return Match::None;
    };

    let tied: Vec<usize> = scores
        .iter()
        .enumerate()
        .filter(|(_, score)| **score > threshold && (best - **score).abs() <= EPSILON)
        .map(|(index, _)| index)
        .collect();

    if tied.len() == 1 {
        Match::Unique(tied[0])
    } else {
        Match::Ambiguous(tied)
    }
}
