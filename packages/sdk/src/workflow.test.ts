import { BastionWorkflow } from "./workflow";
import type { BastionSidecar } from "./sidecar";
import type { TrustIntent } from "./workflow";

type RequestCall = { method: string; path: string; body?: unknown };

/** Build a BastionWorkflow over a sidecar whose request() is captured. */
function makeWorkflow(
  respond: (call: RequestCall) => unknown,
): { wf: BastionWorkflow; calls: RequestCall[] } {
  const calls: RequestCall[] = [];
  const sidecar = {
    request: async (method: string, path: string, body?: unknown) => {
      const call = { method, path, body };
      calls.push(call);
      return respond(call);
    },
  } as unknown as BastionSidecar;
  return { wf: new BastionWorkflow(sidecar), calls };
}

const INTENT: TrustIntent = {
  description: "swap then stake",
  agent_id: "agent-001",
  source_chain: "Ethereum",
  actions: [
    {
      action_type: "Swap",
      chain: "Ethereum",
      protocol: "uniswap",
      token: "USDC",
      amount: 1000,
    },
    { action_type: "Stake", chain: "Base", protocol: "aerodrome" },
  ],
};

describe("BastionWorkflow.executeIntent", () => {
  it("POSTs the intent to /execute with approval enabled by default", async () => {
    const { wf, calls } = makeWorkflow(() => ({
      plan_id: "plan-1",
      workflow_id: "wf-1",
      status: "running",
      selected_chain: "Base",
      routes: [{ chain: "Base", score: 0.9 }],
      legs: [],
      require_approval: true,
    }));

    const res = await wf.executeIntent(INTENT);

    expect(calls).toHaveLength(1);
    expect(calls[0].method).toBe("POST");
    expect(calls[0].path).toBe("/execute");
    const body = calls[0].body as {
      intent: TrustIntent & { constraints: unknown[] };
      require_approval: boolean;
    };
    // constraints defaults to [] so the sidecar can deserialize the intent.
    expect(body.intent.constraints).toEqual([]);
    expect(body.intent.agent_id).toBe("agent-001");
    expect(body.require_approval).toBe(true);
    expect(res.plan_id).toBe("plan-1");
    expect(res.selected_chain).toBe("Base");
  });

  it("forwards requireApproval: false", async () => {
    const { wf, calls } = makeWorkflow(() => ({}));
    await wf.executeIntent(INTENT, { requireApproval: false });
    expect((calls[0].body as { require_approval: boolean }).require_approval).toBe(
      false,
    );
  });
});

describe("BastionWorkflow.plan", () => {
  it("GETs /execute/:plan_id", async () => {
    const { wf, calls } = makeWorkflow(() => ({
      plan_id: "plan-1",
      workflow_id: "wf-1",
      workflow_status: "completed",
      compensation: {
        total_compensated: 0,
        total_in_progress: 0,
        total_failed: 0,
      },
    }));
    const res = await wf.plan("plan-1");
    expect(calls[0].method).toBe("GET");
    expect(calls[0].path).toBe("/execute/plan-1");
    expect(res.workflow_status).toBe("completed");
  });
});

describe("BastionWorkflow.compensate", () => {
  it("POSTs to /execute/:plan_id/compensate", async () => {
    const { wf, calls } = makeWorkflow(() => ({
      plan_id: "plan-1",
      status: "Compensating",
      compensation_workflow_id: "wf-2",
      compensating_legs: ["leg-1"],
    }));
    const res = await wf.compensate("plan-1");
    expect(calls[0].method).toBe("POST");
    expect(calls[0].path).toBe("/execute/plan-1/compensate");
    expect(res.compensating_legs).toEqual(["leg-1"]);
  });
});
