use das_types_std::constants::*;
use serde_json::json;

use super::common::*;
use crate::util::accounts::*;
use crate::util::constants::*;
use crate::util::error::*;
use crate::util::template_common_cell::*;
use crate::util::template_generator::*;
use crate::util::template_parser::*;
use crate::util::{self};

fn before_each() -> TemplateGenerator {
    let mut template = init_update();

    // cell_deps
    push_simple_dep_account_cell(&mut template);

    // inputs
    push_simple_input_sub_account_cell(&mut template, 0, 0);
    push_input_normal_cell(&mut template, 10_000_000_000, OWNER);

    template
}

#[test]
fn test_sub_account_create_without_custom_script() {
    let mut template = before_each();

    // outputs
    let smt = push_commen_sign_witness(&mut template);
    template.push_sub_account_witness_v2(json!({
        "action": SubAccountAction::Create.to_string(),
        "sub_account": {
            "lock": {
                "owner_lock_args": OWNER_1,
                "manager_lock_args": MANAGER_1
            },
            "account": SUB_ACCOUNT_1,
            "suffix": SUB_ACCOUNT_SUFFIX,
            "registered_at": TIMESTAMP,
            "expired_at": TIMESTAMP + YEAR_SEC,
        },
        "edit_value": get_compiled_proof(&smt, SUB_ACCOUNT_1)
    }));
    template.push_sub_account_witness_v2(json!({
        "action": SubAccountAction::Create.to_string(),
        "sub_account": {
            "lock": {
                "owner_lock_args": OWNER_2,
                "manager_lock_args": MANAGER_2
            },
            "account": SUB_ACCOUNT_2,
            "suffix": SUB_ACCOUNT_SUFFIX,
            "registered_at": TIMESTAMP,
            "expired_at": TIMESTAMP + YEAR_SEC,
        },
        "edit_value": get_compiled_proof(&smt, SUB_ACCOUNT_2)
    }));
    template.push_sub_account_witness_v2(json!({
        "action": SubAccountAction::Create.to_string(),
        "sub_account": {
            "lock": {
                "owner_lock_args": OWNER_3,
                "manager_lock_args": MANAGER_3
            },
            "account": SUB_ACCOUNT_3,
            "suffix": SUB_ACCOUNT_SUFFIX,
            "registered_at": TIMESTAMP,
            "expired_at": TIMESTAMP + YEAR_SEC,
        },
        "edit_value": get_compiled_proof(&smt, SUB_ACCOUNT_3)
    }));
    push_common_output_cells(&mut template, 3);

    test_tx(template.as_json())
}

#[test]
fn challenge_sub_account_create_invalid_char() {
    let mut template = before_each();
    let account = "✨das🇫🇮001.xxxxx.bit";

    // outputs
    let smt = template.push_sub_account_mint_sign_witness(json!({
        "version": 1,
        "expired_at": TIMESTAMP + DAY_SEC,
        "account_list_smt_root": [
            [account, gen_das_lock_args(OWNER_1, Some(MANAGER_1))],
        ]
    }));
    template.push_sub_account_witness_v2(json!({
        "action": SubAccountAction::Create.to_string(),
        "sub_account": {
            "lock": {
                "owner_lock_args": OWNER_1,
                "manager_lock_args": MANAGER_1
            },
            // Simulate the sub-account contains invalid character.
            "account": [
                { "char": "✨", "type": CharSetType::Emoji as u32 },
                { "char": "d", "type": CharSetType::En as u32 },
                { "char": "a", "type": CharSetType::En as u32 },
                { "char": "s", "type": CharSetType::En as u32 },
                { "char": "🇫🇮", "type": CharSetType::Emoji as u32 },
                { "char": "0", "type": CharSetType::Digit as u32 },
                { "char": "0", "type": CharSetType::Digit as u32 },
                { "char": "1", "type": CharSetType::Digit as u32 },
            ],
            "suffix": SUB_ACCOUNT_SUFFIX,
            "registered_at": TIMESTAMP,
            "expired_at": TIMESTAMP + YEAR_SEC,
        },
        "edit_value": get_compiled_proof(&smt, account)
    }));
    push_common_output_cells(&mut template, 1);

    challenge_tx(template.as_json(), ErrorCode::AccountCharIsInvalid);
}

