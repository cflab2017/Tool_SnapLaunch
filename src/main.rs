// SnapLaunch: Windows 컨텍스트 메뉴 즐겨찾기 툴.
// 하나의 EXE 가 커맨드라인 인자에 따라 서로 다른 모드로 동작한다.

// 콘솔 창이 뜨지 않도록 windows 서브시스템 사용 (GUI / launch 모드 모두에서 검은 창이 보이지 않음)
#![windows_subsystem = "windows"]

mod app;
mod config;
mod launcher;
mod msgbox;
mod path_util;
mod registry;
mod ui;

fn main() {
    // args[0] = EXE 경로, args[1] = 모드
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("");

    match mode {
        // 컨텍스트 메뉴 설치
        "install" => match registry::install() {
            Ok(_) => msgbox::info(
                "컨텍스트 메뉴가 설치되었습니다.\n바탕화면이나 폴더 빈 공간에서 우클릭하여 확인하세요.",
                "SnapLaunch",
            ),
            Err(e) => msgbox::error(
                &format!("메뉴 설치 중 오류가 발생했습니다.\n\n{}", e),
                "SnapLaunch",
            ),
        },
        // 컨텍스트 메뉴 제거 (확인 → 삭제)
        "uninstall" => {
            if msgbox::confirm(
                "컨텍스트 메뉴를 제거하시겠습니까?\n(등록된 툴 목록 파일은 그대로 유지됩니다.)",
                "SnapLaunch",
            ) {
                match registry::uninstall() {
                    Ok(_) => {
                        msgbox::info("컨텍스트 메뉴가 제거되었습니다.", "SnapLaunch");
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
        // 등록된 툴 실행
        "launch" => {
            let tool_id = match args.get(2) {
                Some(id) if !id.trim().is_empty() => id.clone(),
                _ => {
                    msgbox::error(
                        "launch 인자에 실행할 툴 ID 가 지정되지 않았습니다.",
                        "SnapLaunch",
                    );
                    std::process::exit(1);
                }
            };
            launcher::launch(&tool_id);
        }
        // 관리 GUI (컨텍스트 메뉴의 "툴 추가/관리..." 진입점)
        "launch-manage" | "" => {
            if let Err(e) = app::run() {
                msgbox::error(
                    &format!("관리 창을 시작할 수 없습니다.\n\n{}", e),
                    "SnapLaunch",
                );
            }
        }
        // 알 수 없는 인자 → GUI 로 폴백
        _ => {
            if let Err(e) = app::run() {
                msgbox::error(
                    &format!("관리 창을 시작할 수 없습니다.\n\n{}", e),
                    "SnapLaunch",
                );
            }
        }
    }
}
