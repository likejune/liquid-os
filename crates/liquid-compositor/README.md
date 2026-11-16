# Liquid Compositor

Свой Wayland-композитор Liquid OS на Rust.

Готово (не зависит от Smithay, покрыто тестами):
- `shell_layout` — геометрия верхней панели, дока, меню приложений, пункта управления и уведомлений по макетам;
- `decorations` — три кнопки окна слева сверху и их попадание;
- `effects` — параметры шейдера стекла из дизайн-токенов;
- `shaders/` — размытие dual Kawase и стекло Liquid Glass (GLSL ES 1.00).

Следующий шаг (требует компилятора и теста в VM):
1. `src/backend/` на Smithay: udev/DRM для реального железа и winit для отладки в окне.
2. xdg-shell, layer-shell (для Liquid Shell), xdg-decoration (наши декорации), Xwayland.
3. Проход рендера: фон → размытие (blur_down ×N, blur_up ×N) → glass.frag под каждой стеклянной поверхностью → содержимое окна.

Шейдеры рассчитаны на соглашения Smithay `GlesRenderer` (текстура `tex`, координаты `v_coords`, `alpha`); сверить с используемой версией Smithay.
