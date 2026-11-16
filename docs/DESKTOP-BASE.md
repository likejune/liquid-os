# Основа рабочего стола: компоненты COSMIC

Решение 16.11.2026. Писать композитор, панель, файловый менеджер, настройки и экран входа с нуля — годы работы. По правилу «что долго писать — берём готовое» за основу взяты компоненты COSMIC (System76): они написаны на Rust, на Smithay и iced (те же, что выбраны для Liquid), и в композиторе уже есть размытие, скругления и тени (шейдеры blur_downsample, blur_upsample, rounded_rectangle, shadow в epoch-1.11.0).

Внешний вид полностью заменяется на Liquid: токены из `crates/liquid-design`, шейдер стекла, иконки, обои, раскладка из `liquid-compositor::shell_layout`, логика из `liquid-shell`, `liquid-finder`, `liquid-settings`.

| Компонент Liquid | Основа | Что меняем |
|---|---|---|
| Liquid Compositor | cosmic-comp | стекло, кромки, кнопки окна, анимации |
| Верхняя панель и док | cosmic-panel | раскладка и стиль по макетам |
| Меню приложений | cosmic-launcher | центрированная панель, сетка 4 колонки |
| Finder | cosmic-files | боковая панель, теги, имя |
| Настройки | cosmic-settings | разделы и оформление |
| Блокировка и вход | cosmic-greeter | макет экрана блокировки |
| Уведомления, OSD, обои, порталы | cosmic-notifications, cosmic-osd, cosmic-bg, xdg-desktop-portal-cosmic | стиль |
| Заметки, О системе, Центр обновлений, Установщик | свои (на libcosmic) | — |

Лицензия: форки COSMIC остаются GPL-3.0 (собственные крейты Liquid — Apache-2.0, совместимы). Исходники патчей публикуются вместе с образом.

Сборка: `scripts/build-desktop.sh` или workflow `desktop` (контейнер Fedora 44).
