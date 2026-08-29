// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import { Script, console } from "forge-std/Script.sol";
import { BastionPolicy } from "../src/BastionPolicy.sol";
import { BastionAudit } from "../src/BastionAudit.sol";
import { BastionFirewall } from "../src/BastionFirewall.sol";
import { BastionRegistry } from "../src/BastionRegistry.sol";
import { BastionERC8004Registry } from "../src/BastionERC8004Registry.sol";
import { IBastionPolicy } from "../src/interfaces/IBastionPolicy.sol";
import { IBastionAudit } from "../src/interfaces/IBastionAudit.sol";

// ERC-8354 "Confidential Agent Policy Verdicts" (draft)
import { BastionConfidentialVerdict } from "../src/erc8354/BastionConfidentialVerdict.sol";
import { BastionPolicyDomainRegistry } from "../src/erc8354/BastionPolicyDomainRegistry.sol";
import { IPolicyDomainRegistry } from "../src/erc8354/IPolicyDomainRegistry.sol";

// ERC-8380 "Unclonable Agent Execution Credentials" (draft)
import {
    BastionUnclonableCredentialGuard
} from "../src/erc8380/BastionUnclonableCredentialGuard.sol";
import { DomainRegistry } from "../src/erc8380/DomainRegistry.sol";
import { MockVerifier } from "../src/erc8380/mocks/MockVerifier.sol";

/// @title DeployBastion
/// @notice Deploy the full Bastion protocol to any EVM chain.
/// Testnet-only until the external audit clears (see docs/EVM_READINESS.md §6).
/// Usage (load env first: `source .env`):
///   forge script script/DeployBastion.s.sol --rpc-url ethereum_sepolia --broadcast --verify
///   forge script script/DeployBastion.s.sol --rpc-url celo_testnet --broadcast --verify
///   forge script script/DeployBastion.s.sol --rpc-url celo --broadcast --verify
///   forge script script/DeployBastion.s.sol --rpc-url base --broadcast --verify
///   forge script script/DeployBastion.s.sol --rpc-url ethereum --broadcast --verify
///   forge script script/DeployBastion.s.sol --rpc-url zksync_sepolia --broadcast --verify
///   forge script script/DeployBastion.s.sol --rpc-url zksync --broadcast --verify
///   forge script script/DeployBastion.s.sol --rpc-url robinhood_testnet --broadcast --verify
///   forge script script/DeployBastion.s.sol --rpc-url robinhood --broadcast --verify
contract DeployBastion is Script {
    function run() external {
        uint deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        address deployer = vm.addr(deployerPrivateKey);

        console.log("Deployer:", deployer);
        console.log("Chain ID:", block.chainid);

        vm.startBroadcast(deployerPrivateKey);

        // 1. Deploy Audit (owner wires the firewall after it is deployed)
        BastionAudit audit = new BastionAudit(deployer);
        console.log("BastionAudit deployed at:", address(audit));

        // 2. Deploy Policy
        BastionPolicy policy = new BastionPolicy(deployer);
        console.log("BastionPolicy deployed at:", address(policy));

        // 3. Deploy Registry (original BastionRegistry)
        BastionRegistry registry = new BastionRegistry(deployer);
        console.log("BastionRegistry deployed at:", address(registry));

        // 3b. Deploy ERC-8004 Identity Registry
        BastionERC8004Registry erc8004Registry = new BastionERC8004Registry(deployer);
        console.log("BastionERC8004Registry deployed at:", address(erc8004Registry));

        // 4. Deploy Firewall
        BastionFirewall firewall = new BastionFirewall(
            IBastionPolicy(address(policy)), IBastionAudit(address(audit)), deployer
        );
        console.log("BastionFirewall deployed at:", address(firewall));

        // 5. Authorize the firewall as the sole audit-log writer.
        audit.setFirewall(address(firewall));
        console.log("Audit firewall wired to:", address(firewall));

        // 6. Deploy the ERC-8354 confidential-verdict stack
        //    (identity → evidence/validation → policyDomain → confidentialVerdict).
        BastionPolicyDomainRegistry policyDomainRegistry = new BastionPolicyDomainRegistry();
        console.log(
            "BastionPolicyDomainRegistry (ERC-8354) deployed at:", address(policyDomainRegistry)
        );

        BastionConfidentialVerdict confidentialVerdict =
            new BastionConfidentialVerdict(IPolicyDomainRegistry(address(policyDomainRegistry)));
        console.log(
            "BastionConfidentialVerdict (ERC-8354) deployed at:", address(confidentialVerdict)
        );

        // 7. Deploy the ERC-8380 unclonable-credential stack
        //    (domain registry → verifier → unclonableGuard).
        DomainRegistry uacDomainRegistry = new DomainRegistry();
        console.log("DomainRegistry (ERC-8380) deployed at:", address(uacDomainRegistry));

        // NOTE: MockVerifier is a test double. Replace with a real ZK verifier before any
        // production deployment (see ERC-8380 Security Considerations).
        MockVerifier uacVerifier = new MockVerifier();
        console.log(
            "MockVerifier (ERC-8380, test double - REPLACE before prod) deployed at:",
            address(uacVerifier)
        );

        BastionUnclonableCredentialGuard unclonableGuard = new BastionUnclonableCredentialGuard(
            address(uacVerifier), address(uacDomainRegistry), deployer
        );
        console.log(
            "BastionUnclonableCredentialGuard (ERC-8380) deployed at:", address(unclonableGuard)
        );

        vm.stopBroadcast();

        console.log("\n=== Bastion Protocol Deployed ===");
        console.log("Chain ID:", block.chainid);
        console.log("Audit:", address(audit));
        console.log("Policy:", address(policy));
        console.log("Registry:", address(registry));
        console.log("ERC-8004 Registry:", address(erc8004Registry));
        console.log("Firewall:", address(firewall));
        console.log("Policy Domain Registry (ERC-8354):", address(policyDomainRegistry));
        console.log("Confidential Verdict (ERC-8354):", address(confidentialVerdict));
        console.log("Unclonable Domain Registry (ERC-8380):", address(uacDomainRegistry));
        console.log("Unclonable Credential Guard (ERC-8380):", address(unclonableGuard));
    }
}
