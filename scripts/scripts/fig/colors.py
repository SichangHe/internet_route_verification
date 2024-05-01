from typing import Final

import numpy as np

R = 0.2126
"""Grayscale factor for red."""
G = 0.7152
"""Grayscale factor for green."""
B = 0.0722
"""Grayscale factor for blue."""


def linear_rgb_to_srgb(color: float) -> float:
    if color <= 0.0031308:
        return 12.92 * color
    return 1.055 * color ** (1 / 2.4) - 0.055


def srgb_to_linear_rgb(color: float) -> float:
    if color <= 0.04045:
        return color / 12.92
    return ((color + 0.055) / 1.055) ** 2.4


def hue_grayscale_to_linear_rgb(hue: float, grayscale: float):
    """
    Convert Hue and grayscale to linear RGB in range [0, 1].
    :h: Hue (0-360) with wrap-around.
    :grayscale: Grayscale (0-1) for linear RGB.

    Saturation is fixed to 1.

    References:
    <https://en.wikipedia.org/wiki/HSL_and_HSV#Hue_and_chroma>
    <https://en.wikipedia.org/wiki/HSL_and_HSV#Saturation>
    <https://en.wikipedia.org/wiki/Grayscale#Colorimetric_(perceptual_luminance-preserving)_conversion_to_grayscale>
    """
    h, p = hue, grayscale
    h = h % 360
    h_ = h / 60.0  # 0~6
    x, y, z = 0.0, 0.0, 0.0  # r, g, b ordered.

    if h_ > 5:  # r > b > g
        kx, ky, kz = R, B, G
        rgb = lambda: (x, z, y)
        k = 6 - h_
    elif h_ < 1:  # r > g > b
        kx, ky, kz = R, G, B
        rgb = lambda: (x, y, z)
        k = h_
    elif h_ < 2:  # g > r > b
        kx, ky, kz = G, R, B
        rgb = lambda: (y, x, z)
        k = 2 - h_
    elif h_ < 3:  # g > b > r
        kx, ky, kz = G, B, R
        rgb = lambda: (z, x, y)
        k = h_ - 2
    elif h_ < 4:  # b > g > r
        kx, ky, kz = B, G, R
        rgb = lambda: (z, y, x)
        k = 4 - h_
    else:  # b > r > g
        kx, ky, kz = B, R, G
        rgb = lambda: (y, z, x)
        k = h_ - 4

    # Because saturation is 1,
    # either z = 0:
    x, z = p / (ky * k + kx), 0.0
    y = k * x
    if x + z > 1:
        # Or x = 1:
        x, z = 1.0, (p - kx - (ky * k)) / (ky * (1.0 - k) + kz)
        y = (1.0 - k) * z + k

    assert kx * x + ky * y + kz * z - p < 0.0001, (
        locals(),
        "Grayscale recovery failed.",
    )
    r, g, b = rgb()
    for color in (r, g, b):
        assert color >= 0.0 and color <= 1.0, (locals(), "Color out of range.")
    return r, g, b


def hue_grayscale_to_srgb(hue: float, grayscale: float):
    """
    Convert Hue and grayscale to sRGB in range [0, 1].
    :h: Hue (0-360) with wrap-around.
    :grayscale: Grayscale (0-1) for sRGB.

    Saturation is fixed to 1.

    References:
    <https://en.wikipedia.org/wiki/SRGB#From_CIE_XYZ_to_sRGB>
    """
    p = srgb_to_linear_rgb(grayscale)
    r, g, b = hue_grayscale_to_linear_rgb(hue, p)
    return tuple(linear_rgb_to_srgb(color) for color in (r, g, b))


COLORS6: Final = tuple(
    hue_grayscale_to_srgb(hue, grayscale)
    for hue, grayscale in zip(
        [60, 180, 120, 240, 0, 300],
        np.linspace(0.96, 0.2, 6),
    )
)

COLORS5_OUT_OF6: Final = COLORS6[:3] + COLORS6[4:]

COLORS7: Final = tuple(
    hue_grayscale_to_srgb(hue, grayscale)
    for hue, grayscale in zip(
        [60, 180, 120, 240, 0, 30, 300],
        np.linspace(0.96, 0.04, 7),
    )
)
