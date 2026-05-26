// 레지스트리에 등록된 컨텍스트 메뉴를 제거한다.

use std::fs;
use std::io;

use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

use super::{recovery_bat_path, DESKTOP_ROOT, DIRECTORY_ROOT};

/// 바탕화면 배경/폴더 배경 양쪽 모두에서 SnapLaunch 메뉴를 제거한다.
/// 키가 이미 없는 경우는 성공으로 간주한다.
/// 함께 생성되었던 복구 배치 파일도 같이 정리한다.
pub fn uninstall() -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    for root in [DESKTOP_ROOT, DIRECTORY_ROOT] {
        match hkcu.delete_subkey_all(root) {
            Ok(_) => {}
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                // 키가 존재하지 않으면 무시
            }
            Err(e) => return Err(e),
        }
    }

    // 복구 배치 파일도 함께 제거 (실패해도 메뉴 제거 결과에는 영향 없음)
    if let Some(bat) = recovery_bat_path() {
        if bat.exists() {
            let _ = fs::remove_file(bat);
        }
    }

    Ok(())
}
