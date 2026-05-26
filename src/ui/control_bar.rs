// 하단 컨트롤 바: 메뉴 설치 / 제거 / 갱신 버튼 + 상태 표시.

use egui::{Color32, RichText};

use crate::app::AppState;
use crate::msgbox;
use crate::registry;

pub fn show(ui: &mut egui::Ui, state: &mut AppState) {
    ui.separator();
    ui.horizontal(|ui| {
        if ui.button("📥 메뉴 설치").clicked() {
            match registry::install() {
                Ok(_) => {
                    msgbox::info(
                        "컨텍스트 메뉴가 설치되었습니다.\n바탕화면이나 폴더 빈 공간에서 우클릭하여 확인하세요.",
                        "SnapLaunch",
                    );
                    state.refresh_install_state();
                    state.dirty_since_last_install = false;
                }
                Err(e) => {
                    msgbox::error(
                        &format!("메뉴 설치 중 오류가 발생했습니다.\n\n{}", e),
                        "SnapLaunch",
                    );
                }
            }
        }
        if ui.button("📤 메뉴 제거").clicked() {
            if msgbox::confirm(
                "컨텍스트 메뉴를 제거하시겠습니까?\n(등록된 툴 목록 파일은 그대로 유지됩니다.)",
                "SnapLaunch",
            ) {
                match registry::uninstall() {
                    Ok(_) => {
                        msgbox::info("컨텍스트 메뉴가 제거되었습니다.", "SnapLaunch");
                        state.refresh_install_state();
                    }
                    Err(e) => {
                        msgbox::error(
                            &format!("메뉴 제거 중 오류가 발생했습니다.\n\n{}", e),
                            "SnapLaunch",
                        );
                    }
                }
            }
        }
        let refresh_btn = if state.dirty_since_last_install {
            egui::Button::new(RichText::new("🔄 메뉴 갱신 (변경됨)").color(Color32::WHITE))
                .fill(Color32::from_rgb(40, 110, 200))
        } else {
            egui::Button::new("🔄 메뉴 갱신")
        };
        if ui.add(refresh_btn).clicked() {
            match registry::install() {
                Ok(_) => {
                    msgbox::info("컨텍스트 메뉴가 최신 상태로 갱신되었습니다.", "SnapLaunch");
                    state.refresh_install_state();
                    state.dirty_since_last_install = false;
                }
                Err(e) => {
                    msgbox::error(
                        &format!("메뉴 갱신 중 오류가 발생했습니다.\n\n{}", e),
                        "SnapLaunch",
                    );
                }
            }
        }
    });

    ui.add_space(6.0);

    // 상태 라벨
    let status_text = if state.installed {
        RichText::new("✅ 컨텍스트 메뉴가 설치되어 있습니다.")
            .color(Color32::from_rgb(20, 130, 60))
    } else {
        RichText::new("⚠ 컨텍스트 메뉴가 아직 설치되지 않았습니다. \"메뉴 설치\" 를 눌러 주세요.")
            .color(Color32::from_rgb(180, 100, 0))
    };
    ui.label(status_text);

    // EXE 경로 불일치 경고
    if let Some(reg_path) = &state.registered_exe_path {
        if let Some(cur_path) = &state.current_exe_path {
            if !paths_equal(reg_path, cur_path) {
                ui.label(
                    RichText::new(format!(
                        "⚠ 등록된 EXE 경로와 현재 위치가 다릅니다.\n  등록된 경로: {}\n  현재 위치   : {}\n  \"메뉴 갱신\" 을 눌러 재등록해 주세요.",
                        reg_path, cur_path
                    ))
                    .color(Color32::from_rgb(180, 60, 60)),
                );
            }
        }
    }
}

/// 대소문자/슬래시 방향 차이를 무시하고 경로를 비교한다.
fn paths_equal(a: &str, b: &str) -> bool {
    let norm = |s: &str| s.replace('/', "\\").to_lowercase();
    norm(a) == norm(b)
}
