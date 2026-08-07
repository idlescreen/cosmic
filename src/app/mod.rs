// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 IdleScreen

//! COSMIC panel applet for IdleScreen screensaver configuration.

mod helpers;
mod message;
mod state;
mod update;
mod view;

use cosmic::iced::window::Id;
use cosmic::prelude::*;

pub use message::Message;

/// Desktop / hicolor theme icon name (shipped as scalable SVG).
/// Panel button embeds the SVG; this name is for the `.desktop` `Icon=` field.
#[allow(dead_code)]
pub(crate) const ICON_NAME: &str = "io.github.idlescreen.CosmicApplet-symbolic";

#[derive(Default)]
pub struct AppModel {
    pub(crate) core: cosmic::Core,
    pub(crate) popup: Option<Id>,
    pub(crate) config: crate::config::Config,
    pub(crate) local_config: crate::config::ThemeConfig,
    pub(crate) screensavers: Vec<String>,
    pub(crate) daemon_running: bool,
    pub(crate) show_fps_overlay: bool,
    /// Advanced section expanded (render scale, FPS, daemon service).
    pub(crate) show_advanced: bool,
    /// Last user-visible error (cleared on successful actions / refresh).
    pub(crate) last_error: Option<String>,
}

impl cosmic::Application for AppModel {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = "io.github.idlescreen.CosmicApplet";

    fn core(&self) -> &cosmic::Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }

    fn init(
        core: cosmic::Core,
        _flags: Self::Flags,
    ) -> (Self, Task<cosmic::Action<Self::Message>>) {
        Self::init_app(core)
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn view(&self) -> Element<'_, Self::Message> {
        self.view_panel()
    }

    fn view_window(&self, id: Id) -> Element<'_, Self::Message> {
        self.view_popup(id)
    }

    fn subscription(&self) -> cosmic::iced::Subscription<Self::Message> {
        self.subscription_batch()
    }

    fn update(&mut self, message: Self::Message) -> Task<cosmic::Action<Self::Message>> {
        self.handle_update(message)
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}
