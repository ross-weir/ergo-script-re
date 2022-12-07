use ergotree_ir::mir::{
    atleast::Atleast, bin_op::BinOp, block::BlockValue, bool_to_sigma::BoolToSigmaProp,
    coll_by_index::ByIndex, coll_filter::Filter, coll_size::SizeOf, extract_amount::ExtractAmount,
    extract_id::ExtractId, extract_reg_as::ExtractRegisterAs,
    extract_script_bytes::ExtractScriptBytes, func_value::FuncValue, global_vars::GlobalVars,
    if_op::If, option_get::OptionGet, property_call::PropertyCall, select_field::SelectField,
    val_def::ValDef, val_use::ValUse,
};

pub trait Element {
    fn accept<T: Visitor>(&self, visitor: &mut T);
}

pub trait Visitor {
    fn visit_atleast(&mut self, _e: &Atleast) {}
    fn visit_bin_op(&mut self, _e: &BinOp) {}
    fn visit_bool_to_sigma_prop(&mut self, _e: &BoolToSigmaProp) {}
    fn visit_block_value(&mut self, _e: &BlockValue) {}
    fn visit_val_def(&mut self, _e: &ValDef) {}
    fn visit_val_use(&mut self, _e: &ValUse) {}
    fn visit_option_get(&mut self, _e: &OptionGet) {}
    fn visit_global_vars(&mut self, _e: &GlobalVars) {}
    fn visit_extract_reg_as(&mut self, _e: &ExtractRegisterAs) {}
    fn visit_by_index(&mut self, _e: &ByIndex) {}
    fn visit_select_field(&mut self, _e: &SelectField) {}
    fn visit_property_call(&mut self, _e: &PropertyCall) {}
    fn visit_extract_script_bytes(&mut self, _e: &ExtractScriptBytes) {}
    fn visit_extract_id(&mut self, _e: &ExtractId) {}
    fn visit_extract_amount(&mut self, _e: &ExtractAmount) {}
    fn visit_func_value(&mut self, _e: &FuncValue) {}
    fn visit_filter(&mut self, _e: &Filter) {}
    fn visit_if(&mut self, _e: &If) {}
    fn visit_size_of(&mut self, _e: &SizeOf) {}
}
