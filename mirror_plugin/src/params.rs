use serde::Deserialize;

/// Параметры плагина отражения, считываемые из YAML-файла.
#[derive(Deserialize, Debug)]
pub struct Params {
    /// Отражение по вертикали (верх ↔ низ). По умолчанию `false`.
    pub flip_v: bool,
    /// Отражение по горизонтали (лево ↔ право). По умолчанию `false`.
    pub flip_h: bool,
}

/// Разбирает строку `params` в формате YAML и возвращает [`Params`].
///
/// Если строка пуста или содержит некорректный YAML, возвращает безопасное
/// значение по умолчанию: `flip_v = false`, `flip_h = false` (операция-пустышка).
pub fn parse_params(params: &str) -> Params {
    if params.is_empty() {
        return Params {
            flip_v: false,
            flip_h: false,
        };
    }

    serde_yaml::from_str(params).unwrap_or(Params {
        flip_v: false,
        flip_h: false,
    })
}
