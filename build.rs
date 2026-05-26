// 빌드 스크립트: Windows 타겟에서 EXE 에 아이콘과 버전 정보를 임베드한다.
// 다른 플랫폼에서는 아무 일도 하지 않는다.

fn main() {
    println!("cargo:rerun-if-changed=assets/icon.ico");
    println!("cargo:rerun-if-changed=build.rs");

    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("FileDescription", "SnapLaunch — 자주사용하는 툴 즐겨찾기");
        res.set("ProductName", "SnapLaunch");
        res.set("LegalCopyright", "MIT License");
        if let Err(e) = res.compile() {
            // 리소스 컴파일러(rc.exe / windres) 가 없으면 경고만 남기고 빌드는 진행
            println!("cargo:warning=리소스 임베드 실패: {}", e);
        }
    }
}
