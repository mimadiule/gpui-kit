//! M0 skeleton: a four-channel oscilloscope shell — dock layout, channel
//! sidebar, top toolbar, measure dock, dark theme. The waveform canvas and
//! the instrument state arrive in M1+.

mod panels;

use gpui_kit::assets::{Assets, IconName};
use gpui_kit::component::{
    ActiveTheme as _, Sizable as _, Theme, ThemeMode, TitleBar,
    button::{Button, ButtonVariants as _},
    dock::{DockArea, DockLayout, DockPlacement, DockSkin, panel_handle},
    status_bar::StatusBar,
    toolbar::{Toolbar, ToolbarGroup},
};
use gpui_kit::*;
use panels::{ChannelsPanel, MeasurePanel, WaveformPanel};

const DOCK_AREA_ID: &str = "oscilloscope-main";

struct OscilloscopeApp {
    dock_area: Entity<DockArea>,
}

impl OscilloscopeApp {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let (dock_area, _skin) = DockSkin::dock_area(DOCK_AREA_ID, None, window, cx);

        let center = DockLayout::tabs()
            .panel_view(panel_handle(cx.new(WaveformPanel::new)), cx);
        let left = DockLayout::tabs()
            .panel_view(panel_handle(cx.new(ChannelsPanel::new)), cx);
        let bottom = DockLayout::tabs()
            .panel_view(panel_handle(cx.new(MeasurePanel::new)), cx);

        dock_area.update(cx, |area, cx| {
            area.set_center(center, window, cx);
            area.set_dock(DockPlacement::Left, left, window, cx);
            area.set_dock_size(DockPlacement::Left, px(240.), window, cx);
            area.set_dock(DockPlacement::Bottom, bottom, window, cx);
            area.set_dock_size(DockPlacement::Bottom, px(180.), window, cx);
            for placement in [DockPlacement::Left, DockPlacement::Bottom] {
                area.set_dock_collapsible(placement, true, window, cx);
            }
        });

        Self { dock_area }
    }

    fn title_bar(&self) -> impl IntoElement {
        let toolbar = Toolbar::new("osc-toolbar")
            .child(
                ToolbarGroup::new("run-group").label("Run").children([
                    Button::new("run")
                        .label("Run")
                        .icon(IconName::Play)
                        .small()
                        .ghost(),
                    Button::new("stop")
                        .label("Stop")
                        .icon(IconName::Pause)
                        .small()
                        .ghost(),
                    Button::new("single").label("Single").small().ghost(),
                ]),
            )
            .child(
                ToolbarGroup::new("trigger-group").label("Trigger").children([
                    Button::new("trigger-mode")
                        .label("Edge")
                        .icon(IconName::Activity)
                        .small()
                        .ghost(),
                    Button::new("trigger-setup")
                        .icon(IconName::SlidersHorizontal)
                        .small()
                        .ghost(),
                ]),
            );

        TitleBar::new()
            .child(
                div().flex().items_center().child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .child(IconName::Activity)
                        .child("Oscilloscope"),
                ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_end()
                    .px_2()
                    .child(toolbar),
            )
    }

    fn status_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        StatusBar::new()
            .left(
                Button::new("toggle-left-dock")
                    .ghost()
                    .xsmall()
                    .icon(IconName::PanelLeft)
                    .tooltip("Toggle Channels")
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.dock_area.update(cx, |area, cx| {
                            area.toggle_dock(DockPlacement::Left, window, cx)
                        });
                    })),
            )
            .left(
                Button::new("toggle-bottom-dock")
                    .ghost()
                    .xsmall()
                    .icon(IconName::PanelBottom)
                    .tooltip("Toggle Measurements")
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.dock_area.update(cx, |area, cx| {
                            area.toggle_dock(DockPlacement::Bottom, window, cx)
                        });
                    })),
            )
            .right(
                div()
                    .flex()
                    .items_center()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child("Stopped  |  M0 skeleton"),
            )
    }
}

impl Render for OscilloscopeApp {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(self.title_bar())
            .child(div().flex_1().min_h_0().child(self.dock_area.clone()))
            .child(self.status_bar(cx))
    }
}

fn main() {
    let app = gpui_kit::application().with_assets(Assets);

    app.run(move |cx| {
        gpui_kit::init(cx);

        // Dark theme for the instrument look; channel colors layer on top of
        // it in later milestones.
        Theme::change(ThemeMode::Dark, None, cx);
        cx.activate(true);

        // Default sizing: a fixed requested bounds sized itself against the
        // display in physical pixels on scaled-DPI Windows machines and ended
        // up wider than the screen (also observed with the story example).
        let options = WindowOptions {
            kind: WindowKind::Normal,
            ..TitleBar::window_options()
        };

        let (handle, _) = gpui_kit::open_window(options, cx, |window, cx| {
            cx.new(|cx| OscilloscopeApp::new(window, cx))
        })
        .expect("failed to open oscilloscope window");

        handle
            .update(cx, |_, window, _| {
                window.activate_window();
                window.set_window_title("Oscilloscope");
            })
            .expect("failed to set window title");
    });
}
