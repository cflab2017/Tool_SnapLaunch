// 국제화(i18n) 모듈.
// 모든 사용자 노출 문자열을 한국어/영어 두 벌로 보유한다.
// 새 언어가 필요하면 Language enum 과 LANG_KO/LANG_EN 을 따라 LANG_XX 를 추가하면 된다.

use serde::{Deserialize, Serialize};

/// UI 언어 선택지. tools.json 에 직렬화되어 보존된다.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "ko")]
    Korean,
    #[serde(rename = "en")]
    English,
}

impl Default for Language {
    fn default() -> Self {
        Language::Korean
    }
}

/// 표현되는 모든 문자열의 단일 출처.
/// 자리 표시자가 들어가는 항목은 호출 측에서 String::replace 로 치환한다.
pub struct Strings {
    // ── 창/공통 ──
    pub window_title: &'static str,
    pub lang_label: &'static str,
    pub lang_korean: &'static str,
    pub lang_english: &'static str,

    // ── 툴 목록 ──
    pub registered_tools_heading: &'static str,
    pub empty_list_message: &'static str,
    pub col_order: &'static str,
    pub col_name: &'static str,
    pub col_path: &'static str,
    pub col_args: &'static str,
    pub col_actions: &'static str,
    pub edit_tooltip: &'static str,
    pub delete_tooltip: &'static str,
    pub dirty_tooltip: &'static str,
    pub btn_save: &'static str,
    pub btn_cancel: &'static str,
    pub btn_move_up: &'static str,
    pub btn_move_down: &'static str,
    pub selected_prefix: &'static str,
    pub missing_file_prefix: &'static str,

    // ── 새 툴 추가 폼 ──
    pub add_tool_heading: &'static str,
    pub field_name: &'static str,
    pub field_path: &'static str,
    pub field_args: &'static str,
    pub btn_browse: &'static str,
    pub btn_add: &'static str,
    pub btn_clear: &'static str,
    pub dnd_hint: &'static str,
    pub file_dialog_title: &'static str,
    pub file_dialog_filter_exe: &'static str,
    pub file_dialog_filter_all: &'static str,

    // ── 컨트롤 바 ──
    pub btn_install: &'static str,
    pub btn_uninstall: &'static str,
    pub btn_refresh: &'static str,
    pub btn_refresh_dirty: &'static str,
    pub status_installed: &'static str,
    pub status_not_installed: &'static str,
    /// `{old}` → 등록된 경로, `{new}` → 현재 경로
    pub path_mismatch_template: &'static str,

    // ── 메시지 박스 ──
    pub install_success: &'static str,
    pub install_error_prefix: &'static str,
    pub uninstall_confirm: &'static str,
    pub uninstall_success: &'static str,
    pub uninstall_error_prefix: &'static str,
    pub refresh_success: &'static str,
    pub refresh_error_prefix: &'static str,
    /// `{name}` → 툴 이름
    pub delete_confirm_template: &'static str,
    pub dropped_unsupported: &'static str,
    pub save_error_prefix: &'static str,

    // ── launcher ──
    /// `{name}` → 툴 이름, `{detail}` → 상세 정보
    pub launch_failed_template: &'static str,
    pub launch_id_not_found: &'static str,
    /// `{path}`, `{reason}`
    pub launch_path_reason_template: &'static str,
    pub launch_no_id: &'static str,
    pub launch_remove_success: &'static str,
    pub launch_remove_partial_error_prefix: &'static str,
    pub launch_cant_start_gui_prefix: &'static str,

    // ── 컨텍스트 메뉴 항목 (registry) ──
    pub menu_top_label: &'static str,
    pub menu_manage: &'static str,
    pub menu_uninstall: &'static str,

    // ── 복구.bat 메시지 ──
    pub recovery_bat_message_start: &'static str,
    pub recovery_bat_message_done: &'static str,
}

/// 주어진 언어의 문자열 테이블을 반환한다.
pub fn strings(lang: Language) -> &'static Strings {
    match lang {
        Language::Korean => &LANG_KO,
        Language::English => &LANG_EN,
    }
}

