// 툴 경로 유효성 검사 헬퍼.
// 절대/상대 경로면 그대로 exists() 확인, 단일 파일명이면 PATH + PATHEXT 를 탐색한다.
// 이렇게 해야 "calc.exe" 처럼 PATH 에 의존하는 항목도 정상으로 인식된다.

use std::path::{Path, PathBuf};

/// 주어진 툴 경로가 실제로 실행 가능한 파일을 가리키는지 확인한다.
pub fn tool_exists(path: &str) -> bool {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return false;
    }

    let p = Path::new(trimmed);

    // 절대 경로이거나 디렉토리 구분자가 포함된 상대 경로 → 그대로 검사
    if p.is_absolute() || trimmed.contains('\\') || trimmed.contains('/') {
        return p.exists();
    }

    // 단일 파일명 (예: "calc.exe", "notepad") → PATH 디렉토리들을 순회
    find_in_path(trimmed).is_some()
}

/// PATH + PATHEXT 를 이용해 명령 이름의 실제 경로를 찾는다.
fn find_in_path(name: &str) -> Option<PathBuf> {
    let path_var = std::env::var_os("PATH")?;
    let pathext = std::env::var("PATHEXT")
        .unwrap_or_else(|_| ".EXE;.BAT;.CMD;.COM".to_string());

    let exts: Vec<String> = pathext
        .split(';')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    let name_path = Path::new(name);
    let has_ext = name_path.extension().is_some();

    for dir in std::env::split_paths(&path_var) {
        if dir.as_os_str().is_empty() {
            continue;
        }
        let candidate = dir.join(name);

        // 이름에 확장자가 이미 있으면 그대로 검사
        if candidate.is_file() {
            return Some(candidate);
        }

        // 확장자가 없는 경우 PATHEXT 의 각 확장자로 시도
        if !has_ext {
            for ext in &exts {
                let with_ext = dir.join(format!("{}{}", name, ext));
                if with_ext.is_file() {
                    return Some(with_ext);
                }
            }
        }
    }
    None
}
