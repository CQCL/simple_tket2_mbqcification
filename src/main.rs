use hugr::{
    builder::{BuildError, DFGBuilder, Dataflow, DataflowHugr}, 
    extension::{
        declarative::load_extensions, prelude::QB_T, PRELUDE_REGISTRY
    }, 
    types::FunctionType, Hugr, HugrView
};
use tket2::Tk2Op;
use urlencoding;
use webbrowser;

mod patterns;
mod rewrites;
use crate::rewrites::{
    to_mbqc,
    push_s_gates,
    cancel_s_gates,
};


// Copied from Dan's https://github.com/daniel-mills-cqc/tket2-pec-rust
fn viz_hugr(hugr: &impl HugrView) {
    let mut base: String = "https://dreampuf.github.io/GraphvizOnline/#".into();
    base.push_str(&urlencoding::encode(hugr.dot_string().as_ref()));
    webbrowser::open(&base).unwrap();
}

fn circ_example() -> Result<Hugr, BuildError> {
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T; 4], vec![QB_T; 4]))?;

    let mut inps = h.input_wires();
    let q0 = inps.next().unwrap();
    let q1 = inps.next().unwrap();
    let q2 = inps.next().unwrap();
    let q3 = inps.next().unwrap();

    let res = h.add_dataflow_op(Tk2Op::CZ, [q2, q3])?;
    let q2 = res.out_wire(0);
    let q3 = res.out_wire(1);
    let res = h.add_dataflow_op(Tk2Op::H, [q0])?;
    let q0 = res.out_wire(0);
    let res = h.add_dataflow_op(Tk2Op::S, [q1])?;
    let q1 = res.out_wire(0);
    let res = h.add_dataflow_op(Tk2Op::S, [q2])?;
    let q2 = res.out_wire(0);
    let res = h.add_dataflow_op(Tk2Op::CZ, [q1, q2])?;
    let q1 = res.out_wire(0);
    let q2 = res.out_wire(1);
    let res = h.add_dataflow_op(Tk2Op::S, [q3])?;
    let q3 = res.out_wire(0);
    let res = h.add_dataflow_op(Tk2Op::H, [q2])?;
    let q2 = res.out_wire(0);
    let res = h.add_dataflow_op(Tk2Op::S, [q2])?;
    let q2 = res.out_wire(0);
    let res = h.add_dataflow_op(Tk2Op::H, [q1])?;
    let q1 = res.out_wire(0);
    let res = h.add_dataflow_op(Tk2Op::S, [q0])?;
    let q0 = res.out_wire(0);
    let res = h.add_dataflow_op(Tk2Op::CZ, [q0, q3])?;
    let q0 = res.out_wire(0);
    let q3 = res.out_wire(1);
    let res = h.add_dataflow_op(Tk2Op::S, [q0])?;
    let q0 = res.out_wire(0);
    
    h.finish_hugr_with_outputs([q0, q1, q2, q3], &PRELUDE_REGISTRY)
}

fn main() {
    // Load the custom extension
    const DECLARATIVE_YAML: &str = include_str!("mbqc_ops.yaml");
    let mut reg = PRELUDE_REGISTRY.clone();
    load_extensions(DECLARATIVE_YAML, &mut reg).unwrap();

    let mut circ = circ_example().unwrap();
    viz_hugr(&circ);

    // Step 1: Convert each H gate to MBQC pattern
    to_mbqc(&mut circ, &reg);

    // Step 2: Push all corrections and S gates to the end of the qubit wire
    push_s_gates(&mut circ);
    cancel_s_gates(&mut circ);
    viz_hugr(&circ);

    // Step 3: Remove all corrections from ancilla qubits, propagating them to the boolean expression for the correction on output qubits

    // Step 4: Convert the MBQC pattern to a circuit using n qubits

    // Step 5: Apply some basic depth reduction strategies

    // Step 6: Replace each operation from the ExtMBQC extension with its implementation in terms of Tk2Ops
    
}
