use soroban_sdk::{Address, Env, Symbol};
use soroban_security_scanner::escrow::{Escrow, EscrowData};

#[test]
fn test_reentrancy_vulnerability() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Escrow);
    let client = EscrowClient::new(&env, &contract_id);
    
    let beneficiary = Address::generate(&env);
    let amount = 1000;
    
    // Create escrow
    client.create_escrow(&beneficiary, &amount);
    
    // Test vulnerable version - this should be exploitable
    // In a real attack, the beneficiary contract could call release() again
    // before the state is updated, draining all funds
    let escrow_info_before = client.get_escrow_info();
    assert!(!escrow_info_before.released);
    
    // This call is vulnerable to reentrancy
    client.release();
    
    let escrow_info_after = client.get_escrow_info();
    assert!(escrow_info_after.released);
}

#[test]
fn test_fixed_reentrancy() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Escrow);
    let client = EscrowClient::new(&env, &contract_id);
    
    let beneficiary = Address::generate(&env);
    let amount = 1000;
    
    // Create escrow
    client.create_escrow(&beneficiary, &amount);
    
    // Test fixed version - this should be secure
    let escrow_info_before = client.get_escrow_info();
    assert!(!escrow_info_before.released);
    
    // This call is secure against reentrancy
    client.release_fixed();
    
    let escrow_info_after = client.get_escrow_info();
    assert!(escrow_info_after.released);
    
    // Verify that calling release_fixed again fails
    let result = std::panic::catch_unwind(|| {
        client.release_fixed();
    });
    assert!(result.is_err());
}

#[test]
fn test_security_analyzer() {
    let env = Env::default();
    let analyzer = soroban_security_scanner::security_analyzer::SecurityAnalyzer;
    
    // Analyze for reentrancy vulnerabilities
    let report = analyzer.analyze_reentrancy(&env);
    
    // The report should detect the vulnerability
    assert!(!report.is_secure());
    assert!(report.has_high_severity());
    
    // Check that the specific vulnerability is detected
    let vuln_titles: Vec<String> = report.vulnerabilities.iter()
        .map(|v| v.title.clone())
        .collect();
    assert!(vuln_titles.iter().any(|title| title.contains("Reentrancy")));
}
