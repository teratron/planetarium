//! # Loading Screen Components
//!
//! Marker components for the loading screen entities.

use bevy::prelude::*;

/// Marker for the loading screen root node.
#[derive(Component)]
pub struct LoadingRoot;

/// Marker for the bar fill entity to update its width.
#[derive(Component)]
pub struct ProgressBarFill;

/// Marker for the percentage text entity.
#[derive(Component)]
pub struct LoadingPercentText;

/// Marker for the active asset group text entity.
#[derive(Component)]
pub struct LoadingAssetText;

/// Marker for the text entity to update loading hints.
#[derive(Component)]
pub struct LoadingHintText;