#[test]
fn challenge_sub_account_create_undefined_char() {
    let mut template = before_each();
    let account = "✨das大001.xxxxx.bit";

    // outputs
    let smt = template.push_sub_account_mint_sign_witness(json!({
        "version": 1,
        "expired_at": TIMESTAMP + DAY_SEC,
        "account_list_smt_root": [
            [account, gen_das_lock_args(OWNER_1, Some(MANAGER_1))],
        ]
    }));
    template.push_sub_account_witness_v2(json!({
        "action": SubAccountAction::Create.to_string(),
        "sub_account": {
            "lock": {
                "owner_lock_args": OWNER_1,
                "manager_lock_args": MANAGER_1
            },
            // Simulate the sub-account contains undefined character.
            "account": [
                { "char": "✨", "type": CharSetType::Emoji as u32 },
                { "char": "d", "type": CharSetType::En as u32 },
                { "char": "a", "type": CharSetType::En as u32 },
                { "char": "s", "type": CharSetType::En as u32 },
                { "char": "大", "type": CharSetType::ZhHans as u32 },
                { "char": "0", "type": CharSetType::Digit as u32 },
                { "char": "0", "type": CharSetType::Digit as u32 },
                { "char": "1", "type": CharSetType::Digit as u32 },
            ],
            "suffix": SUB_ACCOUNT_SUFFIX,
            "registered_at": TIMESTAMP,
            "expired_at": TIMESTAMP + YEAR_SEC,
        },
        "edit_value": get_compiled_proof(&smt, account)
    }));
    push_common_output_cells(&mut template, 1);

    challenge_tx(template.as_json(), ErrorCode::ConfigIsPartialMissing);
}

#[test]
fn challenge_sub_account_create_too_long() {
    let mut template = before_each();
    let account = "1234567890123456789012345678901234567890123.xxxxx.bit";

    // outputs
    let smt = template.push_sub_account_mint_sign_witness(json!({
        "version": 1,
        "expired_at": TIMESTAMP + DAY_SEC,
        "account_list_smt_root": [
            [account, gen_das_lock_args(OWNER_1, Some(MANAGER_1))],
        ]
    }));
    template.push_sub_account_witness_v2(json!({
        "action": SubAccountAction::Create.to_string(),
        "sub_account": {
            "lock": {
                "owner_lock_args": OWNER_1,
                "manager_lock_args": MANAGER_1
            },
            // Simulate the sub-account is too long.
            "account": "1234567890123456789012345678901234567890123.xxxxx.bit",
            "suffix": SUB_ACCOUNT_SUFFIX,
            "registered_at": TIMESTAMP,
            "expired_at": TIMESTAMP + YEAR_SEC,
        },
        "edit_value": get_compiled_proof(&smt, account)
    }));
    push_common_output_cells(&mut template, 1);

    challenge_tx(template.as_json(), ErrorCode::AccountIsTooLong);
}

#[test]
fn challenge_sub_account_create_suffix_not_match() {
    let mut template = before_each();
    let account = "00000.a.bit";

    // outputs
    let smt = template.push_sub_account_mint_sign_witness(json!({
        "version": 1,
        "expired_at": TIMESTAMP + DAY_SEC,
        "account_list_smt_root": [
            [account, gen_das_lock_args(OWNER_1, Some(MANAGER_1))],
        ]
    }));
    template.push_sub_account_witness_v2(json!({
        "action": SubAccountAction::Create.to_string(),
        "sub_account": {
            "lock": {
                "owner_lock_args": OWNER_1,
                "manager_lock_args": MANAGER_1
            },
            // Simulate the suffix is not match with the parent account.
            "account": "00000.a.bit",
            "suffix": ".a.bit",
            "registered_at": TIMESTAMP,
            "expired_at": TIMESTAMP + YEAR_SEC,
        },
        "edit_value": get_compiled_proof(&smt, account)
    }));
    push_common_output_cells(&mut template, 1);

    challenge_tx(template.as_json(), ErrorCode::SubAccountInitialValueError);
}

