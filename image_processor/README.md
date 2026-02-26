# image_processor

CLI-приложение — хост для системы плагинов обработки изображений. Загружает плагин
(`.so` / `.dylib` / `.dll`) во время выполнения, передаёт ему изображение и параметры,
затем сохраняет результат.

## Использование

```bash
image_processor [OPTIONS] --input <INPUT> --output <OUTPUT> --plugin <PLUGIN> --params <PARAMS>
```

### Аргументы

| Флаг | Обязательный | Описание |
| --- | --- | --- |
| `--input <путь>` | да | Путь к входному изображению (PNG, JPEG и др.) |
| `--output <путь>` | да | Путь для сохранения результата |
| `--plugin <имя>` | да | Имя плагина без префикса `lib` и расширения, например `blur_plugin` |
| `--params <путь>` | да | Путь к YAML-файлу параметров плагина |
| `--plugin-path <путь>` | нет | Директория с `.so`-файлами (по умолчанию `target/debug`) |
| `--log-level <уровень логгирования>` | нет | Уровень логгирования (по умолчанию `warn`) |

### Примеры

```bash
# Применить размытие (debug-сборка)
cargo run -p image_processor -- \
  --input photo.png \
  --output blurred.png \
  --plugin blur_plugin \
  --params blur_plugin/params.yaml

# Применить отражение с явным путём к плагину
cargo run -p image_processor -- \
  --input photo.png \
  --output mirrored.png \
  --plugin mirror_plugin \
  --params mirror_plugin/params.yaml \
  --plugin-path target/release
```
