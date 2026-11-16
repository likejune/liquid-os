# Liquid OS

Самостоятельная настольная ОС в стиле Liquid Glass. От Linux взяты ядро и Flatpak, низкоуровневые компоненты готовые, всё видимое пишется на Rust строго по дизайну. Система атомарная: раздел /usr только для чтения, обновления A/B и автооткат.

Статус: **0.9.1 pre-alpha**. Подробности: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md), [docs/DECISIONS.md](docs/DECISIONS.md), [docs/ROADMAP.md](docs/ROADMAP.md).

## Структура

```
crates/liquid-design/   дизайн-токены (цвета, скругления, стекло, тени, шрифт, анимации)
crates/liquid-ui/       стили компонентов и геометрия (кнопки, переключатели, слайдеры, списки, поверхности)
crates/liquid-compositor/ раскладка оболочки, декорации окон, шейдеры Liquid Glass
crates/liquid-shell/    состояние оболочки: пункт управления (без AirDrop), меню приложений, верхняя панель, уведомления
crates/liquid-finder/   Finder: боковая панель, теги, виды, сортировка, поиск
crates/liquid-notes/    Заметки: папки, счётчики, избранное, чек-листы, поиск
crates/liquid-sysinfo/  «О системе» с реальными данными компьютера
crates/liquid-settings/ Настройки: 11 разделов, страница «Оформление»
crates/liquid-update/   Центр обновлений поверх systemd-sysupdate
crates/liquid-format/   формат размеров и дат по-русски
crates/liquid-installer/ 15 экранов установки и первой настройки, план установки на диск
mkosi.conf, mkosi.conf.d/  описание образа системы
mkosi.repart/           разделы образа: ESP, /usr (erofs + dm-verity)
mkosi.extra/            файлы системы: слот B, раздел данных, sysupdate, os-release
upstream.lock           закреплённые версии готовых компонентов рабочего стола (COSMIC), см. docs/DESKTOP-BASE.md
patches/                патчи Liquid к ним
kernel/                 конфигурация и патчи ядра Liquid
.github/workflows/      ci.yml (Rust), kernel.yml (RPM ядра), desktop.yml (COSMIC), image.yml (образ + QEMU)
```

## Сборка

Код на Rust:
```bash
cargo test --workspace
```

Образ системы (нужен Linux, mkosi 25+ и зависимости из workflows):
```bash
bash scripts/build-kernel.sh
bash scripts/build-desktop.sh
mkosi -f build
bash scripts/test-image.sh
```
Или запустить workflow **image** в GitHub Actions вручную. Он последовательно собирает Liquid kernel и рабочий стол, добавляет их в системный образ и проверяет загрузку в QEMU.

## Лицензия

Собственный код: Apache-2.0 (см. LICENSE и NOTICE). Название и логотип Liquid OS в лицензию не входят.
