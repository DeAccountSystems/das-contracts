use crate::util::{constants::*, error::Error, template_common_cell::*, template_generator::*, template_parser::*};
use ckb_testtool::ckb_hash::blake2b_256;

fn init(action: &str) -> TemplateGenerator {
    let mut template = TemplateGenerator::new(action, None);

    template.push_contract_cell("always_success", true);
    template.push_contract_cell("playground", false);
    template.push_shared_lib_cell("ckb_smt.so");

    template
}

#[test]
fn test_playground() {
    let mut template = init("playground");

    push_input_playground_cell(&mut template);

    test_tx(template.as_json());
}
