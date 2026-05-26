// Windows 네이티브 메시지 박스 표시를 위한 얇은 래퍼.
// MessageBoxW 를 직접 호출하여 UTF-16 변환과 플래그 처리를 캡슐화한다.

use std::iter::once;

use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    MessageBoxW, IDYES, MB_ICONERROR, MB_ICONINFORMATION, MB_ICONQUESTION, MB_OK, MB_YESNO,
    MESSAGEBOX_RESULT, MESSAGEBOX_STYLE,
};

/// UTF-8 문자열을 NULL 종단 UTF-16 시퀀스로 변환한다.
fn to_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(once(0)).collect()
}

/// 내부 공통 호출 함수.
fn show(message: &str, title: &str, style: MESSAGEBOX_STYLE) -> MESSAGEBOX_RESULT {
    let msg = to_wide(message);
    let ttl = to_wide(title);
    unsafe {
        MessageBoxW(
            HWND(std::ptr::null_mut()),
            PCWSTR(msg.as_ptr()),
            PCWSTR(ttl.as_ptr()),
            style,
        )
    }
}

/// 단순 정보 표시 (OK 버튼)
pub fn info(message: &str, title: &str) {
    show(message, title, MB_OK | MB_ICONINFORMATION);
}

/// 에러 표시 (OK 버튼, 빨간색 아이콘)
pub fn error(message: &str, title: &str) {
    show(message, title, MB_OK | MB_ICONERROR);
}

/// 예/아니오 확인. 사용자가 "예" 를 선택했으면 true.
pub fn confirm(message: &str, title: &str) -> bool {
    show(message, title, MB_YESNO | MB_ICONQUESTION) == IDYES
}
