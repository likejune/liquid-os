# Патчи к готовым компонентам

`patches/<repo>/NNNN-описание.patch` применяются скриптом `scripts/fetch-upstream.sh` после клонирования закреплённой версии.

План патчей (см. docs/DESKTOP-BASE.md):
- cosmic-comp: шейдер Liquid Glass (crates/liquid-compositor/shaders/glass.frag) поверх их blur_downsample/blur_upsample; радужная кромка; три кнопки слева; скругление 24px.
- cosmic-panel: верхняя панель (меню приложения слева) + плавающий док на 3 приложения.
- cosmic-launcher: центрированное «Меню» с 3 закреплёнными и сеткой 4×N.
- cosmic-files → Finder: боковая панель и теги по макету.
- cosmic-settings: 11 разделов и страница «Оформление».
- cosmic-greeter: экран блокировки и входа по макету.
