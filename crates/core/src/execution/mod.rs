//! Cross-chain settlement routing and execution planning
//!
//! This module provides the types and logic for decomposing high-level agent intents
//! into multi-leg execution plans that span multiple chains, selecting optimal routes,
//! and managing compensating actions when partial failures occur

pub mod atomicity;
pub mod decomposer;
pub mod plan;
pub mod router;

pub use atomicity::{AtomicityManager, CompensationStatus};
pub use decomposer::IntentDecomposer;
pub use plan::{
    Action, ActionType, CompensatingAction, Constraint, ConstraintType, ExecutionPlan, Intent, Leg,
    LegStatus, PlanStatus,
};
pub use router::{ChainMetrics, RouteSelector, RouteWeights};