#[test]
fn challenge_sub_account_create_id_not_match() {
    let mut template = before_each();

    // outputs
    let smt = template.push_sub_account_mint_sign_witness(json!({
        "version": 1,
        "expired_at": TIMESTAMP + DAY_SEC,
        "account_list_smt_root": [
            [SUB_ACCOUNT_1, gen_das_lock_args(OWNER_1, Some(MANAGER_1))],
        ]
    }));
    template.push_sub_account_witness_v2(json!({
        "action": SubAccountAction::Create.to_string(),
        "sub_account": {
            "lock": {
                "owner_lock_args": OWNER_1,
                "manager_lock_args": MANAGER_1
            },
            // Simulate the id is not match with the account.
            "id": util::bytes_to_hex(&util::account_to_id(SUB_ACCOUNT_1)),
            "account": SUB_ACCOUNT_2,
            "suffix": SUB_ACCOUNT_SUFFIX,
            "registered_at": TIMESTAMP,
            "expired_at": TIMESTAMP + YEAR_SEC,
        },
        "edit_value": get_compiled_proof(&smt, SUB_ACCOUNT_1)
    }));
    push_common_output_cells(&mut template, 1);

    challenge_tx(template.as_json(), ErrorCode::SubAccountWitnessSMTRootError);
}

#[test]
fn challenge_sub_account_create_registered_at_is_invalid() {
    let mut template = before_each();

    // outputs
    let smt = push_commen_sign_witness(&mut template);
    template.push_sub_account_witness_v2(json!({
        "action": SubAccountAction::Create.to_string(),
        "sub_account": {
            "lock": {
                "owner_lock_args": OWNER_1,
                "manager_lock_args": MANAGER_1
            },
            "account": SUB_ACCOUNT_1,
            "suffix": SUB_ACCOUNT_SUFFIX,
            // Simulate the registered_at is not the same as the TimeCell.
            "registered_at": TIMESTAMP - 1,
            "expired_at": TIMESTAMP + YEAR_SEC,
        },
        "edit_value": get_compiled_proof(&smt, SUB_ACCOUNT_1)
    }));
    push_common_output_cells(&mut template, 1);

    challenge_tx(template.as_json(), ErrorCode::SubAccountInitialValueError);
}

#[test]
fn challenge_sub_account_create_expired_at_less_than_one_year() {
    let mut template = before_each();

    // outputs
    let smt = push_commen_sign_witness(&mut template);
    template.push_sub_account_witness_v2(json!({
        "action": SubAccountAction::Create.to_string(),
        "sub_account": {
            "lock": {
                "owner_lock_args": OWNER_1,
                "manager_lock_args": MANAGER_1
            },
            "account": SUB_ACCOUNT_1,
            "suffix": SUB_ACCOUNT_SUFFIX,
            "registered_at": TIMESTAMP,
            // Simulate the expired_at is less than one year.
            "expired_at": TIMESTAMP + YEAR_SEC - 1,
        },
        "edit_value": get_compiled_proof(&smt, SUB_ACCOUNT_1)
    }));
    push_common_output_cells(&mut template, 1);

    challenge_tx(template.as_json(), ErrorCode::SubAccountInitialValueError);
}

#[test]
fn challenge_sub_account_create_no_profit_record() {
    let mut template = before_each();

    // outputs
    let smt = push_commen_sign_witness(&mut template);
    template.push_sub_account_witness_v2(json!({
        "action": SubAccountAction::Create.to_string(),
        "sub_account": {
            "lock": {
                "owner_lock_args": OWNER_1,
                "manager_lock_args": MANAGER_1
            },
            "account": SUB_ACCOUNT_1,
            "suffix": SUB_ACCOUNT_SUFFIX,
            "registered_at": TIMESTAMP,
            "expired_at": TIMESTAMP + YEAR_SEC,
        },
        "edit_value": get_compiled_proof(&smt, SUB_ACCOUNT_1)
    }));

    let das_profit = calculate_sub_account_cost(1);
    // Simulate forget record correct profit in the outputs_data of the SubAccountCell
    push_simple_output_sub_account_cell(&mut template, 0, 0);
    push_output_normal_cell(&mut template, 10_000_000_000 - das_profit, OWNER);

    challenge_tx(template.as_json(), ErrorCode::SubAccountProfitError);
}

