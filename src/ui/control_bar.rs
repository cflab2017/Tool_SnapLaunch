// 하단 컨트롤 바: 메뉴 설치 / 제거 / 갱신 버튼 + 상태 표시.

use egui::{Color32, RichText};

use crate::app::AppState;
use crate::msgbox;
use crate::registry;

pub fn show(ui: &mut egui::Ui, state: &mut AppState) {
    let s = state.s();
    ui.separator();
    ui.horizontal(|ui| {
        if ui.button(s.btn_install).clicked() {
            match registry::install() {
                Ok(_) => {
                    msgbox::info(s.install_success, "SnapLaunch");
                    state.refresh_install_state();
                    state.dirty_since_last_install = false;
                }
                Err(e) => {
                    msgbox::error(
                        &format!("{}{}", s.install_error_prefix, e),
                        "SnapLaunch",
                    );
                }
            }
        }
        if ui.button(s.btn_uninstall).clicked() {
            if msgbox::confirm(s.uninstall_confirm, "SnapLaunch") {
                match registry::uninstall() {
                    Ok(_) => {
                        msgbox::info(s.uninstall_success, "SnapLaunch");
                        state.refresh_install_state();
                    }
                    Err(e) => {
                        msgbox::error(
                            &format!("{}{}", s.uninstall_error_prefix, e),
                            "SnapLaunch",
                        );
                    }
                }
            }
        }
    });

    ui.add_space(6.0);

    // 상태 라벨
    let status_text = if state.installed {
        RichText::new(s.status_installed).color(Color32::from_rgb(20, 130, 60))
    } else {
        RichText::new(s.status_not_installed).color(Color32::from_rgb(180, 100, 0))
    };
    ui.label(status_text);

    // EXE 경로 불일치 경고
    if let Some(reg_path) = &state.registered_exe_path {
        if let Some(cur_path) = &state.current_exe_path {
            if !paths_equal(reg_path, cur_path) {
                let warn = s
                    .path_mismatch_template
                    .replace("{old}", reg_path)
                    .replace("{new}", cur_path);
                ui.label(RichText::new(warn).color(Color32::from_rgb(180, 60, 60)));
            }
        }
    }
}

/// 대소문자/슬래시 방향 차이를 무시하고 경로를 비교한다.
fn paths_equal(a: &str, b: &str) -> bool {
    let norm = |s: &str| s.replace('/', "\\").to_lowercase();
    norm(a) == norm(b)
}

/// "메뉴 갱신" 버튼만 별도로 렌더링한다. 툴 목록의 위/아래 버튼 옆에 배치되어,
/// 사용자가 순서를 바꾼 직후 곧바로 메뉴를 동기화할 수 있게 한다.
/// dirty 상태일 때는 파란 강조 스타일이 적용된다.
pub fn refresh_button(ui: &mut egui::Ui, state: &mut AppState) {
    let s = state.s();
    let btn = if state.dirty_since_last_install {
        egui::Button::new(RichText::new(s.btn_refresh_dirty).color(Color32::WHITE))
            .fill(Color32::from_rgb(40, 110, 200))
    } else {
        egui::Button::new(s.btn_refresh)
    };
    if ui.add(btn).clicked() {
        match registry::install() {
            Ok(_) => {
                msgbox::info(s.refresh_success, "SnapLaunch");
                state.refresh_install_state();
                state.dirty_since_last_install = false;
            }
            Err(e) => {
                msgbox::error(
                    &format!("{}{}", s.refresh_error_prefix, e),
                    "SnapLaunch",
                );
            }
        }
    }
}
