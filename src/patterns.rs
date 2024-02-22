use hugr::{
    builder::{BuildError, Container, DFGBuilder, Dataflow, DataflowHugr},
    extension::{
        prelude::QB_T, ExtensionRegistry, PRELUDE_REGISTRY
    },
    types::{FunctionType, Type}, Hugr, HugrView
};
use tket2::Tk2Op;
use crate::utils::viz_hugr;


pub fn h() -> Result<Hugr, BuildError> {
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T], vec![QB_T]))?;

    let mut inps = h.input_wires();
    let q = inps.next().unwrap();

    let res = h.add_dataflow_op(Tk2Op::H, [q])?;
    let q = res.out_wire(0);
    
    h.finish_hugr_with_outputs([q], &PRELUDE_REGISTRY)
}

pub fn prep(registry: &ExtensionRegistry) -> Result<Hugr, BuildError> {
    // Load the extension
    let extension = registry.get("ExtMBQC").unwrap();
    let prep = extension.instantiate_extension_op("PrepPlus", [], registry).unwrap();

    // Build the HUGR
    let mut h = DFGBuilder::new(FunctionType::new(vec![], vec![QB_T]))?;
    let res = h.add_dataflow_op(prep, [])?;
    let q = res.out_wire(0);
    
    h.finish_hugr_with_outputs([q], registry)
}

pub fn mbqc_h(registry: &ExtensionRegistry) -> Result<Hugr, BuildError> {
    // Load the extension
    let extension = registry.get("ExtMBQC").unwrap();
    let prepare_op = extension.instantiate_extension_op("PrepPlus", [], registry).unwrap();
    let measure_op = extension.instantiate_extension_op("MeasureX", [], registry).unwrap();
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
    h.finish_hugr_with_outputs([q_out], registry)
}

pub fn s_cz_0() -> Result<Hugr, BuildError> {
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T, QB_T], vec![QB_T, QB_T]))?;

    let mut inps = h.input_wires();
    let q0 = inps.next().unwrap();
    let q1 = inps.next().unwrap();

    let res = h.add_dataflow_op(Tk2Op::S, [q0])?;
    let q0 = res.out_wire(0);
    let res = h.add_dataflow_op(Tk2Op::CZ, [q0, q1])?;
    let q0 = res.out_wire(0);
    let q1 = res.out_wire(1);

    h.finish_hugr_with_outputs([q0, q1], &PRELUDE_REGISTRY)   
}

pub fn s_cz_1() -> Result<Hugr, BuildError> {
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T, QB_T], vec![QB_T, QB_T]))?;

    let mut inps = h.input_wires();
    let q0 = inps.next().unwrap();
    let q1 = inps.next().unwrap();

    let res = h.add_dataflow_op(Tk2Op::S, [q1])?;
    let q1 = res.out_wire(0);
    let res = h.add_dataflow_op(Tk2Op::CZ, [q0, q1])?;
    let q0 = res.out_wire(0);
    let q1 = res.out_wire(1);

    h.finish_hugr_with_outputs([q0, q1], &PRELUDE_REGISTRY)   
}

pub fn cz_s_0() -> Result<Hugr, BuildError> {
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T, QB_T], vec![QB_T, QB_T]))?;

    let mut inps = h.input_wires();
    let q0 = inps.next().unwrap();
    let q1 = inps.next().unwrap();

    let res = h.add_dataflow_op(Tk2Op::CZ, [q0, q1])?;
    let q0 = res.out_wire(0);
    let q1 = res.out_wire(1);
    let res = h.add_dataflow_op(Tk2Op::S, [q0])?;
    let q0 = res.out_wire(0);

    h.finish_hugr_with_outputs([q0, q1], &PRELUDE_REGISTRY)   
}

pub fn cz_s_1() -> Result<Hugr, BuildError> {
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T, QB_T], vec![QB_T, QB_T]))?;

    let mut inps = h.input_wires();
    let q0 = inps.next().unwrap();
    let q1 = inps.next().unwrap();

    let res = h.add_dataflow_op(Tk2Op::CZ, [q0, q1])?;
    let q0 = res.out_wire(0);
    let q1 = res.out_wire(1);
    let res = h.add_dataflow_op(Tk2Op::S, [q1])?;
    let q1 = res.out_wire(0);

    h.finish_hugr_with_outputs([q0, q1], &PRELUDE_REGISTRY)   
}

