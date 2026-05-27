// 관리 GUI 의 핵심 상태/이벤트 루프.
// eframe::App 트레잇을 구현하여 매 프레임 UI 를 다시 그린다.

use std::collections::HashSet;

use eframe::egui;

use crate::config::ToolsConfig;
use crate::i18n::{self, Language, Strings};
use crate::msgbox;
use crate::registry;
use crate::ui;

/// 빌드 시점의 Cargo 패키지 버전 (Cargo.toml 의 version)
const VERSION: &str = env!("CARGO_PKG_VERSION");
/// 제작자 표기
const AUTHOR: &str = "Joseph.han";
/// 제작자 웹사이트
const WEBSITE: &str = "coding-now.com";

/// 창 제목을 "<로컬라이즈 제목>  ·  v<버전>  ·  <제작자> · <웹사이트>" 형태로 조립한다.
fn build_window_title(s: &Strings) -> String {
    format!(
        "{}   ·   v{}   ·   {} · {}",
        s.window_title, VERSION, AUTHOR, WEBSITE
    )
}

/// 새 툴 추가 폼이 보유하는 입력 상태.
#[derive(Default)]
pub struct ToolFormState {
    pub name: String,
    pub path: String,
    pub args: String,
}

impl ToolFormState {
    pub fn clear(&mut self) {
        self.name.clear();
        self.path.clear();
        self.args.clear();
    }
}

/// 인라인 편집 중인 행의 임시 버퍼.
pub struct EditingBuffer {
    pub id: String,
    pub name: String,
    pub path: String,
    pub args: String,
}

/// 애플리케이션 전체 상태.
pub struct AppState {
    /// tools.json 로부터 로드된 툴 목록 (변경 즉시 디스크에 저장)
    pub config: ToolsConfig,
    /// 새 툴 추가 폼 입력값
    pub form: ToolFormState,
    /// 현재 인라인 편집 중인 행 (없으면 None)
    pub editing: Option<EditingBuffer>,
    /// 위/아래 버튼의 대상이 되는 선택 행
    pub selected_id: Option<String>,
    /// 삭제 확인이 필요한 행 (다음 프레임에서 확인 팝업을 띄움)
    pub pending_delete_id: Option<String>,
    /// 컨텍스트 메뉴가 현재 레지스트리에 설치되어 있는지
    pub installed: bool,
    /// 마지막 설치/갱신 이후 툴 목록이 바뀌었는지 (UI 강조용)
    pub dirty_since_last_install: bool,
    /// 마지막 설치/갱신 이후 변경된 개별 툴 ID 집합 — 해당 행을 옅은 파랑으로 강조
    pub dirty_tool_ids: HashSet<String>,
    /// 현재 실행 중인 EXE 절대 경로
    pub current_exe_path: Option<String>,
    /// 레지스트리에 등록되어 있는 EXE 경로
    pub registered_exe_path: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        let mut state = Self {
            config: ToolsConfig::load(),
            form: ToolFormState::default(),
            editing: None,
            selected_id: None,
            pending_delete_id: None,
            installed: false,
            dirty_since_last_install: false,
            dirty_tool_ids: HashSet::new(),
            current_exe_path: std::env::current_exe()
                .ok()
                .map(|p| p.to_string_lossy().to_string()),
            registered_exe_path: None,
        };
        state.refresh_install_state();
        state
    }

    /// 레지스트리에서 현재 설치 상태와 등록된 EXE 경로를 다시 조회한다.
    pub fn refresh_install_state(&mut self) {
        self.installed = registry::is_installed();
        self.registered_exe_path = registry::registered_exe_path();
    }

    /// 현재 언어에 해당하는 문자열 테이블.
    pub fn s(&self) -> &'static Strings {
        i18n::strings(self.config.language)
    }

    /// 툴 목록이 변경되었음을 표시하고 디스크에 저장한다.
    /// `changed_ids` 로 전달된 툴 ID 들은 개별 행 강조용으로 dirty 셋에 등록된다.
    /// (삭제처럼 행이 사라지는 변경에는 빈 슬라이스를 넘긴다.)
    pub fn mark_dirty(&mut self, changed_ids: &[&str]) {
        for id in changed_ids {
            self.dirty_tool_ids.insert((*id).to_string());
        }
        if let Err(e) = self.config.save() {
            msgbox::error(
                &format!("{}{}", self.s().save_error_prefix, e),
                "SnapLaunch",
            );
        }
        self.dirty_since_last_install = true;
    }

    /// 메뉴 설치/갱신이 성공했을 때 호출 — 모든 dirty 표시를 초기화한다.
    pub fn clear_dirty_marks(&mut self) {
        self.dirty_tool_ids.clear();
        self.dirty_since_last_install = false;
    }

    /// 언어만 변경되었을 때 호출. UI 갱신을 위해 즉시 저장하지만 dirty 플래그는 올리지 않는다.
    pub fn save_language(&mut self) {
        if let Err(e) = self.config.save() {
            msgbox::error(
                &format!("{}{}", self.s().save_error_prefix, e),
                "SnapLaunch",
            );
        }
    }
}

/// 관리 GUI 의 진입점 컨테이너. eframe 이 보유한다.
pub struct SnapLaunchApp {
    state: AppState,
}

impl SnapLaunchApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            state: AppState::new(),
        }
    }
}

