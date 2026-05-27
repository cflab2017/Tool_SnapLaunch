// 레지스트리 모듈: Windows 컨텍스트 메뉴 등록/제거/상태 확인을 담당한다.
// HKEY_CURRENT_USER 만 사용하므로 관리자 권한이 필요하지 않다.

use std::path::PathBuf;

pub mod install;
pub mod status;
pub mod uninstall;

pub use install::install;
pub use status::{is_installed, registered_exe_path};
pub use uninstall::uninstall;

/// 바탕화면 배경 우클릭 메뉴의 루트 경로 (HKCU 하위)
pub const DESKTOP_ROOT: &str = r"Software\Classes\DesktopBackground\Shell\SnapLaunch";
/// 폴더 배경(탐색기 빈 공간) 우클릭 메뉴의 루트 경로 (HKCU 하위)
pub const DIRECTORY_ROOT: &str = r"Software\Classes\Directory\Background\Shell\SnapLaunch";

// 컨텍스트 메뉴 텍스트는 사용자가 선택한 언어에 따라 i18n::Strings 에서 가져온다.

/// snap-launch.exe 가 통째로 삭제된 경우를 위한 비상 복구 배치 파일 이름.
/// EXE 옆에 설치 시 함께 생성되며, 두 HKCU 키를 강제로 삭제한다.
pub const RECOVERY_BAT_NAME: &str = "SnapLaunch_복구.bat";

/// EXE 옆에 위치한 복구 배치 파일의 절대 경로를 반환한다.
pub fn recovery_bat_path() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let dir = exe.parent()?;
    Some(dir.join(RECOVERY_BAT_NAME))
}