pub fn s_s() -> Result<Hugr, BuildError> {
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T], vec![QB_T]))?;

    let mut inps = h.input_wires();
    let q = inps.next().unwrap();

    let res = h.add_dataflow_op(Tk2Op::S, [q])?;
    let q = res.out_wire(0);
    let res = h.add_dataflow_op(Tk2Op::S, [q])?;
    let q = res.out_wire(0);

    h.finish_hugr_with_outputs([q], &PRELUDE_REGISTRY)   
}

pub fn id() -> Result<Hugr, BuildError> {
    let h = DFGBuilder::new(FunctionType::new(vec![QB_T], vec![QB_T]))?;

    let mut inps = h.input_wires();
    let q = inps.next().unwrap();
    h.finish_hugr_with_outputs([q], &PRELUDE_REGISTRY)   
}

pub fn xcorr_h(registry: &ExtensionRegistry) -> Result<Hugr, BuildError> {
    // Load the extension
    let extension = registry.get("ExtMBQC").unwrap();
    let my_bool = Type::new_extension(extension.get_type("MyBool").unwrap().instantiate([]).unwrap());
    let x_corr = extension.instantiate_extension_op("CorrectionX", [], registry).unwrap();

    // Build the HUGR
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T, my_bool], vec![QB_T]))?;
    let mut inps = h.input_wires();
    let q = inps.next().unwrap();
    let c = inps.next().unwrap();

    let res = h.add_dataflow_op(x_corr, [q, c])?;
    let q = res.out_wire(0);
    let res = h.add_dataflow_op(Tk2Op::H, [q])?;
    let q = res.out_wire(0);
    
    h.finish_hugr_with_outputs([q], registry)
}

pub fn h_zcorr(registry: &ExtensionRegistry) -> Result<Hugr, BuildError> {
    // Load the extension
    let extension = registry.get("ExtMBQC").unwrap();
    let my_bool = Type::new_extension(extension.get_type("MyBool").unwrap().instantiate([]).unwrap());
    let z_corr = extension.instantiate_extension_op("CorrectionZ", [], registry).unwrap();

    // Build the HUGR
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T, my_bool], vec![QB_T]))?;
    let mut inps = h.input_wires();
    let q = inps.next().unwrap();
    let c = inps.next().unwrap();

    let res = h.add_dataflow_op(Tk2Op::H, [q])?;
    let q = res.out_wire(0);
    let res = h.add_dataflow_op(z_corr, [q, c])?;
    let q = res.out_wire(0);
    
    h.finish_hugr_with_outputs([q], registry)
}

pub fn zcorr_h(registry: &ExtensionRegistry) -> Result<Hugr, BuildError> {
    // Load the extension
    let extension = registry.get("ExtMBQC").unwrap();
    let my_bool = Type::new_extension(extension.get_type("MyBool").unwrap().instantiate([]).unwrap());
    let z_corr = extension.instantiate_extension_op("CorrectionZ", [], registry).unwrap();

    // Build the HUGR
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T, my_bool], vec![QB_T]))?;
    let mut inps = h.input_wires();
    let q = inps.next().unwrap();
    let c = inps.next().unwrap();

    let res = h.add_dataflow_op(z_corr, [q, c])?;
    let q = res.out_wire(0);
    let res = h.add_dataflow_op(Tk2Op::H, [q])?;
    let q = res.out_wire(0);
    
    h.finish_hugr_with_outputs([q], registry)
}

pub fn h_xcorr(registry: &ExtensionRegistry) -> Result<Hugr, BuildError> {
    // Load the extension
    let extension = registry.get("ExtMBQC").unwrap();
    let my_bool = Type::new_extension(extension.get_type("MyBool").unwrap().instantiate([]).unwrap());
    let x_corr = extension.instantiate_extension_op("CorrectionX", [], registry).unwrap();

    // Build the HUGR
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T, my_bool], vec![QB_T]))?;
    let mut inps = h.input_wires();
    let q = inps.next().unwrap();
    let c = inps.next().unwrap();

    let res = h.add_dataflow_op(Tk2Op::H, [q])?;
    let q = res.out_wire(0);
    let res = h.add_dataflow_op(x_corr, [q, c])?;
    let q = res.out_wire(0);
    
    h.finish_hugr_with_outputs([q], registry)
}

