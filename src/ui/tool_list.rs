// 등록된 툴 목록을 표 형태로 표시한다.
// 각 행은 인라인 편집/삭제 버튼을 가지며 선택 후 위/아래 버튼으로 순서를 바꿀 수 있다.

use egui::{Color32, Grid, RichText, ScrollArea};

use crate::app::{AppState, EditingBuffer};
use crate::path_util::tool_exists;

/// 죽은(경로가 유효하지 않은) 툴 이름 표시에 사용할 색상
const DEAD_COLOR: Color32 = Color32::from_rgb(200, 60, 60);

/// 툴 목록 영역을 그린다.
pub fn show(ui: &mut egui::Ui, state: &mut AppState) {
    ui.heading("등록된 툴 목록");
    ui.add_space(4.0);

    if state.config.tools.is_empty() {
        ui.label(
            RichText::new(
                "아직 등록된 툴이 없습니다. 아래 \"새 툴 추가\" 영역에서 추가하거나, EXE 파일을 창에 끌어다 놓으세요.",
            )
            .italics()
            .color(egui::Color32::DARK_GRAY),
        );
        ui.add_space(8.0);
        return;
    }

    ScrollArea::vertical()
        .max_height(260.0)
        .id_salt("tool_list_scroll")
        .show(ui, |ui| {
            Grid::new("tool_list_grid")
                .num_columns(5)
                .striped(true)
                .spacing([8.0, 6.0])
                .show(ui, |ui| {
                    // 헤더
                    ui.label(RichText::new("순서").strong());
                    ui.label(RichText::new("이름").strong());
                    ui.label(RichText::new("경로").strong());
                    ui.label(RichText::new("인자").strong());
                    ui.label(RichText::new("작업").strong());
                    ui.end_row();

                    // 순서대로 표시되도록 미리 정렬
                    state.config.tools.sort_by_key(|t| t.order);

                    let to_delete: Option<String> = None;
                    let mut to_select: Option<String> = None;
                    let mut to_save_edit: Option<String> = None;
                    let mut to_cancel_edit = false;

                    let tools_snapshot: Vec<(String, String, String, String, u32)> = state
                        .config
                        .tools
                        .iter()
                        .map(|t| (t.id.clone(), t.name.clone(), t.path.clone(), t.args.clone(), t.order))
                        .collect();

                    for (id, name, path, args, order) in tools_snapshot {
                        let is_editing = state
                            .editing
                            .as_ref()
                            .map(|e| e.id == id)
                            .unwrap_or(false);
                        let is_selected = state.selected_id.as_deref() == Some(id.as_str());

                        // 순서 컬럼 (선택 시 강조)
                        let order_label = RichText::new(format!("{}", order));
                        let order_label = if is_selected {
                            order_label.strong().color(egui::Color32::from_rgb(40, 110, 200))
                        } else {
                            order_label
                        };
                        if ui.selectable_label(is_selected, order_label).clicked() {
                            to_select = Some(id.clone());
                        }

                        if is_editing {
                            // 인라인 편집 모드: 이름/경로/인자 텍스트박스를 표시
                            if let Some(buf) = state.editing.as_mut() {
                                ui.text_edit_singleline(&mut buf.name);
                                ui.text_edit_singleline(&mut buf.path);
                                ui.text_edit_singleline(&mut buf.args);
                                ui.horizontal(|ui| {
                                    if ui.button("💾 저장").clicked() {
                                        to_save_edit = Some(id.clone());
                                    }
                                    if ui.button("취소").clicked() {
                                        to_cancel_edit = true;
                                    }
                                });
                            }
                        } else {
                            // 일반 표시 모드 — 경로가 존재하지 않으면 빨갛게 + 배지 표시
                            let alive = tool_exists(&path);
                            let name_text = if alive {
                                RichText::new(&name)
                            } else {
                                RichText::new(format!("⚠ {}", name))
                                    .color(DEAD_COLOR)
                                    .strong()
                            };
                            if ui.selectable_label(is_selected, name_text).clicked() {
                                to_select = Some(id.clone());
                            }
                            let path_text = if alive {
                                RichText::new(truncate_middle(&path, 50))
                            } else {
                                RichText::new(truncate_middle(&path, 50))
                                    .color(DEAD_COLOR)
                                    .italics()
                            };
                            ui.label(path_text)
                                .on_hover_text(if alive {
                                    path.clone()
                                } else {
                                    format!("파일 없음: {}", path)
                                });
                            ui.label(truncate_middle(&args, 20));

                            ui.horizontal(|ui| {
                                if ui
                                    .button("✏")
                                    .on_hover_text("이 툴 편집")
                                    .clicked()
                                {
                                    state.editing = Some(EditingBuffer {
                                        id: id.clone(),
                                        name: name.clone(),
                                        path: path.clone(),
                                        args: args.clone(),
                                    });
                                    to_select = Some(id.clone());
                                }
                                if ui
                                    .button("❌")
                                    .on_hover_text("이 툴 삭제")
                                    .clicked()
                                {
                                    state.pending_delete_id = Some(id.clone());
                                }
                            });
                        }
                        ui.end_row();
                    }

                    // 그리드 렌더링 도중에는 state 를 변경할 수 없으므로 후처리
                    if let Some(id) = to_select {
                        state.selected_id = Some(id);
                    }
                    if let Some(id) = to_delete {
                        state.pending_delete_id = Some(id);
                    }
                    if let Some(id) = to_save_edit {
                        if let Some(buf) = state.editing.take() {
                            if let Some(t) = state.config.find_mut(&id) {
                                t.name = buf.name.trim().to_string();
                                t.path = buf.path.trim().to_string();
                                t.args = buf.args.trim().to_string();
                            }
                            state.mark_dirty();
                        }
                    }
                    if to_cancel_edit {
                        state.editing = None;
                    }
                });
        });

    ui.add_space(6.0);

    // 순서 이동 버튼
    ui.horizontal(|ui| {
        let selected = state.selected_id.clone();
        let enabled = selected.is_some() && state.config.tools.len() > 1;

        ui.add_enabled_ui(enabled, |ui| {
            if ui.button("🔼 위로").clicked() {
                if let Some(id) = &selected {
                    if let Some(idx) = state.config.tools.iter().position(|t| &t.id == id) {
                        if idx > 0 {
                            state.config.swap(idx, idx - 1);
                            state.mark_dirty();
                        }
                    }
                }
            }
            if ui.button("🔽 아래로").clicked() {
                if let Some(id) = &selected {
                    if let Some(idx) = state.config.tools.iter().position(|t| &t.id == id) {
                        if idx + 1 < state.config.tools.len() {
                            state.config.swap(idx, idx + 1);
                            state.mark_dirty();
                        }
                    }
                }
            }
        });

        if let Some(id) = &state.selected_id {
            if let Some(t) = state.config.find(id) {
                ui.label(
                    RichText::new(format!("선택됨: {}", t.name))
                        .color(egui::Color32::DARK_GRAY),
                );
            }
        }
    });
}

/// 경로/인자처럼 긴 문자열을 표시 폭에 맞게 가운데 생략한다.
fn truncate_middle(s: &str, max_chars: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max_chars {
        return s.to_string();
    }
    let head = max_chars / 2 - 1;
    let tail = max_chars - head - 3;
    let head_str: String = chars[..head].iter().collect();
    let tail_str: String = chars[chars.len() - tail..].iter().collect();
    format!("{}...{}", head_str, tail_str)
}
