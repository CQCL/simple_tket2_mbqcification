use std::collections::HashSet;

use hugr::{Hugr, HugrView};
use tket2::{portmatching::{CircuitPattern, PatternMatcher}, rewrite::CircuitRewrite};
use urlencoding;
use webbrowser;

// Adapted from tket2/src/utils.rs
pub fn viz_hugr(hugr: &impl HugrView) {
    let mut base: String = "https://dreampuf.github.io/GraphvizOnline/#".into();
    base.push_str(&urlencoding::encode(hugr.dot_string().as_ref()));
    webbrowser::open(&base).unwrap();
}

/// Apply all of the rewrite rules on `circ` until no more can be applied.
///
/// Each rule is specified as a tuple (LHS, RHS). It proceeds iteratively,
/// finding all matches of the LHS on the current circuit, replacing each of
/// those with the corresponding RHS and then starting another round of
/// pattern matching. It stops when no more matches are found.
///
/// Rules are applied in arbitrary order, so the user should guarantee
/// confluence of the rewrite, or otherwise be aware that the result may not
/// be deterministic.
pub fn apply_rules_exhaustively(
    rules: Vec<(Hugr, Hugr)>,
    circ: &mut Hugr,
) {
    // Translate the LHS from HUGRs to patterns
    let mut lhs_of_rules = vec![];
    for (lhs, _) in rules.iter() {
        lhs_of_rules.push(
            CircuitPattern::try_from_circuit(&lhs).unwrap()
        );
    }

    // Create the pattern matcher
    let matcher = PatternMatcher::from_patterns(lhs_of_rules);
    // Find all matches in the current circuit
    let mut matches = matcher.find_matches(circ);

    // Apply rewrites and look for matches repeatedly until there are no more
    while matches.len() > 0 {
        // Convert each match to a rewrite with its corresponding RHS
        let mut rewrites = vec![];
        for m in matches {
            // Identify which of the rules was matched in this case
            let rule_id = m.pattern_id().0;  // The .0 is needed to extract the usize from a PatternID
            let rhs = &rules[rule_id].1;  // The .1 is used here to access the second element of the tuple
            // Convert to rewrite and add to the list of rewrites to be applied
            let rw = m.to_rewrite(circ, rhs.clone()).unwrap();
            rewrites.push(rw);
        };

        // Apply all of non-overlapping rewrites
        apply_non_overlapping(rewrites, circ);
        // Find the next set of matches
        matches = matcher.find_matches(circ);
    };
}

/// A rewrite strategy applying as many non-overlapping rewrites as possible.
///
/// The order at which the rewrites are applied is arbitrary. If a rewrite 
/// overlaps with a rewrite that has already been applied, it is skipped.
///
/// This strategy will always return exactly one circuit: the original circuit
/// with as many rewrites applied as possible.
///
/// This code was adapted from tket2::strategy::GreedyRewriteStrategy
fn apply_non_overlapping(
    rewrites: impl IntoIterator<Item = CircuitRewrite>,
    circ: &mut Hugr,
) {
    let rewrites = rewrites.into_iter();
    let mut changed_nodes = HashSet::new();
    for rewrite in rewrites {
        if rewrite  // Skip if it changes a node that has already been changed
            .subcircuit()
            .nodes()
            .iter()
            .any(|n| changed_nodes.contains(n))
        {
            continue;
        }
        // Update the set of changed nodes
        changed_nodes.extend(rewrite.subcircuit().nodes().iter().copied());
        // Apply the rewrite
        rewrite
            .apply(circ)
            .expect("Could not perform rewrite in exhaustive strategy");
    }
}