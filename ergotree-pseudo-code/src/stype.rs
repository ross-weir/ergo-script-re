use ergotree_ir::types::stype::SType;

use crate::ErgoScript;

impl ErgoScript for SType {
    fn ergo_script(&self) -> String {
        match self {
            SType::STypeVar(_) => todo!(),
            SType::SAny => todo!(),
            SType::SUnit => todo!(),
            SType::SBoolean => todo!(),
            SType::SByte => "Byte".to_string(),
            SType::SShort => todo!(),
            SType::SInt => todo!(),
            SType::SLong => "Long".to_string(),
            SType::SBigInt => todo!(),
            SType::SGroupElement => todo!(),
            SType::SSigmaProp => "SigmaProp".to_string(),
            SType::SBox => "Box".to_string(),
            SType::SAvlTree => todo!(),
            SType::SOption(_) => todo!(),
            SType::SColl(inner) => format!("Coll[{}]", (*(*inner)).ergo_script()),
            SType::STuple(_) => todo!(),
            SType::SFunc(_) => todo!(),
            SType::SContext => "Context".to_string(),
            SType::SHeader => todo!(),
            SType::SPreHeader => todo!(),
            SType::SGlobal => todo!(),
        }
    }
}