pub fn xcorr_s(registry: &ExtensionRegistry) -> Result<Hugr, BuildError> {
    // Load the extension
    let extension = registry.get("ExtMBQC").unwrap();
    let my_bool = Type::new_extension(extension.get_type("MyBool").unwrap().instantiate([]).unwrap());
    let x_corr = extension.instantiate_extension_op("CorrectionX", [], registry).unwrap();

    // Build the HUGR
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T, my_bool], vec![QB_T]))?;
    let mut inps = h.input_wires();
    let q = inps.next().unwrap();
    let c = inps.next().unwrap();

    let res = h.add_dataflow_op(x_corr, [q, c])?;
    let q = res.out_wire(0);
    let res = h.add_dataflow_op(Tk2Op::S, [q])?;
    let q = res.out_wire(0);
    
    h.finish_hugr_with_outputs([q], registry)
}

pub fn s_xcorr_zcorr(registry: &ExtensionRegistry) -> Result<Hugr, BuildError> {
    // Load the extension
    let extension = registry.get("ExtMBQC").unwrap();
    let my_bool = Type::new_extension(extension.get_type("MyBool").unwrap().instantiate([]).unwrap());
    let copy = extension.instantiate_extension_op("Copy", [], registry).unwrap();
    let x_corr = extension.instantiate_extension_op("CorrectionX", [], registry).unwrap();
    let z_corr = extension.instantiate_extension_op("CorrectionZ", [], registry).unwrap();

    // Build the HUGR
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T, my_bool], vec![QB_T]))?;
    let mut inps = h.input_wires();
    let q = inps.next().unwrap();
    let c = inps.next().unwrap();

    let res = h.add_dataflow_op(Tk2Op::S, [q])?;
    let q = res.out_wire(0);
    let res = h.add_dataflow_op(copy, [c])?;
    let c_x = res.out_wire(0);
    let c_z = res.out_wire(1);
    let res = h.add_dataflow_op(x_corr, [q, c_x])?;
    let q = res.out_wire(0);
    let res = h.add_dataflow_op(z_corr, [q, c_z])?;
    let q = res.out_wire(0);
    
    h.finish_hugr_with_outputs([q], registry)
}

pub fn zcorr_s(registry: &ExtensionRegistry) -> Result<Hugr, BuildError> {
    // Load the extension
    let extension = registry.get("ExtMBQC").unwrap();
    let my_bool = Type::new_extension(extension.get_type("MyBool").unwrap().instantiate([]).unwrap());
    let z_corr = extension.instantiate_extension_op("CorrectionZ", [], registry).unwrap();

    // Build the HUGR
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T, my_bool], vec![QB_T]))?;
    let mut inps = h.input_wires();
    let q = inps.next().unwrap();
    let c = inps.next().unwrap();

    let res = h.add_dataflow_op(z_corr, [q, c])?;
    let q = res.out_wire(0);
    let res = h.add_dataflow_op(Tk2Op::S, [q])?;
    let q = res.out_wire(0);
    
    h.finish_hugr_with_outputs([q], registry)
}

pub fn s_zcorr(registry: &ExtensionRegistry) -> Result<Hugr, BuildError> {
    // Load the extension
    let extension = registry.get("ExtMBQC").unwrap();
    let my_bool = Type::new_extension(extension.get_type("MyBool").unwrap().instantiate([]).unwrap());
    let z_corr = extension.instantiate_extension_op("CorrectionZ", [], registry).unwrap();

    // Build the HUGR
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T, my_bool], vec![QB_T]))?;
    let mut inps = h.input_wires();
    let q = inps.next().unwrap();
    let c = inps.next().unwrap();

    let res = h.add_dataflow_op(Tk2Op::S, [q])?;
    let q = res.out_wire(0);
    let res = h.add_dataflow_op(z_corr, [q, c])?;
    let q = res.out_wire(0);
    
    h.finish_hugr_with_outputs([q], registry)
}

pub fn xicorr_cz(registry: &ExtensionRegistry) -> Result<Hugr, BuildError> {
    // Load the extension
    let extension = registry.get("ExtMBQC").unwrap();
    let my_bool = Type::new_extension(extension.get_type("MyBool").unwrap().instantiate([]).unwrap());
    let x_corr = extension.instantiate_extension_op("CorrectionX", [], registry).unwrap();

    // Build the HUGR
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T, QB_T, my_bool], vec![QB_T, QB_T]))?;
    let mut inps = h.input_wires();
    let q0 = inps.next().unwrap();
    let q1 = inps.next().unwrap();
    let c = inps.next().unwrap();

    let res = h.add_dataflow_op(x_corr, [q0, c])?;
    let q0 = res.out_wire(0);
    let res = h.add_dataflow_op(Tk2Op::CZ, [q0, q1])?;
    let q0 = res.out_wire(0);
    let q1 = res.out_wire(1);
    
    h.finish_hugr_with_outputs([q0, q1], registry)
}

