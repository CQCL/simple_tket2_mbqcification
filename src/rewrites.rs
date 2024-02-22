use std::collections::HashSet;

use hugr::{Hugr, extension::ExtensionRegistry};
use itertools::Itertools;
use tket2::{portmatching::{CircuitPattern, PatternMatcher}, rewrite::CircuitRewrite};
use portmatching::PatternID;

use crate::patterns;

/// A rewrite strategy applying as many non-overlapping rewrites as possible.
///
/// The order at which the rewrites are applied is arbitrary. If a rewrite 
/// overlaps with a rewrite that has already been applied, it is skipped.
///
/// This strategy will always return exactly one circuit: the original circuit
/// with as many rewrites applied as possible.
fn apply_rewrites_exhaustively(
    rewrites: impl IntoIterator<Item = CircuitRewrite>,
    circ: &mut Hugr,
) {
    /* NOTE: This code was adapted from tket2::strategy::GreedyRewriteStrategy */
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


pub fn to_mbqc(circ: &mut Hugr, registry: &ExtensionRegistry) {
    let h_gate = patterns::h().unwrap();
    let h_replacement = patterns::mbqc_h(registry).unwrap();

    let p = CircuitPattern::try_from_circuit(&h_gate).unwrap();
    let m = PatternMatcher::from_patterns(vec![p]);

    let matches = m.find_matches(circ);
    for matched in matches {
        matched.to_rewrite(circ, h_replacement.clone())
            .unwrap()
            .apply(circ)
            .unwrap();
    }
}

pub fn push_s_gates(circ: &mut Hugr) {

    // Instantiate the two LHS patterns
    let rule_0_pattern = CircuitPattern::try_from_circuit(
        &patterns::s_cz_0().unwrap()
    ).unwrap();
    let rule_1_pattern = CircuitPattern::try_from_circuit(
        &patterns::s_cz_1().unwrap()
    ).unwrap();
    let lhs_of_rules = vec![rule_0_pattern, rule_1_pattern];

    // Instantiate the RHS for the rules
    let rule_0_replacement = patterns::cz_s_0().unwrap();
    let rule_1_replacement = patterns::cz_s_1().unwrap();

    // Create the pattern matcher and find the first matches
    let m = PatternMatcher::from_patterns(lhs_of_rules);
    let mut matches = m.find_matches(circ);
    
    // Apply all of the rewrites exhaustively
    while matches.len() > 0 {
        let mut this_rewrites = vec![];

        for matched in matches {
            // Identify which of the rules was matched in this case
            let this_replacement = match matched.pattern_id() {
                PatternID(0) => Ok(rule_0_replacement.clone()),
                PatternID(1) => Ok(rule_1_replacement.clone()),
                _ => Err("Can't happen!"),
            }.unwrap();

            // Create the rewrite instance, and batch it with the rest
            let rw = matched.to_rewrite(circ, this_replacement).unwrap();
            this_rewrites.push(rw);
        }

        // Apply the batched rewrites using an exhaustive strategy
        apply_rewrites_exhaustively(this_rewrites, circ);

        // Find the next set of matches
        matches = m.find_matches(circ);
    }
}

pub fn cancel_s_gates(circ: &mut Hugr) {

    // Instantiate the two LHS patterns
    let lhs = CircuitPattern::try_from_circuit(
        &patterns::s_s().unwrap()
    ).unwrap();

    // Instantiate the RHS for the rules
    let rhs = patterns::id().unwrap();

    // Create the pattern matcher and find the first matches
    let m = PatternMatcher::from_patterns(vec![lhs]);
    let mut matches = m.find_matches(circ);
    
    // Apply all of the rewrites exhaustively
    while matches.len() > 0 {
        // Convert all matches to rewrites
        let this_rewrites = matches
            .iter()
            .map({
                |matched| matched.to_rewrite(circ, rhs.clone()).unwrap()
            })
            .collect_vec();

        // Apply the rewrites using an exhaustive strategy
        apply_rewrites_exhaustively(this_rewrites, circ);

        // Find the next set of matches
        matches = m.find_matches(circ);
    }
}

pub fn push_corrections(circ: &mut Hugr, registry: &ExtensionRegistry) {
    // Instantiate the LHS patterns
    let mut lhs_of_rules = vec![];
    lhs_of_rules.push(CircuitPattern::try_from_circuit(
        &patterns::xcorr_h(&registry).unwrap()
    ).unwrap());
    lhs_of_rules.push(CircuitPattern::try_from_circuit(
        &patterns::zcorr_h(&registry).unwrap()
    ).unwrap());
    lhs_of_rules.push(CircuitPattern::try_from_circuit(
        &patterns::xicorr_cz(&registry).unwrap()
    ).unwrap());
    lhs_of_rules.push(CircuitPattern::try_from_circuit(
        &patterns::ixcorr_cz(&registry).unwrap()
    ).unwrap());
    lhs_of_rules.push(CircuitPattern::try_from_circuit(
        &patterns::zicorr_cz(&registry).unwrap()
    ).unwrap());
    lhs_of_rules.push(CircuitPattern::try_from_circuit(
        &patterns::izcorr_cz(&registry).unwrap()
    ).unwrap());
    lhs_of_rules.push(CircuitPattern::try_from_circuit(
        &patterns::xcorr_s(&registry).unwrap()
    ).unwrap());
    lhs_of_rules.push(CircuitPattern::try_from_circuit(
        &patterns::zcorr_s(&registry).unwrap()
    ).unwrap());

    // Create the pattern matcher and find the first matches
    let m = PatternMatcher::from_patterns(lhs_of_rules);
    let mut matches = m.find_matches(circ);
    
    // Apply all of the rewrites exhaustively
    while matches.len() > 0 {
        let mut this_rewrites = vec![];

        for matched in matches {
            // Identify which of the rules was matched in this case
            let this_replacement = match matched.pattern_id() {
                PatternID(0) => Ok(patterns::h_zcorr(&registry).unwrap()),
                PatternID(1) => Ok(patterns::h_xcorr(&registry).unwrap()),
                PatternID(2) => Ok(patterns::cz_xzcorr(&registry).unwrap()),
                PatternID(3) => Ok(patterns::cz_zxcorr(&registry).unwrap()),
                PatternID(4) => Ok(patterns::cz_zicorr(&registry).unwrap()),
                PatternID(5) => Ok(patterns::cz_izcorr(&registry).unwrap()),
                PatternID(6) => Ok(patterns::s_xcorr_zcorr(&registry).unwrap()),
                PatternID(7) => Ok(patterns::s_zcorr(&registry).unwrap()),
                _ => Err("Can't happen!"),
            }.unwrap();

            // Create the rewrite instance, and batch it with the rest
            let rw = matched.to_rewrite(circ, this_replacement).unwrap();
            this_rewrites.push(rw);
        }

        // Apply the batched rewrites using an exhaustive strategy
        apply_rewrites_exhaustively(this_rewrites, circ);

        // Find the next set of matches
        matches = m.find_matches(circ);
    }
}



pub fn prep_to_alloc(circ: &mut Hugr, registry: &ExtensionRegistry) {
    let lhs = patterns::prep(registry).unwrap();
    let rhs = patterns::alloc_reset_h().unwrap();

    let p = CircuitPattern::try_from_circuit(&lhs).unwrap();
    let m = PatternMatcher::from_patterns(vec![p]);

    let matches = m.find_matches(circ);
    for matched in matches {
        matched.to_rewrite(circ, rhs.clone())
            .unwrap()
            .apply(circ)
            .unwrap();
    }
}