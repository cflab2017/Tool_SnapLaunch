// 버튼 색상 팔레트와 팩토리 헬퍼.
// 단일 위치에서 정의하여 전체 UI 가 일관된 색을 사용하도록 한다.
//
// 의미 분류:
//   - PRIMARY (녹색): 가장 자주 실행되는 긍정 행동 (추가, 설치, 저장)
//   - DANGER  (빨강): 되돌리기 어려운 파괴적 행동 (제거, 삭제)
//   - INFO    (파랑): 동기화/갱신 같은 정보성 행동
//   - 그 외 (취소·찾아보기·위/아래 등): 기본 스타일 유지

use egui::{Button, Color32, RichText};

pub const PRIMARY: Color32 = Color32::from_rgb(40, 160, 80);   // 녹색
pub const DANGER: Color32 = Color32::from_rgb(200, 60, 60);    // 빨강
pub const INFO: Color32 = Color32::from_rgb(40, 110, 200);     // 파랑
pub const DANGER_TEXT: Color32 = Color32::from_rgb(200, 60, 60); // 아이콘 전용 빨강

/// 흰 글자 + 녹색 배경 버튼.
pub fn primary<'a>(label: &str) -> Button<'a> {
    Button::new(RichText::new(label.to_owned()).color(Color32::WHITE)).fill(PRIMARY)
}

/// 흰 글자 + 빨강 배경 버튼.
pub fn danger<'a>(label: &str) -> Button<'a> {
    Button::new(RichText::new(label.to_owned()).color(Color32::WHITE)).fill(DANGER)
}

/// 흰 글자 + 파랑 배경 버튼.
pub fn info<'a>(label: &str) -> Button<'a> {
    Button::new(RichText::new(label.to_owned()).color(Color32::WHITE)).fill(INFO)
}

/// 채움 없이 빨간 글자만 사용하는 작은 아이콘 버튼 (예: 행 ❌).
pub fn danger_icon<'a>(label: &str) -> Button<'a> {
    Button::new(RichText::new(label.to_owned()).color(DANGER_TEXT))
}
