// Bastion SDK — EVM-only example.
//
// The full-EVM pivot removed the Solana Anchor client (`BastionClient`, PDAs,
// `@solana/web3.js`). EVM contract access goes through `BastionEVMClient`
// (viem); policy/simulation/audit go through the `BastionSidecar` HTTP client;
// confidential verdicts and unclonable credentials use the ERC-8354/8380 wrappers.

import {
  BastionSidecar,
  Bastion,
  computeNullifier,
  computeCapabilityCommitment,
  commitPolicyAction,
  verify,
} from "../src";

const ZERO_ADDR = "0x0000000000000000000000000000000000000000";
const ZERO32 = ("0x" + "00".repeat(32)) as `0x${string}`;

async function main() {
  console.log("=== Bastion SDK Demo (EVM) ===\n");

  // 1. HTTP sidecar: EVM simulation + policy + audit.
  const sidecar = new BastionSidecar({
    baseUrl: "https://bastion-agentique.fly.dev",
  });
  const health = await sidecar.health().catch(() => null);
  console.log("Sidecar health:", health ? health.status : "(unreachable)");

  // 2. Unified runtime facade — composes policy + simulation + audit.
  const bastion = new Bastion({ sidecar });
  console.log(
    "Constructed unified runtime. Example call:\n" +
      '  await bastion.execute({ action: "swap", settlement: "base",\n' +
      '        privacy: "public", transaction: { from, to, value, data } });',
  );

  // 3. ERC-8380 unclonable capability credential (off-chain precompute).
  const agentId = 42n;
  const nullifier = computeNullifier({
    agentId,
    homeChainId: 1n,
    homeDomainId: ZERO32,
    capabilityIndex: 0n,
  });
  const capabilityCommitment = computeCapabilityCommitment({
    nullifier,
    actionCommitment: ZERO32,
    executor: ZERO_ADDR,
  });
  console.log("ERC-8380 nullifier:", nullifier);
  console.log("ERC-8380 capability commitment:", capabilityCommitment);

  // 4. ERC-8354 confidential policy verdict commitment.
  const actionCommitment = commitPolicyAction({
    chainId: 1n,
    domainId: ZERO32,
    agentId,
    target: ZERO_ADDR,
    value: 0n,
    callDataHash: ZERO32,
    actionNonce: 0n,
  });
  console.log("ERC-8354 action commitment:", actionCommitment);

  // 5. Recompute verification (trustless-ai compatible).
  console.log("verify namespace:", Object.keys(verify));

  console.log("\n✅ SDK ready! Run 'pnpm --filter @zkos-labs/bastion-agentique build'.");
}

main().catch(console.error);