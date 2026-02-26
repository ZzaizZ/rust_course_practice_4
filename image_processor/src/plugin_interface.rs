use libloading::Library;
use std::ffi::c_char;

/// Тип функции плагина, экспортируемой по C ABI.
///
/// Принимает размеры изображения, указатель на пиксельные данные в формате RGBA8
/// и указатель на строку параметров в формате YAML (может быть нулевым).
pub type ProcessImageFn =
    unsafe extern "C" fn(width: u32, height: u32, rgba_data: *mut u8, params: *const c_char);

/// Загруженный динамический плагин обработки изображений.
///
/// Держит открытой разделяемую библиотеку (`.so`/`.dylib`/`.dll`) на всё время
/// своего существования. При уничтожении объекта библиотека выгружается.
pub struct Plugin {
    lib: Library,
}

impl Plugin {
    /// Загружает разделяемую библиотеку по указанному пути.
    ///
    /// # Аргументы
    ///
    /// * `filename` — путь к файлу плагина (`.so`, `.dylib` или `.dll`).
    ///
    /// # Ошибки
    ///
    /// Возвращает [`libloading::Error`], если файл не найден или не является
    /// корректной разделяемой библиотекой.
    pub fn new(filename: &str) -> Result<Self, libloading::Error> {
        Ok(Plugin {
            lib: unsafe { Library::new(filename) }?,
        })
    }

    /// Возвращает указатель на функцию `process_image`, экспортируемую плагином.
    ///
    /// Функция ищется по символу `process_image` в таблице экспортов библиотеки
    /// и приводится к типу [`ProcessImageFn`].
    ///
    /// # Ошибки
    ///
    /// Возвращает [`libloading::Error`], если символ `process_image` отсутствует
    /// в загруженной библиотеке.
    pub fn interface(&self) -> Result<ProcessImageFn, libloading::Error> {
        Ok(unsafe { *self.lib.get(b"process_image\0")? })
    }
}
