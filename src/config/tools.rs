// 등록된 툴 목록을 EXE 옆 `snap-launch-tools.json` 파일로 영속화한다.
// 모든 경로는 EXE의 현재 위치를 기준으로 계산되어 포터블하게 동작한다.

use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::i18n::Language;

/// 설정 파일명 (EXE와 동일 디렉토리에 위치)
pub const CONFIG_FILE_NAME: &str = "snap-launch-tools.json";

/// 컨텍스트 메뉴에 등록되는 단일 툴 정보
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// 고유 식별자 (예: "tool_001"). 레지스트리 키와 launch 인자에 사용
    pub id: String,
    /// 컨텍스트 메뉴에 표시될 이름
    pub name: String,
    /// 실행 파일 전체 경로
    pub path: String,
    /// 실행 시 전달할 인자 (공백 구분, 선택 사항)
    #[serde(default)]
    pub args: String,
    /// 아이콘 경로 (비어 있으면 path의 EXE 아이콘 사용)
    #[serde(default)]
    pub icon_path: String,
    /// 메뉴 표시 순서 (오름차순)
    pub order: u32,
}

/// tools.json 의 최상위 구조
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToolsConfig {
    /// UI / 컨텍스트 메뉴 언어. 기본은 Korean.
    #[serde(default)]
    pub language: Language,
    #[serde(default)]
    pub tools: Vec<Tool>,
}

impl ToolsConfig {
    /// EXE와 동일 디렉토리의 설정 파일 절대 경로를 반환한다.
    pub fn config_path() -> io::Result<PathBuf> {
        let exe = std::env::current_exe()?;
        let dir = exe.parent().ok_or_else(|| {
            io::Error::new(io::ErrorKind::Other, "EXE의 부모 디렉토리를 찾을 수 없습니다")
        })?;
        Ok(dir.join(CONFIG_FILE_NAME))
    }

    /// 설정 파일을 읽어 ToolsConfig 를 반환한다.
    /// 파일이 없으면 빈 설정을 반환한다.
    /// 파싱 실패 시에는 .bak 백업을 만들고 빈 설정으로 초기화한다.
    pub fn load() -> Self {
        let path = match Self::config_path() {
            Ok(p) => p,
            Err(_) => return Self::default(),
        };

        if !path.exists() {
            return Self::default();
        }

        let data = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(_) => return Self::default(),
        };

        match serde_json::from_str::<ToolsConfig>(&data) {
            Ok(mut cfg) => {
                // 안전을 위해 항상 order 기준으로 정렬
                cfg.tools.sort_by_key(|t| t.order);
                cfg
            }
            Err(_) => {
                // 파싱 실패 시 백업 후 초기화
                let backup = path.with_extension("json.bak");
                let _ = fs::copy(&path, &backup);
                Self::default()
            }
        }
    }

    /// 현재 설정을 JSON 파일로 저장한다.
    pub fn save(&self) -> io::Result<()> {
        let path = Self::config_path()?;
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        // 임시 파일에 먼저 쓴 뒤 rename하여 원자적으로 교체
        let tmp = path.with_extension("json.tmp");
        fs::write(&tmp, json)?;
        // Windows 에서는 대상이 존재하면 rename이 실패할 수 있으므로 사전에 제거
        if path.exists() {
            let _ = fs::remove_file(&path);
        }
        fs::rename(&tmp, &path)?;
        Ok(())
    }

    /// 새 툴을 추가한다 (자동으로 고유 ID 와 order 부여)
    pub fn add_tool(&mut self, name: String, path: String, args: String) -> &Tool {
        let new_id = self.next_id();
        let next_order = self.tools.iter().map(|t| t.order).max().unwrap_or(0) + 1;
        self.tools.push(Tool {
            id: new_id,
            name,
            path,
            args,
            icon_path: String::new(),
            order: next_order,
        });
        self.tools.last().expect("방금 추가한 툴이 존재해야 함")
    }

    /// 주어진 id 의 툴을 제거한다.
    pub fn remove_tool(&mut self, id: &str) {
        self.tools.retain(|t| t.id != id);
        self.renumber_order();
    }

    /// 두 툴의 위치를 교환한다 (UI 의 위/아래 버튼에서 사용)
    pub fn swap(&mut self, index_a: usize, index_b: usize) {
        if index_a < self.tools.len() && index_b < self.tools.len() && index_a != index_b {
            self.tools.swap(index_a, index_b);
            self.renumber_order();
        }
    }

    /// 특정 id 를 가진 툴의 가변 참조를 반환한다.
    pub fn find_mut(&mut self, id: &str) -> Option<&mut Tool> {
        self.tools.iter_mut().find(|t| t.id == id)
    }

    /// 특정 id 를 가진 툴의 불변 참조를 반환한다.
    pub fn find(&self, id: &str) -> Option<&Tool> {
        self.tools.iter().find(|t| t.id == id)
    }

    /// 사용되지 않은 다음 ID 를 "tool_NNN" 형식으로 생성한다.
    fn next_id(&self) -> String {
        let mut n = 1u32;
        loop {
            let candidate = format!("tool_{:03}", n);
            if !self.tools.iter().any(|t| t.id == candidate) {
                return candidate;
            }
            n += 1;
        }
    }

    /// order 필드를 현재 벡터 순서대로 1..N 으로 재할당한다.
    fn renumber_order(&mut self) {
        for (i, t) in self.tools.iter_mut().enumerate() {
            t.order = (i as u32) + 1;
        }
    }
}

/// 주어진 경로의 파일명(확장자 제외)을 추출한다. 드래그 앤 드롭 시 이름 자동 생성에 사용.
pub fn file_stem_name(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string()
}
