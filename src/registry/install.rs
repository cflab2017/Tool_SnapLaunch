// 레지스트리에 컨텍스트 메뉴를 등록한다.
// 바탕화면 배경/폴더 배경 양쪽에 동일한 cascading 서브메뉴 구조를 미러링한다.

use std::io;

use winreg::enums::{HKEY_CURRENT_USER, KEY_ALL_ACCESS, REG_DWORD};
use winreg::RegKey;

use std::fs;

use super::{
    recovery_bat_path, uninstall, DESKTOP_ROOT, DIRECTORY_ROOT, MENU_LABEL,
};
use crate::config::ToolsConfig;

/// 컨텍스트 메뉴를 (재)설치한다.
/// 1) 기존 등록 내용을 모두 제거한 뒤
/// 2) 현재 EXE 경로와 tools.json 의 툴 목록으로 새로 등록한다.
pub fn install() -> io::Result<()> {
    // 1) 현재 EXE 절대 경로
    let exe_path = std::env::current_exe()?
        .to_string_lossy()
        .to_string();

    // 2) 등록된 툴 목록 로드
    let config = ToolsConfig::load();

    // 3) 깨끗한 상태에서 시작하기 위해 기존 등록은 모두 제거
    let _ = uninstall();

    // 4) 두 루트(바탕화면/폴더 배경) 각각에 동일한 서브메뉴 트리를 구성한다.
    for root in [DESKTOP_ROOT, DIRECTORY_ROOT] {
        write_cascading_menu(root, &exe_path, &config)?;
    }

    // 5) snap-launch.exe 가 삭제된 비상 상황을 대비한 복구 배치 파일을 EXE 옆에 생성한다.
    //    배치 파일이 남아 있으면 사용자가 두 번 클릭만으로 컨텍스트 메뉴를 제거할 수 있다.
    //    배치 파일 생성에 실패해도 본 기능에는 영향이 없으므로 오류는 무시한다.
    let _ = write_recovery_bat();

    Ok(())
}

/// 비상 복구용 배치 파일을 EXE 옆에 작성한다.
/// 내용은 두 HKCU 키를 강제로 삭제하므로 snap-launch.exe 가 없어도 동작한다.
fn write_recovery_bat() -> std::io::Result<()> {
    let Some(path) = recovery_bat_path() else {
        return Ok(());
    };
    // chcp 65001 로 UTF-8 표시를 시도하되, 실패해도 reg 명령은 정상 동작한다.
    let content = format!(
        "@echo off\r\n\
chcp 65001 >nul 2>&1\r\n\
echo SnapLaunch 컨텍스트 메뉴를 강제 제거합니다...\r\n\
reg delete \"HKCU\\{desktop}\" /f >nul 2>&1\r\n\
reg delete \"HKCU\\{directory}\" /f >nul 2>&1\r\n\
echo.\r\n\
echo 완료되었습니다. 탐색기를 재시작하면 메뉴가 사라집니다.\r\n\
pause\r\n",
        desktop = DESKTOP_ROOT,
        directory = DIRECTORY_ROOT,
    );
    fs::write(path, content)
}

