use std::{collections::HashMap, ops::Index};

use ergotree_ir::mir::val_def::ValDef;

use crate::visitor::Visitor;

#[derive(Default)]
pub struct DataFlowAnalyzer {
    name_map: HashMap<u32, String>,
}

impl DataFlowAnalyzer {
    pub fn name_for_id(&self, id: u32) -> String {
        self.name_map.index(&id).clone()
    }

    pub fn add_var(&mut self, vd: &ValDef) {
        let id = vd.id.0;
        // TODO: better name gen?
        let name = format!("var_{id}");

        self.name_map.insert(id, name);
    }
}

// TODO: we may need to visit func_value as well to get func args
impl Visitor for DataFlowAnalyzer {
    fn visit_val_def(&mut self, vd: &ValDef) {
        self.add_var(vd)
    }
}

#[cfg(test)]
mod tests {
    use ergotree_ir::{ergo_tree::ErgoTree, serialization::SigmaSerializable};

    use crate::visitor::Element;

    use super::*;

    #[test]
    fn test_visiting_ergo_tree_adds_var() {
        let tree_bytes =
            base16::decode("10010400d1d801d601b2a573000094e4c672010402e4c672010502".as_bytes())
                .unwrap();
        let tree = ErgoTree::sigma_parse_bytes(&tree_bytes).unwrap();
        let mut dfa = DataFlowAnalyzer::default();
        let expr = tree.proposition().unwrap();

        expr.accept(&mut dfa);

        assert_eq!(dfa.name_for_id(1), "var_1")
    }
}
