use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::retry::RetryPolicy;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowStep {
    pub id: String,
    pub activity: String,
    #[serde(default)]
    pub input: serde_json::Value,
    #[serde(default)]
    pub retry: RetryPolicy,
    #[serde(default = "default_timeout")]
    pub timeout_ms: u64,
    #[serde(default)]
    pub requires_approval: bool,
    #[serde(default)]
    pub on_failure: FailurePolicy,
}

fn default_timeout() -> u64 {
    30000
}

impl Default for WorkflowStep {
    fn default() -> Self {
        Self {
            id: String::new(),
            activity: String::new(),
            input: serde_json::Value::Null,
            retry: RetryPolicy::default(),
            timeout_ms: default_timeout(),
            requires_approval: false,
            on_failure: FailurePolicy::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum FailurePolicy {
    #[default]
    Halt,
    Continue,
}

#[async_trait]
pub trait WorkflowDefinition: Send + Sync {
    fn name(&self) -> &str;
    fn steps(&self) -> Vec<WorkflowStep>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YamlWorkflow {
    pub api_version: String,
    pub kind: String,
    pub metadata: YamlWorkflowMetadata,
    pub spec: YamlWorkflowSpec,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YamlWorkflowMetadata {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub labels: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct YamlWorkflowSpec {
    #[serde(default)]
    pub steps: Vec<WorkflowStep>,
}

impl YamlWorkflow {
    pub fn from_yaml(yaml: &str) -> serde_yaml::Result<Self> {
        serde_yaml::from_str(yaml)
    }

    pub fn to_definition(&self) -> impl WorkflowDefinition + '_ {
        YamlWorkflowDef { workflow: self }
    }
}

struct YamlWorkflowDef<'a> {
    workflow: &'a YamlWorkflow,
}

#[async_trait]
impl WorkflowDefinition for YamlWorkflowDef<'_> {
    fn name(&self) -> &str {
        if self.workflow.metadata.name.is_empty() {
            "unnamed-yaml-workflow"
        } else {
            &self.workflow.metadata.name
        }
    }

    fn steps(&self) -> Vec<WorkflowStep> {
        self.workflow.spec.steps.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_yaml_workflow() {
        let yaml = r#"
apiVersion: bastion.io/v1
kind: Workflow
metadata:
  name: test-wf
spec:
  steps:
    - id: step1
      activity: simulate
      input: { tx: "abc" }
      retry:
        maxAttempts: 3
        initialBackoffMs: 1000
        backoffMultiplier: 2.0
      timeoutMs: 30000
    - id: step2
      activity: approve
      requiresApproval: true
      timeoutMs: 3600000
"#;
        let wf = YamlWorkflow::from_yaml(yaml).expect("valid YAML");
        assert_eq!(wf.metadata.name, "test-wf");
        assert_eq!(wf.spec.steps.len(), 2);
        assert!(wf.spec.steps[1].requires_approval);
    }

    #[test]
    fn yaml_to_definition() {
        let yaml = r#"
apiVersion: bastion.io/v1
kind: Workflow
metadata:
  name: test-wf
spec:
  steps:
    - id: step1
      activity: simulate
"#;
        let wf = YamlWorkflow::from_yaml(yaml).expect("valid");
        let def = wf.to_definition();
        assert_eq!(def.name(), "test-wf");
        assert_eq!(def.steps().len(), 1);
    }
}
