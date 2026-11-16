# Ядро Liquid

- `config/liquid.config` — фрагмент конфигурации поверх конфига Fedora, подключается через штатный `kernel-local`.
- `patches/*.patch` — патчи Liquid, применяются по алфавиту после патчей Fedora.

## Сборка

На Fedora 44 запустите из корня репозитория:

```bash
bash scripts/build-kernel.sh
```

Сценарий скачивает исходный пакет kernel из ветки Fedora 44, добавляет конфигурацию и патчи Liquid, затем собирает базовый RPM с суффиксом `.liquid`. Debug-варианты, debuginfo, документация и selftests отключены, чтобы сборка помещалась в GitHub Actions.

Готовые пакеты появляются в `mkosi.packages/`. Mkosi автоматически создаёт из них локальный репозиторий и использует при сборке образа.

В GitHub Actions ту же сборку выполняет workflow **kernel** в контейнере Fedora 44; RPM сохраняются в артефакте `liquid-kernel`.
