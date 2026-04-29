//! Optimization algorithms used by the keyboard layout optimizer.
//!
//! This module contains optimization routines and cost-function related types.
//! The main algorithm currently implemented is simulated annealing, which
//! searches over keyboard layout permutations by swapping pairs of keys.

pub mod cost;
pub mod sa;
