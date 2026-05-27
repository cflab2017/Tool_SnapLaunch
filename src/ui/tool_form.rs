// 새 툴 추가 폼 영역.
// 이름/경로/인자 입력 + 파일 선택 다이얼로그 + 추가 버튼.

use std::path::PathBuf;

use crate::app::AppState;
use crate::config::tools::file_stem_name;
use crate::i18n::Strings;
use crate::ui::style;

pub fn show(ui: &mut egui::Ui, state: &mut AppState) {
    let s = state.s();
    ui.separator();
    ui.heading(s.add_tool_heading);
    ui.add_space(4.0);

    egui::Grid::new("add_tool_grid")
        .num_columns(2)
        .spacing([8.0, 6.0])
        .show(ui, |ui| {
            ui.label(s.field_name).on_hover_text(s.tip_field_name);
            ui.text_edit_singleline(&mut state.form.name)
                .on_hover_text(s.tip_field_name);
            ui.end_row();

            ui.label(s.field_path).on_hover_text(s.tip_field_path);
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut state.form.path)
                        .desired_width(ui.available_width() - 110.0),
                )
                .on_hover_text(s.tip_field_path);
                if ui
                    .add(style::utility(s.btn_browse))
                    .on_hover_text(s.tip_browse)
                    .clicked()
                {
                    if let Some(picked) = pick_executable(s) {
                        let picked_str = picked.to_string_lossy().to_string();
                        if state.form.name.trim().is_empty() {
                            state.form.name = file_stem_name(&picked);
                        }
                        state.form.path = picked_str;
                    }
                }
            });
            ui.end_row();

            ui.label(s.field_args).on_hover_text(s.tip_field_args);
            ui.text_edit_singleline(&mut state.form.args)
                .on_hover_text(s.tip_field_args);
            ui.end_row();
        });

    ui.add_space(6.0);
    ui.horizontal(|ui| {
        let can_add = !state.form.name.trim().is_empty() && !state.form.path.trim().is_empty();
        if ui
            .add_enabled(can_add, style::primary(s.btn_add))
            .on_hover_text(s.tip_add)
            .clicked()
        {
            // 새로 생성된 툴의 ID 를 받아서 dirty 표시 대상으로 등록
            let new_id = state
                .config
                .add_tool(
                    state.form.name.trim().to_string(),
                    state.form.path.trim().to_string(),
                    state.form.args.trim().to_string(),
                )
                .id
                .clone();
            state.form.clear();
            state.mark_dirty(&[new_id.as_str()]);
        }
        if ui
            .add(style::warning(s.btn_clear))
            .on_hover_text(s.tip_clear_form)
            .clicked()
        {
            state.form.clear();
        }
        ui.label(
            egui::RichText::new(s.dnd_hint)
                .small()
                .color(egui::Color32::DARK_GRAY),
        );
    });
}

/// EXE 전용 네이티브 파일 선택 다이얼로그.
fn pick_executable(s: &Strings) -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter(s.file_dialog_filter_exe, &["exe", "bat", "cmd", "lnk"])
        .add_filter(s.file_dialog_filter_all, &["*"])
        .set_title(s.file_dialog_title)
        .pick_file()
}
