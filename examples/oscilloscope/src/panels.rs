//! M0 placeholder panels. Each is a dock `Panel` (presentation trait of
//! gpui-kit) with a stable `panel_name`; their contents get replaced by the
//! canvas (M1), channel controls (M3), and live measurements (M3/M4).

use gpui_kit::assets::IconName;
use gpui_kit::component::{
    ActiveTheme as _,
    dock::{BasePanel, Panel, PanelEvent},
    sidebar::{Sidebar, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem},
};
use gpui_kit::*;

// ---------------------------------------------------------------- waveform

pub struct WaveformPanel {
    focus_handle: FocusHandle,
}

impl WaveformPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Focusable for WaveformPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for WaveformPanel {}

impl BasePanel for WaveformPanel {
    fn panel_name(&self) -> &'static str {
        "waveform"
    }
}

impl Panel for WaveformPanel {
    fn title(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        "Waveform"
    }
}

impl Render for WaveformPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .bg(cx.theme().background)
            .text_color(cx.theme().muted_foreground)
            .child("Waveform canvas — arrives in M1")
    }
}

// ---------------------------------------------------------------- channels

pub struct ChannelsPanel {
    focus_handle: FocusHandle,
}

impl ChannelsPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Focusable for ChannelsPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for ChannelsPanel {}

impl BasePanel for ChannelsPanel {
    fn panel_name(&self) -> &'static str {
        "channels"
    }
}

impl Panel for ChannelsPanel {
    fn title(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        "Channels"
    }
}

impl Render for ChannelsPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let menu = SidebarMenu::new().children([
            SidebarMenuItem::new("CH1").icon(IconName::Circle),
            SidebarMenuItem::new("CH2").icon(IconName::Circle),
            SidebarMenuItem::new("CH3").icon(IconName::Circle),
            SidebarMenuItem::new("CH4").icon(IconName::Circle),
        ]);

        div().size_full().bg(cx.theme().sidebar).child(
            Sidebar::new("channel-sidebar")
                .w_full()
                .header(
                    SidebarHeader::new().child(
                        div()
                            .flex()
                            .items_center()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .child(IconName::Activity)
                            .child("Oscilloscope"),
                    ),
                )
                .child(SidebarGroup::new("Channels").child(menu))
                .footer(SidebarFooter::new().child("4 channels  |  M0 shell")),
        )
    }
}

// -------------------------------------------------------------- measurements

pub struct MeasurePanel {
    focus_handle: FocusHandle,
}

impl MeasurePanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Focusable for MeasurePanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for MeasurePanel {}

impl BasePanel for MeasurePanel {
    fn panel_name(&self) -> &'static str {
        "measure"
    }
}

impl Panel for MeasurePanel {
    fn title(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        "Measurements"
    }
}

fn readout(label: &'static str, cx: &App) -> Div {
    div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .gap_1()
        .rounded_sm()
        .border_1()
        .border_color(cx.theme().border)
        .bg(cx.theme().secondary)
        .px_2()
        .py_1()
        .child(
            div()
                .text_xs()
                .text_color(cx.theme().muted_foreground)
                .child(label),
        )
        .child(
            div()
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
                .child("--"),
        )
}

impl Render for MeasurePanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .gap_2()
            .p_2()
            .bg(cx.theme().background)
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(readout("CH1  Vpp", cx))
                    .child(readout("CH1  Vrms", cx))
                    .child(readout("CH1  Freq", cx))
                    .child(readout("CH1  Period", cx)),
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(readout("CH2  Vpp", cx))
                    .child(readout("CH2  Vrms", cx))
                    .child(readout("CH2  Freq", cx))
                    .child(readout("CH2  Period", cx)),
            )
    }
}
