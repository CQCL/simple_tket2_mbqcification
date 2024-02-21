use hugr::{Hugr, extension::ExtensionRegistry};
use tket2::portmatching::{CircuitPattern, PatternMatcher};

use crate::patterns::{h, mbqc_h};


pub fn to_mbqc(circ: &mut Hugr, registry: &ExtensionRegistry) {
    let h_gate = h().unwrap();
    let h_replacement = mbqc_h(registry).unwrap();

    let p = CircuitPattern::try_from_circuit(&h_gate).unwrap();
    let m = PatternMatcher::from_patterns(vec![p]);

    let matches = m.find_matches(circ);
    for matched in matches {
        let rewrite = matched.to_rewrite(circ, h_replacement.clone()).unwrap();
        rewrite.apply(circ).unwrap();
    }
}