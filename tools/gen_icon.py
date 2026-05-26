"""
SnapLaunch 아이콘 생성기.

256x256 마스터 이미지를 Pillow 로 직접 그린 뒤
- assets/icon.png  (런타임 egui 창 아이콘용)
- assets/icon.ico  (16/32/48/64/128/256, EXE 임베드용)
두 파일을 생성한다.

디자인:
- 파란 라운드 사각형 배경
- 가운데 흰색 번개(snap·launch 의 빠름을 상징)
- 우상단 작은 금색 별(favorite/즐겨찾기 의미)
"""
from __future__ import annotations

import math
import os
import sys

from PIL import Image, ImageDraw, ImageFilter

# ──────────────────────────────────────────────────────────────
# 출력 경로
# ──────────────────────────────────────────────────────────────
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
ASSETS = os.path.join(ROOT, "assets")
os.makedirs(ASSETS, exist_ok=True)

PNG_PATH = os.path.join(ASSETS, "icon.png")
ICO_PATH = os.path.join(ASSETS, "icon.ico")

# ──────────────────────────────────────────────────────────────
# 색상 팔레트 (RGBA)
# ──────────────────────────────────────────────────────────────
COLOR_BG_TOP = (62, 122, 240, 255)       # 밝은 파랑
COLOR_BG_BOT = (24, 64, 175, 255)        # 진한 파랑
COLOR_BOLT = (255, 255, 255, 255)        # 흰 번개
COLOR_BOLT_SHADOW = (10, 30, 90, 90)     # 번개 그림자
COLOR_STAR = (252, 211, 77, 255)         # 금색 별
COLOR_STAR_OUTLINE = (180, 130, 20, 255)

SIZE = 256
CORNER_RADIUS = 52


# ──────────────────────────────────────────────────────────────
# 헬퍼: 위→아래 선형 그라데이션 이미지 생성
# ──────────────────────────────────────────────────────────────
def vertical_gradient(size: int, top: tuple, bottom: tuple) -> Image.Image:
    img = Image.new("RGBA", (size, size))
    px = img.load()
    for y in range(size):
        t = y / (size - 1)
        r = round(top[0] + (bottom[0] - top[0]) * t)
        g = round(top[1] + (bottom[1] - top[1]) * t)
        b = round(top[2] + (bottom[2] - top[2]) * t)
        a = round(top[3] + (bottom[3] - top[3]) * t)
        for x in range(size):
            px[x, y] = (r, g, b, a)
    return img


# ──────────────────────────────────────────────────────────────
# 헬퍼: n각 별 폴리곤 좌표
# ──────────────────────────────────────────────────────────────
def star_points(cx: float, cy: float, r_out: float, r_in: float, n: int = 5):
    pts = []
    for i in range(2 * n):
        r = r_out if i % 2 == 0 else r_in
        # 위(12시 방향)에서 시작
        a = -math.pi / 2 + i * math.pi / n
        pts.append((cx + r * math.cos(a), cy + r * math.sin(a)))
    return pts


# ──────────────────────────────────────────────────────────────
# 메인 그리기 로직
# ──────────────────────────────────────────────────────────────
def render_master() -> Image.Image:
    # 1) 라운드 사각형 마스크
    mask = Image.new("L", (SIZE, SIZE), 0)
    ImageDraw.Draw(mask).rounded_rectangle(
        [(0, 0), (SIZE - 1, SIZE - 1)],
        radius=CORNER_RADIUS,
        fill=255,
    )

    # 2) 그라데이션 배경을 마스크로 잘라낸다
    gradient = vertical_gradient(SIZE, COLOR_BG_TOP, COLOR_BG_BOT)
    canvas = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    canvas.paste(gradient, (0, 0), mask)

    # 3) 흰색 번개 폴리곤
    # 위쪽 우측 → 아래쪽 좌측으로 향하는 6점 지그재그
    bolt = [
        (155, 28),    # 0: 최상단(우)
        (76, 138),    # 1: 좌하단 (상부 사선의 끝)
        (116, 130),   # 2: 우측 jog (지그재그 허리 - 우)
        (100, 228),   # 3: 최하단(좌, 뾰족한 끝)
        (180, 118),   # 4: 우측 (하부 사선 시작점)
        (140, 126),   # 5: 좌측 jog (지그재그 허리 - 좌)
    ]

    # 3-1) 번개 그림자: 살짝 오프셋된 흐릿한 어두운 폴리곤
    shadow = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    ImageDraw.Draw(shadow).polygon(
        [(x + 4, y + 6) for (x, y) in bolt],
        fill=COLOR_BOLT_SHADOW,
    )
    shadow = shadow.filter(ImageFilter.GaussianBlur(radius=4))
    canvas = Image.alpha_composite(canvas, shadow)

    # 3-2) 흰색 번개 본체
    draw = ImageDraw.Draw(canvas)
    draw.polygon(bolt, fill=COLOR_BOLT)

    # 4) 우상단 금색 별 (favorite 상징)
    star = star_points(cx=204, cy=56, r_out=26, r_in=12, n=5)
    # 별 그림자
    sh = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    ImageDraw.Draw(sh).polygon(
        [(x + 2, y + 3) for (x, y) in star],
        fill=(0, 0, 0, 100),
    )
    sh = sh.filter(ImageFilter.GaussianBlur(radius=2))
    canvas = Image.alpha_composite(canvas, sh)

    draw = ImageDraw.Draw(canvas)
    draw.polygon(star, fill=COLOR_STAR, outline=COLOR_STAR_OUTLINE)

    return canvas


def main() -> int:
    master = render_master()

    # 1) 256px PNG 저장 — egui 창 아이콘으로 include_bytes! 사용
    master.save(PNG_PATH, "PNG")
    print(f"  wrote {PNG_PATH}  ({master.size[0]}x{master.size[1]})")

    # 2) 멀티 해상도 ICO 저장 — Windows 탐색기/작업표시줄 표시용
    ico_sizes = [(16, 16), (24, 24), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)]
    master.save(ICO_PATH, format="ICO", sizes=ico_sizes)
    print(f"  wrote {ICO_PATH}  (sizes: {', '.join(f'{w}' for w, _ in ico_sizes)})")

    return 0


if __name__ == "__main__":
    sys.exit(main())
