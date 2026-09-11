// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 IdleScreen

//! On-disk theme/settings config shared with the IdleScreen daemon.
//! Prefer `~/.config/idle/config.yaml`; fall back to legacy `~/.config/trance/`.

use cosmic::cosmic_config::{self, CosmicConfigEntry, cosmic_config_derive::CosmicConfigEntry};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, CosmicConfigEntry, Eq, PartialEq)]
#[version = 1]
pub struct Config {
    demo: String,
}

#[derive(Debug, Clone, Default)]
pub struct ThemeConfig {
    pub accent_color: String,
    pub idle_timeout_mins: u32,
    pub theme_idx: usize,
    pub active_saver: Option<String>,
    pub idle_enabled: bool,
    pub show_fps_overlay: bool,
    pub render_scale: f32,
}

impl ThemeConfig {
    /// Defaults used when no on-disk config is present.
    pub fn defaults() -> Self {
        Self {
            accent_color: "#00BFFF".to_string(),
            idle_timeout_mins: 5,
            theme_idx: 0,
            active_saver: Some("beams".to_string()),
            idle_enabled: true,
            show_fps_overlay: false,
            render_scale: 1.0,
        }
    }

    /// Write path and primary read path: `~/.config/idle/config.yaml`.
    pub fn get_config_path() -> Option<PathBuf> {
        Self::config_path_candidates().into_iter().next()
    }

    /// Idle first, legacy `trance` second (matches idle-daemon).
    pub fn config_path_candidates() -> Vec<PathBuf> {
        let mut bases = Vec::new();
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME")
            && !xdg.is_empty()
        {
            bases.push(PathBuf::from(xdg));
        }
        if let Ok(home) = std::env::var("HOME") {
            bases.push(PathBuf::from(home).join(".config"));
        }
        let mut out = Vec::new();
        for base in bases {
            out.push(base.join("idle").join("config.yaml"));
            out.push(base.join("trance").join("config.yaml"));
        }
        out
    }

    /// Apply a single `key: value` line (YAML-ish) onto `config`.
    ///
    /// Unknown keys and malformed values are ignored so partial or hand-edited
    /// files still load.
    pub fn apply_yaml_line(config: &mut Self, line: &str) {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return;
        }
        let Some(idx) = line.find(':') else {
            return;
        };
        let key = line[..idx].trim();
        let val = line[idx + 1..].trim().trim_matches('"').trim_matches('\'');
        match key {
            "accent_color" => {
                config.accent_color = val.to_string();
            }
            "idle_timeout_mins" => {
                if let Ok(n) = val.parse::<u32>() {
                    config.idle_timeout_mins = n;
                }
            }
            "theme_idx" => {
                if let Ok(idx) = val.parse::<usize>() {
                    config.theme_idx = idx;
                }
            }
            "active_saver" => {
                if !val.is_empty() && val != "none" {
                    config.active_saver = Some(val.to_string());
                } else {
                    config.active_saver = None;
                }
            }
            "idle_enabled" => {
                if let Ok(b) = val.parse::<bool>() {
                    config.idle_enabled = b;
                }
            }
            "show_fps_overlay" => {
                if let Ok(b) = val.parse::<bool>() {
                    config.show_fps_overlay = b;
                }
            }
            "render_scale" => {
                if let Ok(s) = val.parse::<f32>() {
                    config.render_scale = s;
                }
            }
            _ => {}
        }
    }

    /// Parse multi-line YAML-ish content into a [`ThemeConfig`].
    pub fn from_yaml_content(content: &str) -> Self {
        let mut config = Self::defaults();
        for line in content.lines() {
            Self::apply_yaml_line(&mut config, line);
        }
        config
    }

    pub fn load() -> Self {
        for path in Self::config_path_candidates() {
            if let Ok(content) = fs::read_to_string(&path) {
                return Self::from_yaml_content(&content);
            }
        }
        Self::defaults()
    }

    pub fn save(&self) -> std::io::Result<()> {
        // Always write to primary idle path so settings converge with the daemon.
        if let Some(path) = Self::get_config_path() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let active_str = self.active_saver.as_deref().unwrap_or("none");
            let content = format!(
                "# IdleScreen themes and settings\n\
                 accent_color: \"{}\"\n\
                 # dark_mode is auto-detected from system\n\
                 idle_timeout_mins: {}\n\
                 theme_idx: {}\n\
                 active_saver: \"{}\"\n\
                 idle_enabled: {}\n\
                 show_fps_overlay: {}\n\
                 render_scale: {}\n",
                self.accent_color,
                self.idle_timeout_mins,
                self.theme_idx,
                active_str,
                self.idle_enabled,
                self.show_fps_overlay,
                self.render_scale
            );
            fs::write(&path, content)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::ThemeConfig;

    #[test]
    fn defaults_are_sane() {
        let d = ThemeConfig::defaults();
        assert_eq!(d.idle_timeout_mins, 5);
        assert_eq!(d.active_saver.as_deref(), Some("beams"));
        assert!(d.idle_enabled);
        assert!((d.render_scale - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn parses_known_keys() {
        let yaml = r##"
# comment
accent_color: "#FF0000"
idle_timeout_mins: 15
theme_idx: 2
active_saver: "cosmos"
idle_enabled: false
show_fps_overlay: true
render_scale: 0.5
unknown_key: ignored
"##;
        let c = ThemeConfig::from_yaml_content(yaml);
        assert_eq!(c.accent_color, "#FF0000");
        assert_eq!(c.idle_timeout_mins, 15);
        assert_eq!(c.theme_idx, 2);
        assert_eq!(c.active_saver.as_deref(), Some("cosmos"));
        assert!(!c.idle_enabled);
        assert!(c.show_fps_overlay);
        assert!((c.render_scale - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn active_saver_none_clears() {
        let c = ThemeConfig::from_yaml_content("active_saver: none\n");
        assert_eq!(c.active_saver, None);
        let c2 = ThemeConfig::from_yaml_content("active_saver: \"\"\n");
        assert_eq!(c2.active_saver, None);
    }

    #[test]
    fn bad_values_keep_defaults() {
        let c = ThemeConfig::from_yaml_content(
            "idle_timeout_mins: not-a-number\nrender_scale: xyz\nidle_enabled: maybe\n",
        );
        let d = ThemeConfig::defaults();
        assert_eq!(c.idle_timeout_mins, d.idle_timeout_mins);
        assert!((c.render_scale - d.render_scale).abs() < f32::EPSILON);
        assert_eq!(c.idle_enabled, d.idle_enabled);
    }

    #[test]
    fn quoted_and_unquoted_values() {
        let c = ThemeConfig::from_yaml_content("active_saver: 'storm'\naccent_color: #00FF00\n");
        assert_eq!(c.active_saver.as_deref(), Some("storm"));
        assert_eq!(c.accent_color, "#00FF00");
    }
}
