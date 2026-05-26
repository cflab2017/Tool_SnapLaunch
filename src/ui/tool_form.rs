// 새 툴 추가 폼 영역.
// 이름/경로/인자 입력 + 파일 선택 다이얼로그 + 추가 버튼.

use std::path::PathBuf;

use crate::app::AppState;
use crate::config::tools::file_stem_name;

pub fn show(ui: &mut egui::Ui, state: &mut AppState) {
    ui.separator();
    ui.heading("➕ 새 툴 추가");
    ui.add_space(4.0);

    egui::Grid::new("add_tool_grid")
        .num_columns(2)
        .spacing([8.0, 6.0])
        .show(ui, |ui| {
            ui.label("이름:");
            ui.text_edit_singleline(&mut state.form.name);
            ui.end_row();

            ui.label("경로:");
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut state.form.path)
                        .desired_width(ui.available_width() - 110.0),
                );
                if ui.button("📁 찾아보기").clicked() {
                    if let Some(picked) = pick_executable() {
                        let picked_str = picked.to_string_lossy().to_string();
                        // 이름이 비어 있으면 파일명을 자동 채움
                        if state.form.name.trim().is_empty() {
                            state.form.name = file_stem_name(&picked);
                        }
                        state.form.path = picked_str;
                    }
                }
            });
            ui.end_row();

            ui.label("인자:");
            ui.text_edit_singleline(&mut state.form.args);
            ui.end_row();
        });

    ui.add_space(6.0);
    ui.horizontal(|ui| {
        let can_add = !state.form.name.trim().is_empty() && !state.form.path.trim().is_empty();
        if ui
            .add_enabled(can_add, egui::Button::new("➕ 추가"))
            .clicked()
        {
            state.config.add_tool(
                state.form.name.trim().to_string(),
                state.form.path.trim().to_string(),
                state.form.args.trim().to_string(),
            );
            state.form.clear();
            state.mark_dirty();
        }
        if ui.button("입력 초기화").clicked() {
            state.form.clear();
        }
        ui.label(
            egui::RichText::new("(EXE 파일을 창에 끌어다 놓으면 자동으로 채워집니다)")
                .small()
                .color(egui::Color32::DARK_GRAY),
        );
    });
}

/// EXE 전용 네이티브 파일 선택 다이얼로그.
fn pick_executable() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("실행 파일", &["exe", "bat", "cmd", "lnk"])
        .add_filter("모든 파일", &["*"])
        .set_title("실행할 프로그램 선택")
        .pick_file()
}
