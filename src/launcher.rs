// 컨텍스트 메뉴에서 호출되는 launch 모드의 핵심 로직.
// 주어진 tool id 에 해당하는 프로그램을 spawn 으로 실행하고 즉시 종료한다.
// 실행 실패 시 사용자에게 컨텍스트 메뉴에서 해당 항목을 제거할지 묻는다.

use std::path::Path;
use std::process::Command;

use crate::config::ToolsConfig;
use crate::i18n;
use crate::msgbox;
use crate::registry;

/// 지정된 ID 의 툴을 실행한다.
/// 실패 시 사용자에게 메시지 박스로 알리고 비정상 종료한다.
pub fn launch(tool_id: &str) {
    let config = ToolsConfig::load();
    let s = i18n::strings(config.language);

    let tool = match config.find(tool_id) {
        Some(t) => t.clone(),
        None => {
            // 설정 파일에서 ID 자체를 찾을 수 없는 경우 → 메뉴 갱신 안내
            offer_cleanup(tool_id, "(?)", s.launch_id_not_found);
            std::process::exit(1);
        }
    };

    // 인자가 비어 있으면 빈 슬라이스를, 아니면 공백 기준으로 토큰화한다.
    let arg_tokens: Vec<String> = if tool.args.trim().is_empty() {
        Vec::new()
    } else {
        tool.args.split_whitespace().map(|s| s.to_string()).collect()
    };

    // 작업 디렉토리는 실행 파일이 위치한 폴더로 지정 (상대 경로 인자가 있을 때 직관적)
    let work_dir = Path::new(&tool.path).parent().map(|p| p.to_path_buf());

    let mut cmd = Command::new(&tool.path);
    cmd.args(&arg_tokens);
    if let Some(wd) = work_dir {
        if !wd.as_os_str().is_empty() {
            cmd.current_dir(wd);
        }
    }

    match cmd.spawn() {
        Ok(_) => {
            // 자식 프로세스만 살려두고 부모(snap-launch) 는 즉시 종료
            std::process::exit(0);
        }
        Err(e) => {
            let detail = s
                .launch_path_reason_template
                .replace("{path}", &tool.path)
                .replace("{reason}", &e.to_string());
            offer_cleanup(tool_id, &tool.name, &detail);
            std::process::exit(1);
        }
    }
}

/// 실행 실패 시 사용자에게 컨텍스트 메뉴에서 해당 항목을 제거할지 묻는다.
/// "예" 선택 시 tools.json 에서 항목을 제거하고 레지스트리 메뉴를 재등록한다.
fn offer_cleanup(tool_id: &str, tool_name: &str, detail: &str) {
    let s = i18n::strings(ToolsConfig::load().language);

    let prompt = s
        .launch_failed_template
        .replace("{name}", tool_name)
        .replace("{detail}", detail);

    if !msgbox::confirm(&prompt, "SnapLaunch") {
        return;
    }

    // 1) 설정에서 제거
    let mut config = ToolsConfig::load();
    config.remove_tool(tool_id);
    if let Err(e) = config.save() {
        msgbox::error(&format!("{}{}", s.save_error_prefix, e), "SnapLaunch");
        return;
    }

    // 2) 레지스트리 재등록으로 메뉴 동기화
    match registry::install() {
        Ok(_) => msgbox::info(s.launch_remove_success, "SnapLaunch"),
        Err(e) => msgbox::error(
            &format!("{}{}", s.launch_remove_partial_error_prefix, e),
            "SnapLaunch",
        ),
    }
}
