# SnapLaunch

<img src="assets/icon.png" align="left" width="96" hspace="16" />

Windows 바탕화면/폴더 빈 공간 **우클릭 메뉴**에 자주 쓰는 프로그램 즐겨찾기를 추가하는 포터블 도구.
관리자 권한 불필요, 단일 EXE, 한/영 지원.

<br clear="left" />

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

## 🎬 데모 페이지

- **GitHub Pages**: https://cflab2017.github.io/Tool_SnapLaunch/ *(저장소 Settings → Pages 에서 main 브랜치 `/docs` 활성화 필요)*
- **즉시 미리보기**: [htmlpreview 로 열기](https://htmlpreview.github.io/?https://github.com/cflab2017/Tool_SnapLaunch/blob/main/docs/index.html)
- **로컬**: `docs/index.html` 를 브라우저로 열어도 동작합니다.

우클릭 → "자주사용하는 툴" → 앱 실행 흐름이 12 초 주기로 자동 반복됩니다.

---

## 최신 버전 다운로드

| 버전 | 배포일 | 설치 파일 | 소스코드 |
|---|---|---|---|
| **v1.0.0** | 2026-05-27 | [📥 snap-launch.exe](https://github.com/cflab2017/Tool_SnapLaunch/releases/download/v1.0.0/snap-launch.exe) | [📦 Source (zip)](https://github.com/cflab2017/Tool_SnapLaunch/archive/refs/tags/v1.0.0.zip) · [📦 Source (tar.gz)](https://github.com/cflab2017/Tool_SnapLaunch/archive/refs/tags/v1.0.0.tar.gz) |

전체 릴리즈 이력: [Releases 페이지](https://github.com/cflab2017/Tool_SnapLaunch/releases)

### 설치 방법
1. `snap-launch.exe` 를 원하는 폴더(예: `C:\Tools\SnapLaunch\`) 에 둡니다.
2. 더블클릭으로 관리 창을 엽니다.
3. 자주 쓰는 프로그램을 등록 → **"📥 마우스 오른쪽 메뉴 설치"** 클릭.
4. 바탕화면/폴더 빈 공간을 우클릭하여 메뉴 확인.

> 🛡 백신이 막을 경우: 코드 서명을 하지 않은 Rust 빌드라 SmartScreen 이 가끔 경고할 수 있습니다. "추가 정보 → 실행" 으로 통과시키거나, 소스에서 직접 빌드하세요.

---

## 특징

- **포터블** — 모든 설정이 EXE 옆 `snap-launch-tools.json` 파일 하나로 관리. USB 에 담아 옮겨도 동작.
- **관리자 권한 불필요** — `HKEY_CURRENT_USER` 만 사용.
- **단일 실행 파일** — Rust + MSVC 정적 링킹으로 약 3.4 MB EXE. 별도 DLL/런타임 없음.
- **콘솔 창 없음** — `windows_subsystem = "windows"` 로 모든 모드에서 검은 창이 뜨지 않음.
- **한국어 / English** — GUI 상단 라디오로 즉시 전환. 우클릭 메뉴 텍스트도 같이 따라옴.
- **의미별 컬러 버튼** — 녹(주요)·빨(파괴)·파(동기화)·슬레이트(네비)·틸(유틸)·앰버(리셋)로 한눈에 구분.
- **변경 행 시각화** — 추가/편집/순서변경한 행이 옅은 파랑 + ● 으로 강조 → 메뉴 갱신 시 클리어.
- **죽은 경로 감지** — 등록된 EXE 가 사라지면 빨간 ⚠ 로 표시되고, 컨텍스트 메뉴 클릭 시 제거 여부를 묻습니다.
- **모든 위젯 툴팁** — 버튼/필드/라디오 위에 마우스 올리면 한/영으로 설명.
- **비상 복구 배치 파일** — 메뉴 설치 시 EXE 옆에 `SnapLaunch_복구.bat` 자동 생성 → snap-launch.exe 가 삭제돼도 메뉴 정리 가능.

---

## 사용법

### 1) 최초 설치
1. `snap-launch.exe` 실행 → 관리 창 열림
2. **"📁 찾아보기"** 로 EXE 등록 (또는 EXE 를 창에 드래그 앤 드롭)
3. **"📥 마우스 오른쪽 메뉴 설치"** 클릭
4. 바탕화면/폴더 빈 공간에서 우클릭하여 확인

### 2) 툴 추가/수정
- 컨텍스트 메뉴의 **"툴 추가/관리..."** 또는 `snap-launch.exe` 직접 실행
- 추가/삭제/순서 변경 후 옆쪽 **"🔄 메뉴 갱신"** (변경 보류 시 파란색으로 강조)

### 3) Uninstall
- 컨텍스트 메뉴의 **"Uninstall"** 항목 (가장 간편)
- 또는 관리 창의 **"📤 Uninstall"** 버튼
- 또는 CLI: `snap-launch.exe uninstall`

### 4) 비상 복구
`snap-launch.exe` 자체가 사라졌다면 EXE 가 있던 폴더의 **`SnapLaunch_복구.bat`** 을 더블클릭하세요.

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

모든 모드에서 메시지는 `tools.json` 에 저장된 언어 설정(`ko` / `en`)을 따라갑니다.

---

## 설정 파일 형식

EXE 옆 `snap-launch-tools.json`:

```json
{
  "language": "ko",
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

- `language`: `"ko"` 또는 `"en"`
- `path`: 절대 경로뿐 아니라 PATH 에 있는 명령(`calc.exe`, `notepad` 등)도 지원
- `icon_path` 가 비어 있으면 `path` 의 EXE 아이콘이 자동 사용

---

## 빌드

요구: Rust stable (MSVC toolchain, Windows).

```powershell
cargo build --release
# 결과: target\release\snap-launch.exe
```

`.cargo/config.toml` 에서 `+crt-static` 를 설정하므로 빌드된 EXE 는 별도 Visual C++ 런타임이 필요 없습니다.

아이콘을 재생성하려면 (한 번만):
```powershell
python tools/gen_icon.py
```

### 의존 라이브러리

- [`eframe`](https://crates.io/crates/eframe) / [`egui`](https://crates.io/crates/egui) — GUI
- [`winreg`](https://crates.io/crates/winreg) — Windows 레지스트리
- [`rfd`](https://crates.io/crates/rfd) — 네이티브 파일 선택 다이얼로그
- [`windows`](https://crates.io/crates/windows) — `MessageBoxW`
- [`serde`](https://crates.io/crates/serde) / [`serde_json`](https://crates.io/crates/serde_json) — 설정 직렬화
- [`winresource`](https://crates.io/crates/winresource) (build) — EXE 아이콘/메타데이터 임베드

---

## 동작 원리

컨텍스트 메뉴는 다음 두 레지스트리 위치에 cascading 서브메뉴로 등록됩니다:

```
HKCU\Software\Classes\DesktopBackground\Shell\SnapLaunch   ; 바탕화면 우클릭
HKCU\Software\Classes\Directory\Background\Shell\SnapLaunch ; 폴더 빈 공간 우클릭
```

각 툴은 `shell\aaa_NNN_<id>` 하위 키로 등록되고, 클릭 시 `snap-launch.exe launch <id>` 가 호출되어 해당 프로그램을 `Command::spawn()` 으로 실행한 뒤 즉시 종료합니다 (백그라운드 상주 없음).

"툴 추가/관리..." 와 "Uninstall" 항목은 `zzz_manage` / `zzz_uninstall` 키로 등록되며, 관리 항목에 `CommandFlags=0x20` (ECF_SEPARATORBEFORE) 를 두어 그 위에 자동 구분선이 그려집니다.

---

## 프로젝트 구조

```
src/
├── main.rs              CLI 인자 분기
├── app.rs               eframe 앱 + 한글 폰트 자동 로드 + 언어 라디오 + 창 제목 조립
├── i18n.rs              Language enum + Strings 테이블 (KO/EN)
├── launcher.rs          launch <id> 처리 + 실행 실패 시 항목 제거 제안
├── msgbox.rs            MessageBoxW 래퍼 (info/error/confirm)
├── path_util.rs         툴 경로 유효성 검사 (PATH 탐색 포함)
├── config/
│   ├── mod.rs
│   └── tools.rs         tools.json 읽기/쓰기, Tool/ToolsConfig
├── registry/
│   ├── mod.rs           키 경로 상수, 복구 배치 경로
│   ├── install.rs       cascading 서브메뉴 작성 + 복구.bat 생성 + 언어 적용
│   ├── uninstall.rs     키 트리 + 복구.bat 정리
│   └── status.rs        설치 상태 / 등록된 EXE 경로 조회
└── ui/
    ├── mod.rs
    ├── style.rs         색상 팔레트와 Button 팩토리 (primary/danger/info/nav/utility/warning)
    ├── tool_list.rs     툴 목록 표 + 죽은/변경 행 색칠 + 위/아래 + 메뉴 갱신
    ├── tool_form.rs     새 툴 추가 폼 + 파일 선택
    └── control_bar.rs   설치/제거 버튼 + 상태 라벨 + 경로 불일치 경고
```

---

## 제작자

**Joseph.han** · [coding-now.com](https://coding-now.com)

## 라이선스

MIT
