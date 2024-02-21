use hugr::{
    builder::{BuildError, DFGBuilder, Dataflow, DataflowHugr},
    extension::{
        prelude::QB_T, ExtensionRegistry, PRELUDE_REGISTRY
    },
    types::FunctionType, Hugr
};
use tket2::Tk2Op;


pub fn h() -> Result<Hugr, BuildError> {
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T], vec![QB_T]))?;

    let mut inps = h.input_wires();
    let q = inps.next().unwrap();

    let res = h.add_dataflow_op(Tk2Op::H, [q])?;
    let q = res.out_wire(0);
    
    h.finish_hugr_with_outputs([q], &PRELUDE_REGISTRY)
}

pub fn mbqc_h(registry: &ExtensionRegistry) -> Result<Hugr, BuildError> {

    // Load the extension
    let extension = registry.get("ExtMBQC").unwrap();
    let prepare_op = extension.instantiate_extension_op("PrepPlus", [], registry).unwrap();
    let measure_op = extension.instantiate_extension_op("MyMeasure", [], registry).unwrap();
    let x_corr = extension.instantiate_extension_op("CorrectionX", [], registry).unwrap();

    // Build the HUGR
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T], vec![QB_T]))?;
    let mut inps = h.input_wires();
    let q_in = inps.next().unwrap();

    let res = h.add_dataflow_op(prepare_op, [])?;
    let q_out = res.out_wire(0);
    let res = h.add_dataflow_op(Tk2Op::CZ, [q_in, q_out])?;
    let q_in = res.out_wire(0);
    let q_out = res.out_wire(1);
    let res = h.add_dataflow_op(measure_op, [q_in])?;
    let q_in = res.out_wire(0);
    let c_out = res.out_wire(1);
    h.add_dataflow_op(Tk2Op::QFree, [q_in])?;
    let res = h.add_dataflow_op(x_corr, [q_out, c_out])?;
    let q_out = res.out_wire(0);
    
    // let bool_rows = [type_row![], type_row![]];
    // let mut cond_node = h.conditional_builder(
    //     (bool_rows, c_out),
    //     [(QB_T, q_out)], 
    //     type_row![QB_T], 
    //     ExtensionSet::new()
    // )?;

    // let false_case = cond_node.case_builder(0)?;
    // let mut inps = false_case.input_wires();
    // let q_out = inps.next().unwrap();
    // false_case.finish_with_outputs([q_out])?;

    // let mut true_case = cond_node.case_builder(1)?;
    // let mut inps = true_case.input_wires();
    // let q_out = inps.next().unwrap();
    // let res = true_case.add_dataflow_op(Tk2Op::X, [q_out]).unwrap();
    // let q_out = res.out_wire(0);
    // true_case.finish_with_outputs([q_out])?;

    // let res = cond_node.finish_sub_container()?;
    // let q_out = res.out_wire(0);

    // viz_hugr(h.hugr());
    h.finish_hugr_with_outputs([q_out], &registry)
}