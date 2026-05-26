// 컨텍스트 메뉴 설치 상태를 확인하는 헬퍼들.

use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

use super::{DESKTOP_ROOT, DIRECTORY_ROOT};

/// 두 위치(바탕화면/폴더 배경) 중 하나라도 메뉴가 등록되어 있으면 true 를 반환한다.
pub fn is_installed() -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    hkcu.open_subkey(DESKTOP_ROOT).is_ok() || hkcu.open_subkey(DIRECTORY_ROOT).is_ok()
}

/// 현재 레지스트리에 기록된 "Icon" 값(= 현재 등록된 EXE 경로)을 반환한다.
/// EXE 경로가 이동되었는지 확인할 때 사용한다.
pub fn registered_exe_path() -> Option<String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key = hkcu.open_subkey(DESKTOP_ROOT).ok()?;
    let icon: String = key.get_value("Icon").ok()?;
    // Icon 값은 "<path>,0" 형식으로 저장됨 → ',' 이전까지가 EXE 경로
    let cleaned = icon.split(',').next().unwrap_or("").trim().to_string();
    if cleaned.is_empty() {
        None
    } else {
        Some(cleaned)
    }
}
