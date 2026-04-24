# Soroban Security Scanner

A comprehensive security scanner for Soroban smart contracts, focusing on detecting and preventing common vulnerabilities including reentrancy attacks.

## Issue #106: Reentrancy Vulnerability in Escrow Release

### Vulnerability Description

The Escrow contract's `release` function contains a critical reentrancy vulnerability where state updates occur after external interactions. This allows malicious contracts to drain funds by calling the function recursively before the state is properly updated.

### Vulnerable Code Pattern

```rust
// VULNERABLE: External call before state update
env.current_contract_address()
    .require_auth_for_args((&escrow_data.beneficiary, balance));

// Transfer funds to beneficiary (external call)
self::transfer_funds(&env, &escrow_data.beneficiary, balance);

// STATE UPDATE HAPPENS AFTER EXTERNAL CALL - VULNERABLE!
let mut updated_escrow = escrow_data.clone();
updated_escrow.released = true;
env.storage().instance().set(&escrow_key, &updated_escrow);
```

### Fixed Code Pattern

```rust
// FIX: Update state BEFORE external call
let mut updated_escrow = escrow_data.clone();
updated_escrow.released = true;
env.storage().instance().set(&escrow_key, &updated_escrow);

// Clear balance immediately
env.storage().instance().remove(&balance_key);

// External call AFTER state update - safe from reentrancy
env.current_contract_address()
    .require_auth_for_args((&escrow_data.beneficiary, balance));

// Transfer funds to beneficiary (external call)
self::transfer_funds(&env, &escrow_data.beneficiary, balance);
```

## Features

- **Reentrancy Detection**: Automatically identifies patterns where external calls precede state updates
- **Security Analysis**: Comprehensive vulnerability scanning for Soroban contracts
- **Test Suite**: Extensive tests demonstrating vulnerabilities and their fixes

## Usage

```bash
# Run tests to verify the fix
cargo test

# Run security analysis
cargo run --bin security-analyzer
```

## Security Best Practices

1. **Checks-Effects-Interactions Pattern**: Always perform checks and update state before making external calls
2. **Reentrancy Guards**: Implement reentrancy protection mechanisms
3. **Proper State Management**: Ensure atomic operations and consistent state updates

## Testing

The project includes comprehensive tests that demonstrate:
- The reentrancy vulnerability in the original `release` function
- The security of the fixed `release_fixed` function
- Automated detection of the vulnerability patterns

Run tests with:
```bash
cargo test --test reentrancy_test
```
