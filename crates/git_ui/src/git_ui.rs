use ::settings::Settings;
use feature_flags::WaitForFlag;
use futures::{select_biased, FutureExt};
use git::status::FileStatus;
use git_panel_settings::GitPanelSettings;
use gpui::{AppContext, Hsla};
use ui::{Color, Icon, IconName, IntoElement};

pub mod git_panel;
mod git_panel_settings;
pub mod repository_selector;

pub fn init(cx: &mut AppContext) {
    GitPanelSettings::register(cx);
}

// TODO: Remove this before launching Git UI
pub async fn git_ui_enabled(flag: WaitForFlag) -> bool {
    let mut git_ui_feature_flag = flag.fuse();
    let mut timeout = FutureExt::fuse(smol::Timer::after(std::time::Duration::from_secs(5)));

    select_biased! {
        is_git_ui_enabled = git_ui_feature_flag => is_git_ui_enabled,
        _ = timeout => false,
    }
}

const ADDED_COLOR: Hsla = Hsla {
    h: 142. / 360.,
    s: 0.68,
    l: 0.45,
    a: 1.0,
};
const MODIFIED_COLOR: Hsla = Hsla {
    h: 48. / 360.,
    s: 0.76,
    l: 0.47,
    a: 1.0,
};
const REMOVED_COLOR: Hsla = Hsla {
    h: 355. / 360.,
    s: 0.65,
    l: 0.65,
    a: 1.0,
};

// TODO: Add updated status colors to theme
pub fn git_status_icon(status: FileStatus) -> impl IntoElement {
    let (icon_name, color) = if status.is_conflicted() {
        (IconName::Warning, REMOVED_COLOR)
    } else if status.is_deleted() {
        (IconName::SquareMinus, REMOVED_COLOR)
    } else if status.is_modified() {
        (IconName::SquareDot, MODIFIED_COLOR)
    } else {
        (IconName::SquarePlus, ADDED_COLOR)
    };
    Icon::new(icon_name).color(Color::Custom(color))
}