impl eframe::App for SnapLaunchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 드래그 앤 드롭 처리: EXE 류 파일이 떨어지면 폼에 자동 입력
        handle_dropped_files(ctx, &mut self.state);

        // 현재 언어에 따라 창 제목을 매 프레임 동기화 (라디오 변경 즉시 반영)
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(build_window_title(
            self.state.s(),
        )));

        egui::CentralPanel::default().show(ctx, |ui| {
            language_bar(ui, &mut self.state);
            ui.separator();
            ui::tool_list::show(ui, &mut self.state);
            ui.add_space(4.0);
            ui::tool_form::show(ui, &mut self.state);
            ui.add_space(8.0);
            ui::control_bar::show(ui, &mut self.state);
        });

        // 삭제 확인 팝업은 패널 렌더링 직후에 처리하여 borrow 충돌을 피한다.
        if let Some(id) = self.state.pending_delete_id.take() {
            let name = self
                .state
                .config
                .find(&id)
                .map(|t| t.name.clone())
                .unwrap_or_else(|| id.clone());
            let prompt = self
                .state
                .s()
                .delete_confirm_template
                .replace("{name}", &name);
            if msgbox::confirm(&prompt, "SnapLaunch") {
                self.state.config.remove_tool(&id);
                // 삭제된 툴은 더 이상 행이 없으므로 dirty_tool_ids 에서도 제거
                self.state.dirty_tool_ids.remove(&id);
                if self.state.selected_id.as_deref() == Some(id.as_str()) {
                    self.state.selected_id = None;
                }
                self.state.mark_dirty(&[]);
            }
        }
    }
}

/// 상단 언어 선택 바.
fn language_bar(ui: &mut egui::Ui, state: &mut AppState) {
    let s = state.s();
    ui.horizontal(|ui| {
        ui.label(s.lang_label).on_hover_text(s.tip_lang_radio);
        let mut lang = state.config.language;
        ui.radio_value(&mut lang, Language::Korean, s.lang_korean)
            .on_hover_text(s.tip_lang_radio);
        ui.radio_value(&mut lang, Language::English, s.lang_english)
            .on_hover_text(s.tip_lang_radio);
        if lang != state.config.language {
            state.config.language = lang;
            state.save_language();
        }
    });
}

/// eframe 의 드래그 앤 드롭 이벤트를 검사하여 EXE 류 파일이면 폼을 채운다.
fn handle_dropped_files(ctx: &egui::Context, state: &mut AppState) {
    let mut unsupported = false;
    let mut path_to_set: Option<std::path::PathBuf> = None;

    ctx.input(|i| {
        for f in &i.raw.dropped_files {
            if let Some(path) = &f.path {
                let ext = path
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_ascii_lowercase();

                if matches!(ext.as_str(), "exe" | "bat" | "cmd" | "lnk") {
                    path_to_set = Some(path.clone());
                } else {
                    unsupported = true;
                }
            }
        }
    });

    if let Some(path) = path_to_set {
        state.form.path = path.to_string_lossy().to_string();
        if state.form.name.trim().is_empty() {
            state.form.name = crate::config::tools::file_stem_name(&path);
        }
    }
    if unsupported {
        msgbox::info(state.s().dropped_unsupported, "SnapLaunch");
    }
}

/// EXE 에 임베드된 아이콘 PNG 바이트. 런타임에 egui 창 / 작업표시줄에 표시된다.
const ICON_PNG: &[u8] = include_bytes!("../assets/icon.png");

/// PNG 바이트를 egui 의 IconData (RGBA 픽셀 + 크기) 로 디코드한다.
fn load_icon() -> Option<egui::IconData> {
    let img = image::load_from_memory(ICON_PNG).ok()?.into_rgba8();
    let (w, h) = img.dimensions();
    Some(egui::IconData {
        rgba: img.into_raw(),
        width: w,
        height: h,
    })
}

/// GUI 모드 진입점. eframe::run_native 를 호출한다.
pub fn run() -> eframe::Result<()> {
    // 시작 시 저장된 언어로 초기 창 제목을 설정한다 (이후 매 프레임 동기화).
    let initial_lang = ToolsConfig::load().language;
    let initial_title = build_window_title(i18n::strings(initial_lang));

    let mut viewport = egui::ViewportBuilder::default()
        .with_title(initial_title)
        .with_inner_size([720.0, 600.0])
        .with_min_inner_size([560.0, 420.0]);

    if let Some(icon) = load_icon() {
        viewport = viewport.with_icon(icon);
    }

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };
    eframe::run_native(
        "SnapLaunch",
        options,
        Box::new(|cc| {
            install_korean_font(&cc.egui_ctx);
            Ok(Box::new(SnapLaunchApp::new(cc)))
        }),
    )
}

/// Windows 시스템에 포함된 맑은 고딕 폰트를 등록하여 한글이 깨지지 않게 한다.
/// 영어 모드에서도 비용은 동일하며, 한글 → 영문 전환 시 폰트 부재로 인한 ☐ 표시를 방지한다.
fn install_korean_font(ctx: &egui::Context) {
    let candidates = [
        r"C:\Windows\Fonts\malgun.ttf",
        r"C:\Windows\Fonts\malgunbd.ttf",
        r"C:\Windows\Fonts\gulim.ttc",
    ];

    for path in candidates {
        if let Ok(bytes) = std::fs::read(path) {
            let mut fonts = egui::FontDefinitions::default();
            fonts
                .font_data
                .insert("korean".to_owned(), egui::FontData::from_owned(bytes));
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "korean".to_owned());
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("korean".to_owned());
            ctx.set_fonts(fonts);
            return;
        }
    }
}
