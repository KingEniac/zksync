use std::collections::HashMap;
use std::process::Command;
use std::str::FromStr;
use web3::types::{Address, H256};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Contracts {
    pub governance: Address,
    pub priority_queue: Address,
    pub verifier: Address,
    pub contract: Address,
    pub test_erc20_address: Address,
}

fn get_contract_address(deploy_script_out: &str) -> Option<(String, Address)> {
    if deploy_script_out.starts_with("GOVERNANCE_ADDR=0x") {
        Some((
            String::from("GOVERNANCE_ADDR"),
            Address::from_str(&deploy_script_out["GOVERNANCE_ADDR=0x".len()..])
                .expect("can't parse contract address"),
        ))
    } else if deploy_script_out.starts_with("PRIORITY_QUEUE_ADDR=0x") {
        Some((
            String::from("PRIORITY_QUEUE_ADDR"),
            Address::from_str(&deploy_script_out["PRIORITY_QUEUE_ADDR=0x".len()..])
                .expect("can't parse contract address"),
        ))
    } else if deploy_script_out.starts_with("VERIFIER_ADDR=0x") {
        Some((
            String::from("VERIFIER_ADDR"),
            Address::from_str(&deploy_script_out["VERIFIER_ADDR=0x".len()..])
                .expect("can't parse contract address"),
        ))
    } else if deploy_script_out.starts_with("CONTRACT_ADDR=0x") {
        Some((
            String::from("CONTRACT_ADDR"),
            Address::from_str(&deploy_script_out["CONTRACT_ADDR=0x".len()..])
                .expect("can't parse contract address"),
        ))
    } else if deploy_script_out.starts_with("TEST_ERC20=0x") {
        Some((
            String::from("TEST_ERC20"),
            Address::from_str(&deploy_script_out["TEST_ERC20=0x".len()..])
                .expect("can't parse contract address"),
        ))
    } else {
        None
    }
}

pub fn deploy_test_contracts() -> Contracts {
    let result = Command::new("sh")
        .arg("execute-deploy-test.sh")
        .output()
        .expect("failed to execute contract deploy script");

    if !result.status.success() {
        panic!("failed to run contract deploy script")
    }

    let stdout = String::from_utf8(result.stdout).expect("stdout is not valid utf8");

    let mut contracts = HashMap::new();
    for std_out_line in stdout.split_whitespace().collect::<Vec<_>>() {
        if let Some((name, address)) = get_contract_address(std_out_line) {
            contracts.insert(name, address);
        }
    }

    Contracts {
        governance: contracts
            .remove("GOVERNANCE_ADDR")
            .expect("GOVERNANCE_ADDR missing"),
        priority_queue: contracts
            .remove("PRIORITY_QUEUE_ADDR")
            .expect("PRIORITY_QUEUE_ADDR missing"),
        verifier: contracts
            .remove("VERIFIER_ADDR")
            .expect("VERIFIER_ADDR missing"),
        contract: contracts
            .remove("CONTRACT_ADDR")
            .expect("CONTRACT_ADDR missing"),
        test_erc20_address: contracts.remove("TEST_ERC20").expect("TEST_ERC20 missing"),
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ETHAccountInfo {
    pub address: Address,
    pub private_key: H256,
}
pub fn get_test_accounts() -> Vec<ETHAccountInfo> {
    let result = Command::new("sh")
        .arg("print-test-accounts.sh")
        .output()
        .expect("failed to execute print test accounts script");
    if !result.status.success() {
        panic!("print test accounts script failed")
    }
    let stdout = String::from_utf8(result.stdout).expect("stdout is not valid utf8");

    for std_out_line in stdout.split_whitespace().collect::<Vec<_>>() {
        if let Ok(parsed) = serde_json::from_str(std_out_line) {
            return parsed;
        }
    }

    panic!("Print test accounts script output is not parsed correctly")
}
