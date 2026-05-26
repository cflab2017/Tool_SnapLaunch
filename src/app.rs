// 관리 GUI 의 핵심 상태/이벤트 루프.
// eframe::App 트레잇을 구현하여 매 프레임 UI 를 다시 그린다.

use eframe::egui;

use crate::config::ToolsConfig;
use crate::msgbox;
use crate::registry;
use crate::ui;

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

    /// 툴 목록이 변경되었음을 표시하고 디스크에 저장한다.
    pub fn mark_dirty(&mut self) {
        if let Err(e) = self.config.save() {
            msgbox::error(
                &format!("툴 목록 저장에 실패했습니다.\n\n{}", e),
                "SnapLaunch",
            );
        }
        self.dirty_since_last_install = true;
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

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(4.0);
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
            if msgbox::confirm(
                &format!("정말 \"{}\" 툴을 삭제하시겠습니까?", name),
                "SnapLaunch",
            ) {
                self.state.config.remove_tool(&id);
                if self.state.selected_id.as_deref() == Some(id.as_str()) {
                    self.state.selected_id = None;
                }
                self.state.mark_dirty();
            }
        }
    }
}

/// eframe 의 드래그 앤 드롭 이벤트를 검사하여 EXE 류 파일이면 폼을 채운다.
fn handle_dropped_files(ctx: &egui::Context, state: &mut AppState) {
    ctx.input(|i| {
        for f in &i.raw.dropped_files {
            if let Some(path) = &f.path {
                let ext = path
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_ascii_lowercase();

                if matches!(ext.as_str(), "exe" | "bat" | "cmd" | "lnk") {
                    state.form.path = path.to_string_lossy().to_string();
                    if state.form.name.trim().is_empty() {
                        state.form.name = crate::config::tools::file_stem_name(path);
                    }
                } else {
                    msgbox::info(
                        "지원하지 않는 파일 형식입니다.\nEXE / BAT / CMD / LNK 만 끌어다 놓을 수 있습니다.",
                        "SnapLaunch",
                    );
                }
            }
        }
    });
}

/// EXE 에 임베드된 아이콘 PNG 바이트. 런타임에 egui 창 / 작업표시줄에 표시된다.
const ICON_PNG: &[u8] = include_bytes!("../assets/icon.png");

/// PNG 바이트를 egui 의 IconData (RGBA 픽셀 + 크기) 로 디코드한다.
/// 디코드 실패 시 None 을 반환하여 기본 아이콘을 사용한다.
fn load_icon() -> Option<egui::IconData> {
    // image 크레이트는 eframe 의 transitive dependency 라 별도 추가 없이 사용 가능
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
    let mut viewport = egui::ViewportBuilder::default()
        .with_title("🔧 자주사용하는 툴 관리")
        .with_inner_size([640.0, 560.0])
        .with_min_inner_size([520.0, 400.0]);

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
            // 한글 표시를 위해 시스템 한글 폰트를 로드 (있을 때만)
            install_korean_font(&cc.egui_ctx);
            Ok(Box::new(SnapLaunchApp::new(cc)))
        }),
    )
}

/// Windows 시스템에 포함된 맑은 고딕 폰트를 등록하여 한글이 깨지지 않게 한다.
/// 폰트 파일을 찾지 못하면 egui 의 기본 폰트를 그대로 사용한다.
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