/// 단일 루트 경로 아래에 cascading 서브메뉴 트리를 작성한다.
fn write_cascading_menu(root: &str, exe_path: &str, config: &ToolsConfig) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // -- 최상위 메뉴 항목 (서브메뉴 컨테이너) --
    let (root_key, _) = hkcu.create_subkey(root)?;
    root_key.set_value("MUIVerb", &MENU_LABEL.to_string())?;
    // SubCommands 값을 빈 문자열로 두면 Shell 서브키를 cascading 으로 노출한다.
    root_key.set_value("SubCommands", &String::new())?;
    // Icon: EXE 의 0 번 아이콘 리소스를 사용
    root_key.set_value("Icon", &format!("{},0", exe_path))?;

    // -- 서브 항목들을 보관할 "shell" 컨테이너 --
    // 키 이름은 대소문자를 구분하지 않으며 정렬 순서대로 메뉴에 노출된다.
    let shell_path = format!(r"{}\shell", root);
    let (_shell_key, _) = hkcu.create_subkey(&shell_path)?;

    // -- 등록된 툴들을 order 순서대로 작성 --
    // 알파벳 정렬 순서대로 표시되므로 "aaa_NN_<id>" 형식의 키 이름을 사용한다.
    let mut sorted = config.tools.clone();
    sorted.sort_by_key(|t| t.order);

    for (idx, tool) in sorted.iter().enumerate() {
        let key_name = format!("aaa_{:03}_{}", idx + 1, sanitize_key(&tool.id));
        let tool_path = format!(r"{}\{}", shell_path, key_name);

        let (tool_key, _) = hkcu.create_subkey(&tool_path)?;
        tool_key.set_value("MUIVerb", &tool.name)?;

        // 아이콘: icon_path 지정 시 그것을, 아니면 실행 파일의 첫 번째 아이콘을 사용
        let icon_value = if tool.icon_path.trim().is_empty() {
            format!("{},0", tool.path)
        } else {
            tool.icon_path.clone()
        };
        tool_key.set_value("Icon", &icon_value)?;

        // command 서브키: "<snap-launch.exe>" launch <tool_id>
        let cmd_path = format!(r"{}\command", tool_path);
        let (cmd_key, _) = hkcu.create_subkey(&cmd_path)?;
        let command_line = format!("\"{}\" launch {}", exe_path, tool.id);
        // command 키의 기본값(@)에 명령줄을 기록
        cmd_key.set_value("", &command_line)?;
    }

    // -- 관리 GUI 호출 항목 --
    // CommandFlags = 0x20 (ECF_SEPARATORBEFORE) 는 "이 항목 위에 구분선 그리기" 의미.
    // 별도 구분선 항목을 두면 Shell 이 "-" 텍스트로 잘못 렌더링하므로, 관리 항목 자체에 플래그를 단다.
    let manage_path = format!(r"{}\zzz_manage", shell_path);
    let (manage_key, _) = hkcu.create_subkey(&manage_path)?;
    manage_key.set_value("MUIVerb", &"툴 추가/관리...".to_string())?;
    manage_key.set_value("Icon", &format!("{},0", exe_path))?;
    manage_key.set_raw_value(
        "CommandFlags",
        &winreg::RegValue {
            bytes: 0x20u32.to_le_bytes().to_vec(),
            vtype: REG_DWORD,
        },
    )?;
    let manage_cmd_path = format!(r"{}\command", manage_path);
    let (manage_cmd_key, _) = hkcu.create_subkey(&manage_cmd_path)?;
    manage_cmd_key.set_value("", &format!("\"{}\" launch-manage", exe_path))?;

    // -- 메뉴 제거 항목 --
    let uninstall_path = format!(r"{}\zzz_uninstall", shell_path);
    let (uninstall_key, _) = hkcu.create_subkey(&uninstall_path)?;
    uninstall_key.set_value("MUIVerb", &"Uninstall".to_string())?;
    let uninstall_cmd_path = format!(r"{}\command", uninstall_path);
    let (uninstall_cmd_key, _) = hkcu.create_subkey(&uninstall_cmd_path)?;
    uninstall_cmd_key.set_value("", &format!("\"{}\" uninstall", exe_path))?;

    // KEY_ALL_ACCESS 권한 명시는 winreg::create_subkey 가 내부적으로 처리하지만,
    // 구버전 호환을 위해 한 번 더 열어보아 권한 검증을 수행한다.
    let _ = hkcu.open_subkey_with_flags(root, KEY_ALL_ACCESS)?;

    Ok(())
}

/// 레지스트리 키 이름으로 사용해도 안전하도록 ASCII 영숫자/_/- 만 남긴다.
fn sanitize_key(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect()
}
