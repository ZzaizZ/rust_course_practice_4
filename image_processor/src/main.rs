mod plugin_interface;

use std::ffi::CString;
use std::path::Path;

use image::{ImageBuffer, Rgba};

use clap::Parser;
use log::{debug, error, info};

/// Аргументы командной строки для `image_processor`.
#[derive(Parser, Debug)]
struct Args {
    /// Путь к входному изображению.
    #[arg(long, required = true)]
    input: String,
    /// Путь к выходному изображению.
    #[arg(long, required = true)]
    output: String,
    /// Имя плагина (без префикса `lib` и расширения), например `blur_plugin`.
    #[arg(long, required = true)]
    plugin: String,
    /// Путь к YAML-файлу параметров плагина.
    #[arg(long, required = true)]
    params: String,
    /// Директория, в которой находится скомпилированный плагин.
    #[arg(long, default_value = "target/debug")]
    plugin_path: String,
    /// Уровень логирования: error, warn, info, debug, trace.
    #[arg(long, default_value = "warn")]
    log_level: log::LevelFilter,
}

/// Открывает изображение по пути `path` и переводит его в формат RGBA8.
///
/// # Ошибки
///
/// Возвращает [`image::ImageError`], если файл не найден или имеет
/// неподдерживаемый формат.
fn get_raw_image(path: &str) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, image::ImageError> {
    let img = image::open(path)?.into_rgba8();
    Ok(img)
}

/// Сохраняет изображение `img` в файл по пути `path`.
///
/// Формат определяется по расширению файла.
///
/// # Ошибки
///
/// Возвращает [`image::ImageError`], если не удалось записать файл.
fn save_as_png(path: &str, img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<(), image::ImageError> {
    img.save_with_format(path, image::ImageFormat::Png)?;
    Ok(())
}

/// Формирует полный путь к файлу плагина с учётом платформы.
///
/// На Linux добавляет префикс `lib` и суффикс `.so`,
/// на macOS — `lib` + `.dylib`, на Windows — `.dll`.
///
/// # Аргументы
///
/// * `plugin_name` — базовое имя плагина, например `blur_plugin`.
/// * `plugin_path` — директория, в которой расположен плагин.
fn build_plugin_path(plugin_name: &str, plugin_path: &str) -> String {
    let lib_name = if cfg!(target_os = "windows") {
        format!("{}.dll", plugin_name)
    } else if cfg!(target_os = "macos") {
        format!("lib{}.dylib", plugin_name)
    } else {
        format!("lib{}.so", plugin_name)
    };
    format!("{}/{}", plugin_path, lib_name)
}

/// Читает содержимое файла параметров целиком в строку.
///
/// # Ошибки
///
/// Возвращает [`std::io::Error`], если файл не существует или недоступен для чтения.
fn read_params(path: &str) -> Result<String, std::io::Error> {
    let params = std::fs::read_to_string(path)?;
    Ok(params)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    env_logger::Builder::new()
        .filter_level(args.log_level)
        .init();

    debug!("аргументы: {:?}", args);

    // Проверяем существование входного изображения до попытки его открыть
    if !Path::new(&args.input).exists() {
        error!("входной файл не найден: {}", args.input);
        return Err(format!("входной файл не найден: {}", args.input).into());
    }

    // Проверяем существование файла параметров до попытки его прочитать
    if !Path::new(&args.params).exists() {
        error!("файл параметров не найден: {}", args.params);
        return Err(format!("файл параметров не найден: {}", args.params).into());
    }

    let lib_path = &build_plugin_path(&args.plugin, &args.plugin_path);

    // Проверяем существование файла плагина до попытки его загрузить
    if !Path::new(lib_path).exists() {
        error!("файл плагина не найден: {}", lib_path);
        return Err(format!("файл плагина не найден: {}", lib_path).into());
    }

    info!("загрузка плагина: {}", lib_path);
    let plugin = plugin_interface::Plugin::new(lib_path)?;
    let img_transformer = plugin.interface()?;
    debug!("плагин загружен, символ process_image найден");

    info!("загрузка изображения: {}", args.input);
    let mut img = get_raw_image(&args.input)?;
    debug!("размер изображения: {}x{}", img.width(), img.height());

    let params = CString::new(read_params(&args.params)?)?;

    info!("обработка изображения плагином");
    unsafe {
        img_transformer(
            img.width(),
            img.height(),
            img.as_mut().as_mut_ptr(),
            params.as_ptr(),
        );
    }

    info!("сохранение результата: {}", args.output);
    save_as_png(&args.output, &img)?;
    info!("готово");
    Ok(())
}
