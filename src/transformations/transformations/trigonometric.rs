//! Trigonometric Transformations

use crate::core::*;
use crate::transformations::{matches, rules};
use lazy_static::lazy_static;

lazy_static! {
    /// Provides common trigonometric identities.
    pub static ref TRIGONOMETRIC_IDENTITIES: Transformation = Transformation {
        rules: vec![
            // cos(0) = 1
            (|e| matches::first("cos", 0.into())(e), rules::ONE),
            // sec(0) = 1
            (|e| matches::first("sec", 0.into())(e), rules::ONE),
            // sin(0) = 0
            (|e| matches::first("sin", 0.into())(e), rules::ZERO),
            // tan(0) = 0
            (|e| matches::first("tan", 0.into())(e), rules::ZERO),
        ]
    };
}