// ──────────────────────────────────────────────────────────────
// 한국어
// ──────────────────────────────────────────────────────────────
pub const LANG_KO: Strings = Strings {
    window_title: "🔧 자주사용하는 툴 관리",
    lang_label: "언어:",
    lang_korean: "한국어",
    lang_english: "English",

    registered_tools_heading: "등록된 툴 목록",
    empty_list_message: "아직 등록된 툴이 없습니다. 아래 \"새 툴 추가\" 영역에서 추가하거나, EXE 파일을 창에 끌어다 놓으세요.",
    col_order: "순서",
    col_name: "이름",
    col_path: "경로",
    col_args: "인자",
    col_actions: "작업",
    edit_tooltip: "이 툴 편집",
    delete_tooltip: "이 툴 삭제",
    dirty_tooltip: "변경됨 — \"메뉴 갱신\" 으로 동기화하세요.",
    btn_save: "💾 저장",
    btn_cancel: "취소",
    btn_move_up: "🔼 위로",
    btn_move_down: "🔽 아래로",
    selected_prefix: "선택됨: ",
    missing_file_prefix: "파일 없음: ",

    add_tool_heading: "➕ 새 툴 추가",
    field_name: "이름:",
    field_path: "경로:",
    field_args: "인자:",
    btn_browse: "📁 찾아보기",
    btn_add: "➕ 추가",
    btn_clear: "입력 초기화",
    dnd_hint: "(EXE 파일을 창에 끌어다 놓으면 자동으로 채워집니다)",
    file_dialog_title: "실행할 프로그램 선택",
    file_dialog_filter_exe: "실행 파일",
    file_dialog_filter_all: "모든 파일",

    btn_install: "📥 마우스 오른쪽 메뉴 설치",
    btn_uninstall: "📤 Uninstall",
    btn_refresh: "🔄 메뉴 갱신",
    btn_refresh_dirty: "🔄 메뉴 갱신 (변경됨)",
    status_installed: "✅ 컨텍스트 메뉴가 설치되어 있습니다.",
    status_not_installed: "⚠ 컨텍스트 메뉴가 아직 설치되지 않았습니다. \"마우스 오른쪽 메뉴 설치\" 를 눌러 주세요.",
    path_mismatch_template: "⚠ 등록된 EXE 경로와 현재 위치가 다릅니다.\n  등록된 경로: {old}\n  현재 위치   : {new}\n  \"메뉴 갱신\" 을 눌러 재등록해 주세요.",

    install_success: "컨텍스트 메뉴가 설치되었습니다.\n바탕화면이나 폴더 빈 공간에서 우클릭하여 확인하세요.",
    install_error_prefix: "메뉴 설치 중 오류가 발생했습니다.\n\n",
    uninstall_confirm: "컨텍스트 메뉴를 제거하시겠습니까?\n(등록된 툴 목록 파일은 그대로 유지됩니다.)",
    uninstall_success: "컨텍스트 메뉴가 제거되었습니다.",
    uninstall_error_prefix: "메뉴 제거 중 오류가 발생했습니다.\n\n",
    refresh_success: "컨텍스트 메뉴가 최신 상태로 갱신되었습니다.",
    refresh_error_prefix: "메뉴 갱신 중 오류가 발생했습니다.\n\n",
    delete_confirm_template: "정말 \"{name}\" 툴을 삭제하시겠습니까?",
    dropped_unsupported: "지원하지 않는 파일 형식입니다.\nEXE / BAT / CMD / LNK 만 끌어다 놓을 수 있습니다.",
    save_error_prefix: "툴 목록 저장에 실패했습니다.\n\n",

    launch_failed_template: "\"{name}\" 실행에 실패했습니다.\n\n{detail}\n\n이 항목을 컨텍스트 메뉴에서 제거하시겠습니까?",
    launch_id_not_found: "이 항목이 설정 파일에 더 이상 존재하지 않습니다.",
    launch_path_reason_template: "경로: {path}\n사유: {reason}",
    launch_no_id: "launch 인자에 실행할 툴 ID 가 지정되지 않았습니다.",
    launch_remove_success: "컨텍스트 메뉴에서 해당 항목을 제거했습니다.",
    launch_remove_partial_error_prefix: "항목은 설정에서 제거되었으나 메뉴 갱신 중 오류가 발생했습니다.\n관리 창에서 \"메뉴 갱신\" 을 한 번 더 눌러 주세요.\n\n",
    launch_cant_start_gui_prefix: "관리 창을 시작할 수 없습니다.\n\n",

    menu_top_label: "자주사용하는 툴",
    menu_manage: "툴 추가/관리...",
    menu_uninstall: "Uninstall",

    recovery_bat_message_start: "SnapLaunch 컨텍스트 메뉴를 강제 제거합니다...",
    recovery_bat_message_done: "완료되었습니다. 탐색기를 재시작하면 메뉴가 사라집니다.",
};

