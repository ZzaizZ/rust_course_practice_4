use std::ffi::{CStr, c_char};

mod blur;
mod params;

/// Применяет размытие к изображению в формате RGBA8.
///
/// Функция экспортируется по C ABI и вызывается хост-приложением через `libloading`.
/// Алгоритм: взвешенное среднее по круговой окрестности радиуса `radius`,
/// повторённое `iterations` раз.
///
/// # Safety
///
/// * `rgba_data` должен указывать на непрерывный буфер размером не менее
///   `width * height * 4` байт в формате RGBA8 (4 байта на пиксель).
/// * Буфер должен оставаться действительным на всё время выполнения функции.
/// * `params` должен быть либо нулевым указателем, либо указывать на корректную
///   C-строку в кодировке UTF-8 с YAML-содержимым.
///
/// При нулевом `rgba_data` или нулевых размерах функция завершается досрочно
/// без каких-либо изменений.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
) {
    if rgba_data.is_null() || width == 0 || height == 0 {
        return;
    }

    let params_str = if params.is_null() {
        ""
    } else {
        let c_str = unsafe { CStr::from_ptr(params) };
        c_str.to_str().unwrap_or("")
    };

    let params = params::parse_params(params_str);

    let len = (width * height * 4) as usize;
    let data: &mut [u8] = unsafe { std::slice::from_raw_parts_mut(rgba_data, len) };

    blur::blur(data, width, height, &params);
}