#[test]
fn challenge_sub_account_create_profit_not_match_capacity() {
    let mut template = before_each();

    // outputs
    let smt = push_commen_sign_witness(&mut template);
    template.push_sub_account_witness_v2(json!({
        "action": SubAccountAction::Create.to_string(),
        "sub_account": {
            "lock": {
                "owner_lock_args": OWNER_1,
                "manager_lock_args": MANAGER_1
            },
            "account": SUB_ACCOUNT_1,
            "suffix": SUB_ACCOUNT_SUFFIX,
            "registered_at": TIMESTAMP,
            "expired_at": TIMESTAMP + YEAR_SEC,
        },
        "edit_value": get_compiled_proof(&smt, SUB_ACCOUNT_1)
    }));

    let das_profit = calculate_sub_account_cost(1);
    let current_root = template.smt_with_history.current_root();
    push_output_sub_account_cell(
        &mut template,
        json!({
            // Simulate forget put profit into the capacity of the SubAccountCell
            "capacity": SUB_ACCOUNT_BASIC_CAPACITY + das_profit - 1,
            "type": {
                "args": ACCOUNT_1
            },
            "data": {
                "root": String::from("0x") + &hex::encode(&current_root),
                "das_profit": das_profit
            }
        }),
    );

    push_output_normal_cell(&mut template, 10_000_000_000 - das_profit, OWNER);

    challenge_tx(template.as_json(), ErrorCode::SubAccountCellCapacityError);
}

#[test]
fn challenge_sub_account_create_mint_sign_expired() {
    let mut template = before_each();

    // outputs
    let smt = template.push_sub_account_mint_sign_witness(json!({
        "version": 1,
        // Simulate generating the mint signature with too large value.
        "expired_at": TIMESTAMP + YEAR_SEC + 1,
        "account_list_smt_root": [
            [SUB_ACCOUNT_1, gen_das_lock_args(OWNER_1, Some(MANAGER_1))],
            [SUB_ACCOUNT_2, gen_das_lock_args(OWNER_2, Some(MANAGER_2))],
            [SUB_ACCOUNT_3, gen_das_lock_args(OWNER_3, Some(MANAGER_3))],
            [SUB_ACCOUNT_4, gen_das_lock_args(OWNER_4, Some(MANAGER_4))],
        ]
    }));
    template.push_sub_account_witness_v2(json!({
        "action": SubAccountAction::Create.to_string(),
        "sub_account": {
            "lock": {
                "owner_lock_args": OWNER_1,
                "manager_lock_args": MANAGER_1
            },
            "account": SUB_ACCOUNT_1,
            "suffix": SUB_ACCOUNT_SUFFIX,
            "registered_at": TIMESTAMP,
            "expired_at": TIMESTAMP + YEAR_SEC,
        },
        "edit_value": get_compiled_proof(&smt, SUB_ACCOUNT_1)
    }));
    push_common_output_cells(&mut template, 1);

    challenge_tx(template.as_json(), ErrorCode::SubAccountSignMintExpiredAtTooLarge);
}

#[test]
fn challenge_sub_account_create_sign_expired_at_less_than_the_minimal_expired_at_1() {
    let mut template = before_each();

    // outputs
    let smt = template.push_sub_account_mint_sign_witness(json!({
        "version": 1,
        // Simulate generating the mint signature with too large value.
        "expired_at": TIMESTAMP + YEAR_SEC + 1,
        "account_list_smt_root": [
            [SUB_ACCOUNT_1, gen_das_lock_args(OWNER_1, Some(MANAGER_1))],
            [SUB_ACCOUNT_2, gen_das_lock_args(OWNER_2, Some(MANAGER_2))],
            [SUB_ACCOUNT_3, gen_das_lock_args(OWNER_3, Some(MANAGER_3))],
            [SUB_ACCOUNT_4, gen_das_lock_args(OWNER_4, Some(MANAGER_4))],
        ]
    }));
    template.push_sub_account_witness_v2(json!({
        "action": SubAccountAction::Create.to_string(),
        "sub_account": {
            "lock": {
                "owner_lock_args": OWNER_1,
                "manager_lock_args": MANAGER_1
            },
            "account": SUB_ACCOUNT_1,
            "suffix": SUB_ACCOUNT_SUFFIX,
            "registered_at": TIMESTAMP,
            "expired_at": TIMESTAMP + YEAR_SEC,
        },
        "edit_value": get_compiled_proof(&smt, SUB_ACCOUNT_1)
    }));
    push_common_output_cells(&mut template, 1);

    challenge_tx(template.as_json(), ErrorCode::SubAccountSignMintExpiredAtTooLarge);
}