// ──────────────────────────────────────────────────────────────
// English
// ──────────────────────────────────────────────────────────────
pub const LANG_EN: Strings = Strings {
    window_title: "🔧 SnapLaunch — Favorite Tools",
    lang_label: "Language:",
    lang_korean: "한국어",
    lang_english: "English",

    registered_tools_heading: "Registered Tools",
    empty_list_message: "No tools registered yet. Add one below in \"Add New Tool\" or drag an EXE onto this window.",
    col_order: "#",
    col_name: "Name",
    col_path: "Path",
    col_args: "Args",
    col_actions: "Actions",
    edit_tooltip: "Edit this tool",
    delete_tooltip: "Delete this tool",
    dirty_tooltip: "Changed — click \"Refresh Menu\" to sync.",
    btn_save: "💾 Save",
    btn_cancel: "Cancel",
    btn_move_up: "🔼 Up",
    btn_move_down: "🔽 Down",
    selected_prefix: "Selected: ",
    missing_file_prefix: "File not found: ",

    add_tool_heading: "➕ Add New Tool",
    field_name: "Name:",
    field_path: "Path:",
    field_args: "Args:",
    btn_browse: "📁 Browse",
    btn_add: "➕ Add",
    btn_clear: "Clear",
    dnd_hint: "(Drag an EXE onto the window to auto-fill)",
    file_dialog_title: "Select program to launch",
    file_dialog_filter_exe: "Executables",
    file_dialog_filter_all: "All files",

    btn_install: "📥 Install Right-Click Menu",
    btn_uninstall: "📤 Uninstall",
    btn_refresh: "🔄 Refresh Menu",
    btn_refresh_dirty: "🔄 Refresh Menu (changed)",
    status_installed: "✅ The context menu is installed.",
    status_not_installed: "⚠ The context menu is not installed yet. Click \"Install Right-Click Menu\".",
    path_mismatch_template: "⚠ The registered EXE path differs from the current location.\n  Registered: {old}\n  Current   : {new}\n  Click \"Refresh Menu\" to re-register.",

    install_success: "The context menu has been installed.\nRight-click the desktop or an empty folder area to try it.",
    install_error_prefix: "An error occurred while installing the menu.\n\n",
    uninstall_confirm: "Remove the context menu?\n(The tools.json file will be kept.)",
    uninstall_success: "The context menu has been removed.",
    uninstall_error_prefix: "An error occurred while removing the menu.\n\n",
    refresh_success: "The context menu has been refreshed.",
    refresh_error_prefix: "An error occurred while refreshing the menu.\n\n",
    delete_confirm_template: "Really delete the tool \"{name}\"?",
    dropped_unsupported: "Unsupported file type.\nOnly EXE / BAT / CMD / LNK can be dropped.",
    save_error_prefix: "Failed to save the tools list.\n\n",

    launch_failed_template: "Failed to launch \"{name}\".\n\n{detail}\n\nRemove this entry from the context menu?",
    launch_id_not_found: "This entry no longer exists in the config file.",
    launch_path_reason_template: "Path: {path}\nReason: {reason}",
    launch_no_id: "No tool ID was specified for the launch argument.",
    launch_remove_success: "The entry was removed from the context menu.",
    launch_remove_partial_error_prefix: "The entry was removed from the config, but refreshing the menu failed.\nPlease click \"Refresh Menu\" again in the management window.\n\n",
    launch_cant_start_gui_prefix: "Failed to open the management window.\n\n",

    menu_top_label: "Favorite Tools",
    menu_manage: "Add / Manage Tools...",
    menu_uninstall: "Uninstall",

    recovery_bat_message_start: "Forcibly removing the SnapLaunch context menu...",
    recovery_bat_message_done: "Done. Restart Explorer to refresh the menus.",
};
