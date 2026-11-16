# Архитектура Liquid OS

| Слой | Компонент | Тип |
|---|---|---|
| Ядро | Linux с конфигурацией и патчами Liquid (`kernel/`) | Своё (C) |
| Загрузка | systemd-boot + UKI, счётчик попыток загрузки | Готовое |
| Система | systemd, dbus-broker, polkit, UPower, power-profiles-daemon | Готовое |
| Сеть | NetworkManager + iwd, BlueZ | Готовое |
| Звук | PipeWire + WirePlumber | Готовое |
| Графика | Mesa, драйвер NVIDIA, libinput, Xwayland | Готовое |
| Композитор | Liquid Compositor: форк cosmic-comp (Rust, Smithay) + свои шейдеры и раскладка | Готовое + своё |
| Дизайн | liquid-design (токены), Liquid UI (iced) | Своё |
| Оболочка | Liquid Shell: форки cosmic-panel, cosmic-launcher, cosmic-greeter, cosmic-notifications в стиле Liquid | Готовое + своё |
| Приложения | Finder, Заметки, О системе, Настройки, Центр обновлений | Своё |
| Приложения третьих сторон | Flatpak + Flathub, свои окна порталов | Готовое + своё |
| Установка | Liquid Installer и первая настройка (Rust) поверх systemd-repart | Своё |
| Обновления | systemd-sysupdate (A/B), dm-verity | Готовое |

## Разделы диска

| Раздел | Содержимое | Запись |
|---|---|---|
| ESP 1 ГБ | systemd-boot, два UKI (текущий и предыдущий) | только при обновлении |
| usr A + verity | система версии N (erofs) | нет |
| usr B + verity | система версии N+1 | только sysupdate |
| root (btrfs, TPM2) | /etc, /var, /home, Flatpak-приложения | да |

## Обновление и откат

1. systemd-sysupdate скачивает новую версию в свободный слот и кладёт UKI с счётчиком `+3`.
2. После перезагрузки systemd-boot грузит новую версию.
3. Если загрузка успешна, systemd-bless-boot помечает её хорошей.
4. После 3 неудачных попыток systemd-boot автоматически грузит предыдущую версию.

Подход основан на ParticleOS (образная ОС от разработчиков systemd).