#[test]
fn challenge_sub_account_create_sign_expired_at_less_than_the_minimal_expired_at_2() {
    let mut template = before_each();

    // outputs
    let smt = template.push_sub_account_mint_sign_witness(json!({
        "version": 1,
        // Simulate generating the mint signature with too large value.
        "expired_at": TIMESTAMP - DAY_SEC - 1,
        "account_list_smt_root": [
            [SUB_ACCOUNT_1, gen_das_lock_args(OWNER_1, Some(MANAGER_1))],
            [SUB_ACCOUNT_2, gen_das_lock_args(OWNER_2, Some(MANAGER_2))],
            [SUB_ACCOUNT_3, gen_das_lock_args(OWNER_3, Some(MANAGER_3))],
            [SUB_ACCOUNT_4, gen_das_lock_args(OWNER_4, Some(MANAGER_4))],
        ]
    }));
    template.push_sub_account_witness_v2(json!({
        "action": SubAccountAction::Create.to_string(),
        "sub_account": {
            "lock": {
                "owner_lock_args": OWNER_1,
                "manager_lock_args": MANAGER_1
            },
            "account": SUB_ACCOUNT_1,
            "suffix": SUB_ACCOUNT_SUFFIX,
            "registered_at": TIMESTAMP,
            "expired_at": TIMESTAMP + YEAR_SEC,
        },
        "edit_value": get_compiled_proof(&smt, SUB_ACCOUNT_1)
    }));
    push_common_output_cells(&mut template, 1);

    challenge_tx(template.as_json(), ErrorCode::SubAccountSignMintExpiredAtReached);
}

#[test]
fn challenge_sub_account_create_balance_cell_capacity_not_match() {
    let mut template = init_update();

    // cell_deps
    push_simple_dep_account_cell(&mut template);

    // inputs
    push_simple_input_sub_account_cell(&mut template, 0, 0);
    push_input_balance_cell(&mut template, 10_000_000_000, OWNER);

    // outputs
    let smt = push_commen_sign_witness(&mut template);
    template.push_sub_account_witness_v2(json!({
        "action": SubAccountAction::Create.to_string(),
        "sub_account": {
            "lock": {
                "owner_lock_args": OWNER_1,
                "manager_lock_args": MANAGER_1
            },
            "account": SUB_ACCOUNT_1,
            "suffix": SUB_ACCOUNT_SUFFIX,
            "registered_at": TIMESTAMP,
            "expired_at": TIMESTAMP + YEAR_SEC,
        },
        "edit_value": get_compiled_proof(&smt, SUB_ACCOUNT_1)
    }));

    let das_profit = calculate_sub_account_cost(1);
    push_simple_output_sub_account_cell(&mut template, das_profit, 0);

    // Simulate spending more capacity than really needed in the new account creation.
    push_output_balance_cell(
        &mut template,
        10_000_000_000 - SUB_ACCOUNT_COMMON_FEE - das_profit - 1,
        OWNER,
    );

    challenge_tx(template.as_json(), ErrorCode::SubAccountBalanceManagerError);
}

#[test]
fn challenge_sub_account_create_spend_others_balance_cell() {
    let mut template = init_update();

    // cell_deps
    push_simple_dep_account_cell(&mut template);

    // inputs
    push_simple_input_sub_account_cell(&mut template, 0, 0);
    // Simulate spending others BalanceCells in the transaction.
    push_input_balance_cell(&mut template, 10_000_000_000, OWNER_4);

    // outputs
    let smt = push_commen_sign_witness(&mut template);
    template.push_sub_account_witness_v2(json!({
        "action": SubAccountAction::Create.to_string(),
        "sub_account": {
            "lock": {
                "owner_lock_args": OWNER_1,
                "manager_lock_args": MANAGER_1
            },
            "account": SUB_ACCOUNT_1,
            "suffix": SUB_ACCOUNT_SUFFIX,
            "registered_at": TIMESTAMP,
            "expired_at": TIMESTAMP + YEAR_SEC,
        },
        "edit_value": get_compiled_proof(&smt, SUB_ACCOUNT_1)
    }));

    let das_profit = calculate_sub_account_cost(1);
    push_simple_output_sub_account_cell(&mut template, das_profit, 0);

    push_output_balance_cell(
        &mut template,
        10_000_000_000 - SUB_ACCOUNT_COMMON_FEE - das_profit,
        OWNER,
    );

    challenge_tx(template.as_json(), ErrorCode::BalanceCellCanNotBeSpent);
}