use sled::Db;
use std::path::Path;

pub struct WorkflowStore {
    db: Db,
}

const TREE_WORKFLOWS: &str = "workflows";
const TREE_EVENTS: &str = "workflow_events";
const TREE_INDEX: &str = "workflow_index";
const TREE_AGENTS: &str = "workflow_agents";
const TREE_IDEMPOTENCY: &str = "idempotency";

impl WorkflowStore {
    pub fn open(path: &Path) -> sled::Result<Self> {
        let db = sled::Config::new().path(path).open()?;
        Ok(Self { db })
    }

    pub fn db(&self) -> &Db {
        &self.db
    }

    pub fn workflows_tree(&self) -> sled::Result<sled::Tree> {
        self.db.open_tree(TREE_WORKFLOWS)
    }

    pub fn events_tree(&self) -> sled::Result<sled::Tree> {
        self.db.open_tree(TREE_EVENTS)
    }

    pub fn index_tree(&self) -> sled::Result<sled::Tree> {
        self.db.open_tree(TREE_INDEX)
    }

    pub fn agents_tree(&self) -> sled::Result<sled::Tree> {
        self.db.open_tree(TREE_AGENTS)
    }

    pub fn idempotency_tree(&self) -> sled::Result<sled::Tree> {
        self.db.open_tree(TREE_IDEMPOTENCY)
    }

    pub fn save_workflow(&self, state: &crate::state::WorkflowState) -> super::Result<()> {
        let tree = self.workflows_tree()?;
        let key = state.id.as_bytes();
        let value = serde_json::to_vec(state)?;
        tree.insert(key, value)?;
        Ok(())
    }

    pub fn load_workflow(&self, id: &str) -> super::Result<Option<crate::state::WorkflowState>> {
        let tree = self.workflows_tree()?;
        let key = id.as_bytes();
        match tree.get(key)? {
            Some(v) => Ok(Some(serde_json::from_slice(&v)?)),
            None => Ok(None),
        }
    }

    pub fn append_event(&self, wf_id: &str, event: &crate::event::WorkflowEvent) -> super::Result<()> {
        let tree = self.events_tree()?;
        let seq = tree.scan_prefix(wf_id.as_bytes()).count() as u64;
        let mut key = Vec::with_capacity(wf_id.len() + 9);
        key.extend_from_slice(wf_id.as_bytes());
        key.push(0);
        key.extend_from_slice(&seq.to_be_bytes());
        let value = serde_json::to_vec(event)?;
        tree.insert(key, value)?;
        Ok(())
    }

    pub fn load_events(&self, wf_id: &str) -> super::Result<Vec<crate::event::WorkflowEvent>> {
        let tree = self.events_tree()?;
        let mut events = Vec::new();
        for item in tree.scan_prefix(wf_id.as_bytes()) {
            let (_, v) = item?;
            let event: crate::event::WorkflowEvent = serde_json::from_slice(&v)?;
            events.push(event);
        }
        Ok(events)
    }

    pub fn index_by_status(&self, wf_id: &str, status: &str) -> super::Result<()> {
        let tree = self.index_tree()?;
        let mut key = Vec::with_capacity(status.len() + 1 + wf_id.len());
        key.extend_from_slice(status.as_bytes());
        key.push(0);
        key.extend_from_slice(wf_id.as_bytes());
        tree.insert(key, &[])?;
        Ok(())
    }

    pub fn remove_status_index(&self, wf_id: &str, status: &str) -> super::Result<()> {
        let tree = self.index_tree()?;
        let mut key = Vec::with_capacity(status.len() + 1 + wf_id.len());
        key.extend_from_slice(status.as_bytes());
        key.push(0);
        key.extend_from_slice(wf_id.as_bytes());
        tree.remove(key)?;
        Ok(())
    }

    pub fn scan_by_status(&self, status: &str) -> super::Result<Vec<String>> {
        let tree = self.index_tree()?;
        let prefix = status.as_bytes();
        let mut ids = Vec::new();
        for item in tree.scan_prefix(prefix) {
            let (k, _) = item?;
            if let Some(pos) = k.iter().position(|&b| b == 0) {
                let wf_id = std::str::from_utf8(&k[pos + 1..]).unwrap_or("").to_string();
                if !wf_id.is_empty() {
                    ids.push(wf_id);
                }
            }
        }
        Ok(ids)
    }

    pub fn index_by_agent(&self, agent_id: &str, wf_id: &str) -> super::Result<()> {
        let tree = self.agents_tree()?;
        let mut key = Vec::with_capacity(agent_id.len() + 1 + wf_id.len());
        key.extend_from_slice(agent_id.as_bytes());
        key.push(0);
        key.extend_from_slice(wf_id.as_bytes());
        tree.insert(key, &[])?;
        Ok(())
    }

    pub fn list_by_agent(&self, agent_id: &str) -> super::Result<Vec<String>> {
        let tree = self.agents_tree()?;
        let prefix = agent_id.as_bytes();
        let mut ids = Vec::new();
        for item in tree.scan_prefix(prefix) {
            let (k, _) = item?;
            if let Some(pos) = k.iter().position(|&b| b == 0) {
                let wf_id = std::str::from_utf8(&k[pos + 1..]).unwrap_or("").to_string();
                if !wf_id.is_empty() {
                    ids.push(wf_id);
                }
            }
        }
        Ok(ids)
    }

    pub fn save_idempotency(&self, wf_id: &str, step_id: &str, output: &serde_json::Value) -> super::Result<()> {
        let tree = self.idempotency_tree()?;
        let mut key = Vec::with_capacity(wf_id.len() + 1 + step_id.len());
        key.extend_from_slice(wf_id.as_bytes());
        key.push(0);
        key.extend_from_slice(step_id.as_bytes());
        tree.insert(key, serde_json::to_vec(output)?)?;
        Ok(())
    }

    pub fn check_idempotency(&self, wf_id: &str, step_id: &str) -> super::Result<Option<serde_json::Value>> {
        let tree = self.idempotency_tree()?;
        let mut key = Vec::with_capacity(wf_id.len() + 1 + step_id.len());
        key.extend_from_slice(wf_id.as_bytes());
        key.push(0);
        key.extend_from_slice(step_id.as_bytes());
        match tree.get(key)? {
            Some(v) => Ok(Some(serde_json::from_slice(&v)?)),
            None => Ok(None),
        }
    }

    pub fn flush(&self) -> sled::Result<usize> {
        self.db.flush()
    }
}
