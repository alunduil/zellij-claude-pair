// SPDX-FileCopyrightText: 2026 Alex Brandt
//
// SPDX-License-Identifier: MIT

//! Zellij plugin for pairing with Claude Code.
//!
//! A layout loads one instance per surface, each selecting its surface with
//! the `surface` configuration key. Separate instances keep a busy picker from
//! stalling the widgets, and let each instance request only the permissions
//! its surface needs.

use std::{collections::BTreeMap, fmt};

use zellij_tile::prelude::*;

const SURFACE_KEY: &str = "surface";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Surface {
    Branch,
    Pr,
    Picker,
}

impl Surface {
    const ALL: [Self; 3] = [Self::Branch, Self::Pr, Self::Picker];

    fn from_configuration(
        configuration: &BTreeMap<String, String>,
    ) -> Result<Self, ConfigurationError> {
        let name = configuration
            .get(SURFACE_KEY)
            .ok_or(ConfigurationError::MissingSurface)?;
        Self::ALL
            .into_iter()
            .find(|surface| surface.name() == name)
            .ok_or_else(|| ConfigurationError::UnknownSurface(name.clone()))
    }

    /// Widgets need the mode and theme to style themselves. The picker also
    /// needs to run `git` and `gh` and to open the pair tab.
    fn permissions(self) -> &'static [PermissionType] {
        match self {
            Self::Branch | Self::Pr => &[PermissionType::ReadApplicationState],
            Self::Picker => &[
                PermissionType::ReadApplicationState,
                PermissionType::RunCommands,
                PermissionType::ChangeApplicationState,
            ],
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Branch => "branch",
            Self::Pr => "pr",
            Self::Picker => "picker",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ConfigurationError {
    MissingSurface,
    UnknownSurface(String),
}

impl fmt::Display for ConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingSurface => write!(f, "missing {SURFACE_KEY} configuration")?,
            Self::UnknownSurface(name) => write!(f, "unknown {SURFACE_KEY} \"{name}\"")?,
        }
        let names: Vec<_> = Surface::ALL.iter().map(|surface| surface.name()).collect();
        write!(f, "; expected one of: {}", names.join(", "))
    }
}

#[derive(Default)]
enum State {
    #[default]
    Unloaded,
    Loaded(Surface),
    Misconfigured(ConfigurationError),
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        *self = match Surface::from_configuration(&configuration) {
            Ok(surface) => {
                request_permission(surface.permissions());
                Self::Loaded(surface)
            }
            Err(error) => Self::Misconfigured(error),
        };
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        match self {
            Self::Unloaded => {}
            Self::Loaded(surface) => println!("{}", surface.name()),
            Self::Misconfigured(error) => println!("{error}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn configuration(surface: &str) -> BTreeMap<String, String> {
        BTreeMap::from([(SURFACE_KEY.to_owned(), surface.to_owned())])
    }

    #[test]
    fn selects_each_surface_by_name() {
        for (name, surface) in [
            ("branch", Surface::Branch),
            ("pr", Surface::Pr),
            ("picker", Surface::Picker),
        ] {
            assert_eq!(
                Surface::from_configuration(&configuration(name)),
                Ok(surface)
            );
        }
    }

    #[test]
    fn rejects_an_unknown_surface() {
        assert_eq!(
            Surface::from_configuration(&configuration("status")),
            Err(ConfigurationError::UnknownSurface("status".to_owned()))
        );
    }

    #[test]
    fn rejects_a_missing_surface() {
        assert_eq!(
            Surface::from_configuration(&BTreeMap::new()),
            Err(ConfigurationError::MissingSurface)
        );
    }

    #[test]
    fn widgets_request_no_picker_only_permission() {
        let picker_only = [
            PermissionType::RunCommands,
            PermissionType::ChangeApplicationState,
        ];
        for widget in [Surface::Branch, Surface::Pr] {
            assert!(
                widget
                    .permissions()
                    .iter()
                    .all(|permission| !picker_only.contains(permission)),
                "{widget:?} requests a picker-only permission"
            );
        }
    }
}
