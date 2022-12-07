use data_flow_analyzer::DataFlowAnalyzer;
use ergotree_ir::{ergo_tree::ErgoTree, mir::expr::Expr};
use error::PseudoCodeError;

pub mod data_flow_analyzer;
pub mod error;
pub mod generator;
pub mod mir;
pub mod stype;
pub mod visitor;

pub struct GeneratorContext {
    pub tree: ErgoTree,
    pub dfa: DataFlowAnalyzer,
}

impl GeneratorContext {
    pub fn from_tree(tree: ErgoTree) -> Self {
        Self {
            tree,
            dfa: Default::default(),
        }
    }

    pub fn from_expr(expr: Expr) -> Self {
        let tree = ErgoTree::new(Default::default(), &expr).unwrap();

        Self {
            tree,
            dfa: Default::default(),
        }
    }
}

pub trait PseudoCode {
    fn pseudo_code(&self, ctx: &GeneratorContext) -> Result<String, PseudoCodeError>;
}

/// Converts Self to its ErgoScript equivilence.
pub trait ErgoScript {
    fn ergo_script(&self) -> String;
}
