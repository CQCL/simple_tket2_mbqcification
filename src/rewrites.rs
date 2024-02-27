use hugr::{Hugr, extension::ExtensionRegistry};

use crate::patterns::*;
use crate::utils::apply_rules_exhaustively;


pub fn to_mbqc(circ: &mut Hugr, reg: &ExtensionRegistry) {
    // Specify the rewrite rules
    let rules = vec![
        (h(), mbqc_h(reg)),
    ]
    // Unwrap all of the above `Result<Hugr, BuildError>` types into `Hugr`
    .iter()
    .map(|rule| (rule.0.clone().unwrap(), rule.1.clone().unwrap()))
    .collect();
    // Apply them exhaustively
    apply_rules_exhaustively(rules, circ);
}

pub fn push_corrections_and_s_gates(circ: &mut Hugr, reg: &ExtensionRegistry) {
    // Specify the rewrite rules
    let rules = vec![
        // Push corrections
        (xcorr_h(&reg), h_zcorr(&reg)),
        (zcorr_h(&reg), h_xcorr(&reg)),
        (xicorr_cz(&reg), cz_xzcorr(&reg)),
        (ixcorr_cz(&reg), cz_zxcorr(&reg)),
        (zicorr_cz(&reg), cz_zicorr(&reg)),
        (izcorr_cz(&reg), cz_izcorr(&reg)),
        (xcorr_s(&reg), s_xcorr_zcorr(&reg)),
        (zcorr_s(&reg), s_zcorr(&reg)),
        // Push S gates
        (s_cz_0(), cz_s_0()),
        (s_cz_1(), cz_s_1()),
    ]
    // Unwrap all of the above `Result<Hugr, BuildError>` types into `Hugr`
    .iter()
    .map(|rule| (rule.0.clone().unwrap(), rule.1.clone().unwrap()))
    .collect();
    // Apply them exhaustively
    apply_rules_exhaustively(rules, circ);
}

pub fn propagate_corrections(circ: &mut Hugr, reg: &ExtensionRegistry) {
    // Specify the rewrite rules
    let rules = vec![
        // X corrections before an X measurement only contribute to a global phase, so we remove them
        (xcorr_xmeas(&reg), xmeas_discard_input_signal(&reg)),
        // Z corrections before an X measurement can be propagated to the classical signal
        (zcorr_xmeas(&reg), xmeas_xor(&reg)),
        // Since the first rule introduced `DiscardSignal` nodes, we may remove some `Copy` and `XOR` nodes
        (copy_discard_0(&reg), classical_wire(&reg)),
        (copy_discard_1(&reg), classical_wire(&reg)),
        (xor_discard(&reg), discard_both(&reg)),
        // Merge Z corrections together
        (xcorr_xcorr(&reg), xor_xcorr(&reg)),
        // Merge X corrections together
        (zcorr_zcorr(&reg), xor_zcorr(&reg)),
        // Make sure that Z corrections appear after X corrections, so that merging can be maximised
        (zcorr_xcorr(&reg), xcorr_zcorr(&reg)),
    ]
    // Unwrap all of the above `Result<Hugr, BuildError>` types into `Hugr`
    .iter()
    .map(|rule| (rule.0.clone().unwrap(), rule.1.clone().unwrap()))
    .collect();
    // Apply them exhaustively
    apply_rules_exhaustively(rules, circ);
}



pub fn prep_to_alloc(circ: &mut Hugr, reg: &ExtensionRegistry) {
    // Specify the rewrite rules
    let rules = vec![
        (prep(reg), alloc_reset_h()),
    ]
    // Unwrap all of the above `Result<Hugr, BuildError>` types into `Hugr`
    .iter()
    .map(|rule| (rule.0.clone().unwrap(), rule.1.clone().unwrap()))
    .collect();
    // Apply them exhaustively
    apply_rules_exhaustively(rules, circ);
}