// SnapLaunch: Windows 컨텍스트 메뉴 즐겨찾기 툴.
// 하나의 EXE 가 커맨드라인 인자에 따라 서로 다른 모드로 동작한다.

// 콘솔 창이 뜨지 않도록 windows 서브시스템 사용
#![windows_subsystem = "windows"]

mod app;
mod config;
mod i18n;
mod launcher;
mod msgbox;
mod path_util;
mod registry;
mod ui;

use crate::config::ToolsConfig;

fn main() {
    // args[0] = EXE 경로, args[1] = 모드
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("");

    // 모든 모드에서 저장된 언어 설정으로 메시지를 표시한다.
    let s = i18n::strings(ToolsConfig::load().language);

    match mode {
        // 컨텍스트 메뉴 설치
        "install" => match registry::install() {
            Ok(_) => msgbox::info(s.install_success, "SnapLaunch"),
            Err(e) => msgbox::error(
                &format!("{}{}", s.install_error_prefix, e),
                "SnapLaunch",
            ),
        },
        // 컨텍스트 메뉴 제거 (확인 → 삭제)
        "uninstall" => {
            if msgbox::confirm(s.uninstall_confirm, "SnapLaunch") {
                match registry::uninstall() {
                    Ok(_) => {
                        msgbox::info(s.uninstall_success, "SnapLaunch");
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
        // 등록된 툴 실행
        "launch" => {
            let tool_id = match args.get(2) {
                Some(id) if !id.trim().is_empty() => id.clone(),
                _ => {
                    msgbox::error(s.launch_no_id, "SnapLaunch");
                    std::process::exit(1);
                }
            };
            launcher::launch(&tool_id);
        }
        // 관리 GUI (컨텍스트 메뉴의 "툴 추가/관리..." 진입점)
        "launch-manage" | "" => {
            if let Err(e) = app::run() {
                msgbox::error(
                    &format!("{}{}", s.launch_cant_start_gui_prefix, e),
                    "SnapLaunch",
                );
            }
        }
        // 알 수 없는 인자 → GUI 로 폴백
        _ => {
            if let Err(e) = app::run() {
                msgbox::error(
                    &format!("{}{}", s.launch_cant_start_gui_prefix, e),
                    "SnapLaunch",
                );
            }
        }
    }
}