pub fn cz_xzcorr(registry: &ExtensionRegistry) -> Result<Hugr, BuildError> {
    // Load the extension
    let extension = registry.get("ExtMBQC").unwrap();
    let my_bool = Type::new_extension(extension.get_type("MyBool").unwrap().instantiate([]).unwrap());
    let copy = extension.instantiate_extension_op("Copy", [], registry).unwrap();
    let x_corr = extension.instantiate_extension_op("CorrectionX", [], registry).unwrap();
    let z_corr = extension.instantiate_extension_op("CorrectionZ", [], registry).unwrap();

    // Build the HUGR
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T, QB_T, my_bool], vec![QB_T, QB_T]))?;
    let mut inps = h.input_wires();
    let q0 = inps.next().unwrap();
    let q1 = inps.next().unwrap();
    let c = inps.next().unwrap();

    let res = h.add_dataflow_op(Tk2Op::CZ, [q0, q1])?;
    let q0 = res.out_wire(0);
    let q1 = res.out_wire(1);
    let res = h.add_dataflow_op(copy, [c])?;
    let c0 = res.out_wire(0);
    let c1 = res.out_wire(1);
    let res = h.add_dataflow_op(x_corr, [q0, c0])?;
    let q0 = res.out_wire(0);
    let res = h.add_dataflow_op(z_corr, [q1, c1])?;
    let q1 = res.out_wire(0);
    
    h.finish_hugr_with_outputs([q0, q1], registry)
}

pub fn ixcorr_cz(registry: &ExtensionRegistry) -> Result<Hugr, BuildError> {
    // Load the extension
    let extension = registry.get("ExtMBQC").unwrap();
    let my_bool = Type::new_extension(extension.get_type("MyBool").unwrap().instantiate([]).unwrap());
    let x_corr = extension.instantiate_extension_op("CorrectionX", [], registry).unwrap();

    // Build the HUGR
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T, QB_T, my_bool], vec![QB_T, QB_T]))?;
    let mut inps = h.input_wires();
    let q0 = inps.next().unwrap();
    let q1 = inps.next().unwrap();
    let c = inps.next().unwrap();

    let res = h.add_dataflow_op(x_corr, [q1, c])?;
    let q1 = res.out_wire(0);
    let res = h.add_dataflow_op(Tk2Op::CZ, [q0, q1])?;
    let q0 = res.out_wire(0);
    let q1 = res.out_wire(1);
    
    h.finish_hugr_with_outputs([q0, q1], registry)
}

pub fn cz_zxcorr(registry: &ExtensionRegistry) -> Result<Hugr, BuildError> {
    // Load the extension
    let extension = registry.get("ExtMBQC").unwrap();
    let my_bool = Type::new_extension(extension.get_type("MyBool").unwrap().instantiate([]).unwrap());
    let copy = extension.instantiate_extension_op("Copy", [], registry).unwrap();
    let x_corr = extension.instantiate_extension_op("CorrectionX", [], registry).unwrap();
    let z_corr = extension.instantiate_extension_op("CorrectionZ", [], registry).unwrap();

    // Build the HUGR
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T, QB_T, my_bool], vec![QB_T, QB_T]))?;
    let mut inps = h.input_wires();
    let q0 = inps.next().unwrap();
    let q1 = inps.next().unwrap();
    let c = inps.next().unwrap();

    let res = h.add_dataflow_op(Tk2Op::CZ, [q0, q1])?;
    let q0 = res.out_wire(0);
    let q1 = res.out_wire(1);
    let res = h.add_dataflow_op(copy, [c])?;
    let c0 = res.out_wire(0);
    let c1 = res.out_wire(1);
    let res = h.add_dataflow_op(z_corr, [q0, c0])?;
    let q0 = res.out_wire(0);
    let res = h.add_dataflow_op(x_corr, [q1, c1])?;
    let q1 = res.out_wire(0);
    
    h.finish_hugr_with_outputs([q0, q1], registry)
}

