// SPDX-FileCopyrightText: 2026 Alex Brandt
//
// SPDX-License-Identifier: MIT

//! Zellij plugin for pairing with Claude Code.
//!
//! A layout loads one instance per surface, each selecting its surface with
//! the `surface` configuration key. Separate instances keep a busy picker from
//! stalling the widgets, and let each instance request only the permissions
//! its surface needs.

use std::collections::BTreeMap;

use zellij_tile::prelude::*;

const SURFACE_KEY: &str = "surface";
const SURFACE_NAMES: &str = "branch, pr, picker";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Surface {
    Branch,
    Pr,
    Picker,
}

impl Surface {
    fn from_configuration(configuration: &BTreeMap<String, String>) -> Result<Self, String> {
        match configuration.get(SURFACE_KEY).map(String::as_str) {
            Some("branch") => Ok(Self::Branch),
            Some("pr") => Ok(Self::Pr),
            Some("picker") => Ok(Self::Picker),
            Some(other) => Err(format!(
                "unknown {SURFACE_KEY} \"{other}\"; expected one of: {SURFACE_NAMES}"
            )),
            None => Err(format!(
                "missing {SURFACE_KEY} configuration; expected one of: {SURFACE_NAMES}"
            )),
        }
    }

    /// Widgets read the mode and theme to style themselves. The picker also
    /// runs `git` and `gh` to enumerate repositories and opens the pair tab.
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

#[derive(Default)]
enum State {
    #[default]
    Unloaded,
    Loaded(Surface),
    Misconfigured(String),
}

register_plugin!(State);

impl ZellijPlugin for State {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        *self = match Surface::from_configuration(&configuration) {
            Ok(surface) => {
                request_permission(surface.permissions());
                Self::Loaded(surface)
            }
            Err(message) => Self::Misconfigured(message),
        };
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        match self {
            Self::Unloaded => {}
            Self::Loaded(surface) => println!("{}", surface.name()),
            Self::Misconfigured(message) => println!("{message}"),
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
        for surface in [Surface::Branch, Surface::Pr, Surface::Picker] {
            assert_eq!(
                Surface::from_configuration(&configuration(surface.name())),
                Ok(surface)
            );
        }
    }

    #[test]
    fn rejects_an_unknown_surface() {
        let error = Surface::from_configuration(&configuration("status")).unwrap_err();
        assert!(error.contains("\"status\""), "{error}");
    }

    #[test]
    fn rejects_a_missing_surface() {
        let error = Surface::from_configuration(&BTreeMap::new()).unwrap_err();
        assert!(error.contains("missing"), "{error}");
    }

    #[test]
    fn widgets_request_nothing_only_the_picker_uses() {
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
