//! Route selection logic for picking the best chain based on metrics

use crate::transaction::Chain;
use std::collections::HashMap;

/// Performance and cost metrics for a given chain
#[derive(Debug, Clone)]
pub struct ChainMetrics {
    /// Average cost per transaction in USD
    pub cost_per_tx: f64,
    /// Average confirmation latency in milliseconds
    pub latency_ms: u64,
    /// Historical success rate from 0.0 to 1.0
    pub reliability_score: f64,
    /// Current network congestion from 0.0 (empty) to 1.0 (saturated)
    pub congestion_level: f64,
}

impl Default for ChainMetrics {
    fn default() -> Self {
        Self {
            cost_per_tx: 0.01,
            latency_ms: 500,
            reliability_score: 0.95,
            congestion_level: 0.1,
        }
    }
}

/// Weight configuration for the multi-factor chain scoring formula
#[derive(Debug, Clone)]
pub struct RouteWeights {
    pub cost: f64,
    pub latency: f64,
    pub reliability: f64,
    pub congestion: f64,
}

impl Default for RouteWeights {
    fn default() -> Self {
        Self {
            cost: 0.3,
            latency: 0.2,
            reliability: 0.3,
            congestion: 0.2,
        }
    }
}

/// Selects the best execution chain from a set of candidates using weighted scoring
#[derive(Debug, Clone)]
pub struct RouteSelector {
    pub metrics: HashMap<Chain, ChainMetrics>,
    pub weights: RouteWeights,
}

impl Default for RouteSelector {
    fn default() -> Self {
        Self::new()
    }
}

impl RouteSelector {
    /// Creates an empty selector with default weights
    pub fn new() -> Self {
        Self {
            metrics: HashMap::new(),
            weights: RouteWeights::default(),
        }
    }

    /// Overrides the default scoring weights
    pub fn with_weights(mut self, weights: RouteWeights) -> Self {
        self.weights = weights;
        self
    }

    /// Registers metrics for a chain
    pub fn add_chain(&mut self, chain: Chain, metrics: ChainMetrics) {
        self.metrics.insert(chain, metrics);
    }

    /// Computes a single score for a chain using the configured weights
    ///
    /// Each factor is normalized to roughly 0.0..1.0 using a sigmoid-like transform
    /// so that extreme values do not dominate the final score
    fn score_chain(&self, m: &ChainMetrics) -> f64 {
        let cost_score = 1.0 - (m.cost_per_tx / (m.cost_per_tx + 1.0));
        let latency_score = 1.0 - (m.latency_ms as f64 / (m.latency_ms as f64 + 1000.0));
        let reliability_score = m.reliability_score;
        let congestion_score = 1.0 - m.congestion_level;

        cost_score * self.weights.cost
            + latency_score * self.weights.latency
            + reliability_score * self.weights.reliability
            + congestion_score * self.weights.congestion
    }

    /// Picks the single best chain based on weighted scoring
    ///
    /// Returns None when no chains have been registered
    pub fn select_chain(&self) -> Option<Chain> {
        self.metrics
            .iter()
            .map(|(chain, m)| (chain, self.score_chain(m)))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(chain, _)| *chain)
    }

    /// Returns all registered chains ranked from best to worst with their scores
    pub fn suggest_routes(&self) -> Vec<(Chain, f64)> {
        let mut scored: Vec<(Chain, f64)> = self
            .metrics
            .iter()
            .map(|(chain, m)| (*chain, self.score_chain(m)))
            .collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_chain_picks_best() {
        let mut sel = RouteSelector::new();
        sel.add_chain(
            Chain::Ethereum,
            ChainMetrics {
                cost_per_tx: 5.0,
                latency_ms: 12000,
                reliability_score: 0.99,
                congestion_level: 0.7,
            },
        );
        sel.add_chain(
            Chain::Base,
            ChainMetrics {
                cost_per_tx: 0.01,
                latency_ms: 2000,
                reliability_score: 0.97,
                congestion_level: 0.1,
            },
        );
        let best = sel.select_chain().unwrap();
        assert_eq!(best, Chain::Base);
    }

    #[test]
    fn test_suggest_routes_ranked() {
        let mut sel = RouteSelector::new();
        sel.add_chain(
            Chain::Polygon,
            ChainMetrics {
                cost_per_tx: 0.005,
                latency_ms: 2200,
                reliability_score: 0.93,
                congestion_level: 0.3,
            },
        );
        sel.add_chain(
            Chain::Arbitrum,
            ChainMetrics {
                cost_per_tx: 0.02,
                latency_ms: 1500,
                reliability_score: 0.98,
                congestion_level: 0.15,
            },
        );
        sel.add_chain(
            Chain::Ethereum,
            ChainMetrics {
                cost_per_tx: 8.0,
                latency_ms: 12000,
                reliability_score: 0.99,
                congestion_level: 0.8,
            },
        );
        let routes = sel.suggest_routes();
        assert_eq!(routes.len(), 3);
        assert_eq!(routes[0].0, Chain::Arbitrum);
        assert!(routes[0].1 > routes[1].1);
        assert!(routes[1].1 > routes[2].1);
    }

    #[test]
    fn test_custom_weights_affect_ranking() {
        let mut sel = RouteSelector::new().with_weights(RouteWeights {
            cost: 0.9,
            latency: 0.0,
            reliability: 0.0,
            congestion: 0.1,
        });
        sel.add_chain(
            Chain::Solana,
            ChainMetrics {
                cost_per_tx: 0.001,
                latency_ms: 400,
                reliability_score: 0.90,
                congestion_level: 0.2,
            },
        );
        sel.add_chain(
            Chain::Ethereum,
            ChainMetrics {
                cost_per_tx: 10.0,
                latency_ms: 12000,
                reliability_score: 0.99,
                congestion_level: 0.8,
            },
        );
        let best = sel.select_chain().unwrap();
        assert_eq!(best, Chain::Solana);
    }
}