pub fn zicorr_cz(registry: &ExtensionRegistry) -> Result<Hugr, BuildError> {
    // Load the extension
    let extension = registry.get("ExtMBQC").unwrap();
    let my_bool = Type::new_extension(extension.get_type("MyBool").unwrap().instantiate([]).unwrap());
    let z_corr = extension.instantiate_extension_op("CorrectionZ", [], registry).unwrap();

    // Build the HUGR
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T, QB_T, my_bool], vec![QB_T, QB_T]))?;
    let mut inps = h.input_wires();
    let q0 = inps.next().unwrap();
    let q1 = inps.next().unwrap();
    let c = inps.next().unwrap();

    let res = h.add_dataflow_op(z_corr, [q0, c])?;
    let q0 = res.out_wire(0);
    let res = h.add_dataflow_op(Tk2Op::CZ, [q0, q1])?;
    let q0 = res.out_wire(0);
    let q1 = res.out_wire(1);
    
    h.finish_hugr_with_outputs([q0, q1], registry)
}

pub fn izcorr_cz(registry: &ExtensionRegistry) -> Result<Hugr, BuildError> {
    // Load the extension
    let extension = registry.get("ExtMBQC").unwrap();
    let my_bool = Type::new_extension(extension.get_type("MyBool").unwrap().instantiate([]).unwrap());
    let z_corr = extension.instantiate_extension_op("CorrectionZ", [], registry).unwrap();

    // Build the HUGR
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T, QB_T, my_bool], vec![QB_T, QB_T]))?;
    let mut inps = h.input_wires();
    let q0 = inps.next().unwrap();
    let q1 = inps.next().unwrap();
    let c = inps.next().unwrap();

    let res = h.add_dataflow_op(z_corr, [q1, c])?;
    let q1 = res.out_wire(0);
    let res = h.add_dataflow_op(Tk2Op::CZ, [q0, q1])?;
    let q0 = res.out_wire(0);
    let q1 = res.out_wire(1);
    
    h.finish_hugr_with_outputs([q0, q1], registry)
}

pub fn cz_zicorr(registry: &ExtensionRegistry) -> Result<Hugr, BuildError> {
    // Load the extension
    let extension = registry.get("ExtMBQC").unwrap();
    let my_bool = Type::new_extension(extension.get_type("MyBool").unwrap().instantiate([]).unwrap());
    let z_corr = extension.instantiate_extension_op("CorrectionZ", [], registry).unwrap();

    // Build the HUGR
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T, QB_T, my_bool], vec![QB_T, QB_T]))?;
    let mut inps = h.input_wires();
    let q0 = inps.next().unwrap();
    let q1 = inps.next().unwrap();
    let c = inps.next().unwrap();

    let res = h.add_dataflow_op(Tk2Op::CZ, [q0, q1])?;
    let q0 = res.out_wire(0);
    let q1 = res.out_wire(1);
    let res = h.add_dataflow_op(z_corr, [q0, c])?;
    let q0 = res.out_wire(0);
    
    h.finish_hugr_with_outputs([q0, q1], registry)
}

pub fn cz_izcorr(registry: &ExtensionRegistry) -> Result<Hugr, BuildError> {
    // Load the extension
    let extension = registry.get("ExtMBQC").unwrap();
    let my_bool = Type::new_extension(extension.get_type("MyBool").unwrap().instantiate([]).unwrap());
    let z_corr = extension.instantiate_extension_op("CorrectionZ", [], registry).unwrap();

    // Build the HUGR
    let mut h = DFGBuilder::new(FunctionType::new(vec![QB_T, QB_T, my_bool], vec![QB_T, QB_T]))?;
    let mut inps = h.input_wires();
    let q0 = inps.next().unwrap();
    let q1 = inps.next().unwrap();
    let c = inps.next().unwrap();

    let res = h.add_dataflow_op(Tk2Op::CZ, [q0, q1])?;
    let q0 = res.out_wire(0);
    let q1 = res.out_wire(1);
    let res = h.add_dataflow_op(z_corr, [q1, c])?;
    let q1 = res.out_wire(0);
    
    h.finish_hugr_with_outputs([q0, q1], registry)
}

pub fn alloc_reset_h() -> Result<Hugr, BuildError> {

    let mut h = DFGBuilder::new(FunctionType::new(vec![], vec![QB_T]))?;
    let res = h.add_dataflow_op(Tk2Op::QAlloc, [])?;
    let q = res.out_wire(0);
    let res = h.add_dataflow_op(Tk2Op::Reset, [q])?;
    let q = res.out_wire(0);
    let res = h.add_dataflow_op(Tk2Op::H, [q])?;
    let q = res.out_wire(0);
    
    h.finish_hugr_with_outputs([q], &PRELUDE_REGISTRY)
}
