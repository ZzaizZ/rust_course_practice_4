use serde::Deserialize;

/// Параметры плагина размытия, считываемые из YAML-файла.
#[derive(Deserialize, Debug)]
pub(crate) struct Params {
    /// Радиус круговой окрестности в пикселях. При значении `0` размытие не применяется.
    pub(crate) radius: u32,
    /// Количество повторений алгоритма. При значении `0` размытие не применяется.
    pub(crate) iterations: u32,
}

/// Разбирает строку `params` в формате YAML и возвращает [`Params`].
///
/// Если строка пуста или содержит некорректный YAML, возвращает безопасное
/// значение по умолчанию: `radius = 0`, `iterations = 0` (операция-пустышка).
pub(crate) fn parse_params(params: &str) -> Params {
    if params.is_empty() {
        return Params {
            radius: 0,
            iterations: 0,
        };
    }

    serde_yaml::from_str(params).unwrap_or(Params {
        radius: 0,
        iterations: 0,
    })
}
