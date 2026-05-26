# SnapLaunch

Windows 바탕화면/폴더 빈 공간 우클릭에 **자주 쓰는 프로그램 즐겨찾기 메뉴**를 추가하는 포터블 도구입니다. 관리자 권한이 필요 없고, 단일 EXE 로 동작합니다.

```
바탕화면 우클릭
  └── 🔧 자주사용하는 툴  ▶
        ├── ⭐ Notepad++
        ├── ⭐ VS Code
        ├── ⭐ 계산기
        ├── ──────────────
        ├── ➕ 툴 추가/관리...
        └── ❌ Uninstall
```

---

## 특징

- **포터블** — 모든 설정이 EXE 옆 `snap-launch-tools.json` 파일 하나로 관리됩니다. USB 에 담아 다른 PC 로 옮겨도 동작합니다.
- **관리자 권한 불필요** — `HKEY_CURRENT_USER` 만 사용합니다.
- **단일 실행 파일** — Rust + MSVC 정적 링킹으로 약 3.4 MB EXE 한 개. 별도 DLL/런타임 없음.
- **콘솔 창 없음** — `windows_subsystem = "windows"` 로 모든 모드에서 검은 창이 뜨지 않습니다.
- **죽은 항목 자동 감지** — 등록된 프로그램이 사라지면 GUI 에서 빨간색으로 표시되고, 메뉴에서 클릭 시 제거 여부를 묻습니다.
- **비상 복구 배치 파일** — 메뉴 설치 시 EXE 옆에 `SnapLaunch_복구.bat` 가 함께 생성되어, 본 프로그램이 통째로 삭제되어도 메뉴를 정리할 수 있습니다.

---

## 사용법

### 1) 최초 설치

1. `snap-launch.exe` 를 더블클릭하여 관리 창을 엽니다.
2. **"📁 찾아보기"** 로 자주 쓰는 EXE 를 등록하거나, EXE 파일을 창 안에 끌어다 놓습니다.
3. **"📥 메뉴 설치"** 버튼을 누릅니다.
4. 바탕화면이나 폴더 빈 공간에서 우클릭 → "자주사용하는 툴" 서브메뉴 확인.

### 2) 툴 추가/수정

- 컨텍스트 메뉴의 **"툴 추가/관리..."** 또는 `snap-launch.exe` 직접 실행 → 관리 창
- 추가/삭제/순서 변경 후 **"🔄 메뉴 갱신"** (변경이 있으면 파란색으로 강조됨)

### 3) 언인스톨

- 컨텍스트 메뉴의 **"Uninstall"** 항목 (가장 간편)
- 또는 관리 창의 **"📤 Uninstall"** 버튼
- 또는 CLI: `snap-launch.exe uninstall`

### 4) 비상 복구

`snap-launch.exe` 자체가 사라졌다면 EXE 가 있던 폴더의 **`SnapLaunch_복구.bat`** 을 더블클릭하세요. 컨텍스트 메뉴를 강제 정리합니다.

> ⚠ **권장**: EXE 를 삭제하기 전에 반드시 먼저 **"Uninstall"** 을 실행하세요. 양쪽 모두 사라지면 같은 폴더에 snap-launch.exe 를 다시 받아 `uninstall` 을 실행해야 합니다.

---

## CLI 사용법

| 명령 | 동작 |
|---|---|
| `snap-launch.exe` | 관리 GUI 열기 |
| `snap-launch.exe install` | 컨텍스트 메뉴 등록 |
| `snap-launch.exe uninstall` | 컨텍스트 메뉴 제거 |
| `snap-launch.exe launch <tool_id>` | 등록된 툴 실행 (컨텍스트 메뉴가 내부적으로 사용) |
| `snap-launch.exe launch-manage` | 관리 GUI 열기 (컨텍스트 메뉴 "툴 추가/관리..." 가 사용) |

---

## 설정 파일 형식

EXE 옆 `snap-launch-tools.json`:

```json
{
  "tools": [
    {
      "id": "tool_001",
      "name": "Notepad++",
      "path": "C:\\Program Files\\Notepad++\\notepad++.exe",
      "args": "",
      "icon_path": "",
      "order": 1
    },
    {
      "id": "tool_002",
      "name": "계산기",
      "path": "calc.exe",
      "args": "",
      "icon_path": "",
      "order": 2
    }
  ]
}
```

- `path` 는 절대 경로뿐 아니라 PATH 에 있는 명령(`calc.exe`, `notepad` 등)도 지원합니다.
- `icon_path` 가 비어 있으면 `path` 의 EXE 아이콘이 자동 사용됩니다.

---

## 빌드

요구: Rust stable (MSVC toolchain, Windows).

```powershell
cargo build --release
# 결과: target\release\snap-launch.exe
```

`.cargo/config.toml` 에서 `+crt-static` 를 설정하므로 빌드된 EXE 는 별도 Visual C++ 런타임이 필요 없습니다.

### 의존 라이브러리

- [`eframe`](https://crates.io/crates/eframe) / [`egui`](https://crates.io/crates/egui) — GUI
- [`winreg`](https://crates.io/crates/winreg) — Windows 레지스트리
- [`rfd`](https://crates.io/crates/rfd) — 네이티브 파일 선택 다이얼로그
- [`windows`](https://crates.io/crates/windows) — `MessageBoxW`
- [`serde`](https://crates.io/crates/serde) / [`serde_json`](https://crates.io/crates/serde_json) — 설정 직렬화

---

## 동작 원리

컨텍스트 메뉴는 다음 두 레지스트리 위치에 cascading 서브메뉴로 등록됩니다:

```
HKCU\Software\Classes\DesktopBackground\Shell\SnapLaunch   ; 바탕화면 우클릭
HKCU\Software\Classes\Directory\Background\Shell\SnapLaunch ; 폴더 빈 공간 우클릭
```

각 툴은 `shell\aaa_NNN_<id>` 하위 키로 등록되고, 클릭 시 `snap-launch.exe launch <id>` 가 호출되어 해당 프로그램을 `Command::spawn()` 으로 실행한 뒤 즉시 종료합니다 (백그라운드 상주 없음).

---

## 프로젝트 구조

```
src/
├── main.rs              CLI 인자 분기
├── app.rs               eframe 앱 + 한글 폰트 자동 로드
├── launcher.rs          launch <id> 처리 + 실행 실패 시 항목 제거 제안
├── msgbox.rs            MessageBoxW 래퍼 (info/error/confirm)
├── path_util.rs         툴 경로 유효성 검사 (PATH 탐색 포함)
├── config/
│   ├── mod.rs
│   └── tools.rs         tools.json 읽기/쓰기, Tool/ToolsConfig
├── registry/
│   ├── mod.rs           키 경로 상수, 복구 배치 경로
│   ├── install.rs       cascading 서브메뉴 작성 + 복구.bat 생성
│   ├── uninstall.rs     키 트리 + 복구.bat 정리
│   └── status.rs        설치 상태 / 등록된 EXE 경로 조회
└── ui/
    ├── mod.rs
    ├── tool_list.rs     툴 목록 표 (죽은 경로 빨간색 표시)
    ├── tool_form.rs     새 툴 추가 폼 + 파일 선택
    └── control_bar.rs   설치/제거/갱신 + 상태 라벨 + 경로 불일치 경고
```

---

## 라이선스

MIT
