# image-processing — система плагинов обработки изображений

Проект, реализующий **расширяемую систему обработки изображений на основе
динамически загружаемых плагинов**. Хост-приложение (`image_processor`) загружает
разделяемые библиотеки (`.so` / `.dylib` / `.dll`) во время выполнения и вызывает стандартную C-ABI функцию `process_image`.

## Быстрый старт

```bash
# 1. Сборка всего воркспейса
cargo build --workspace

# 2. Применить размытие
cargo run -p image_processor -- \
  --input image.png \
  --output output.png \
  --plugin blur_plugin \
  --params blur_plugin/params.yaml

# 3. Применить отражение
cargo run -p image_processor -- \
  --input image.png \
  --output output.png \
  --plugin mirror_plugin \
  --params mirror_plugin/params.yaml
```

По умолчанию плагины ищутся в `target/debug/`. Для использования release-сборки
передайте `--plugin-path target/release` и предварительно выполните `cargo build --release`.
