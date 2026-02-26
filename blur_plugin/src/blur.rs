use crate::params::Params;

/// Применяет взвешенное круговое размытие к изображению `image` в формате RGBA8.
///
/// Новый цвет каждого пикселя вычисляется как взвешенное среднее цветов соседей
/// в пределах круга радиуса `params.radius`. Вес соседа равен его расстоянию до
/// центрального пикселя (более далёкие пиксели влияют сильнее). Центральный
/// пиксель в усреднение не включается (его вес равен нулю).
///
/// Алгоритм повторяется `params.iterations` раз. Если `radius` или `iterations`
/// равен нулю, изображение остаётся без изменений.
///
/// # Аргументы
///
/// * `image`  — пиксельные данные в формате RGBA8 (4 байта на пиксель, построчно).
/// * `width`  — ширина изображения в пикселях.
/// * `height` — высота изображения в пикселях.
/// * `params` — параметры размытия.
pub(crate) fn blur(image: &mut [u8], width: u32, height: u32, params: &Params) {
    if params.radius == 0 || params.iterations == 0 {
        return;
    }

    for _ in 0..params.iterations {
        for y in 0..height {
            for x in 0..width {
                let mut r_sum = 0.0_f64;
                let mut g_sum = 0.0_f64;
                let mut b_sum = 0.0_f64;
                let mut a_sum = 0.0_f64;
                let mut weight_sum = 0.0_f64;

                let x_min = x.saturating_sub(params.radius);
                let y_min = y.saturating_sub(params.radius);
                let x_max = (x + params.radius).min(width - 1);
                let y_max = (y + params.radius).min(height - 1);

                for ny in y_min..=y_max {
                    for nx in x_min..=x_max {
                        let dx = nx as f64 - x as f64;
                        let dy = ny as f64 - y as f64;
                        let dist = (dx * dx + dy * dy).sqrt();

                        // Пиксели за пределами круглого радиуса пропускаем
                        if dist > params.radius as f64 {
                            continue;
                        }

                        // Центральный пиксель (расстояние = 0) имеет вес 0 — пропускаем
                        if dist == 0.0 {
                            continue;
                        }

                        let weight = dist; // вес = расстояние
                        let nidx = ((ny * width + nx) * 4) as usize;
                        r_sum += image[nidx] as f64 * weight;
                        g_sum += image[nidx + 1] as f64 * weight;
                        b_sum += image[nidx + 2] as f64 * weight;
                        a_sum += image[nidx + 3] as f64 * weight;
                        weight_sum += weight;
                    }
                }

                let idx = ((y * width + x) * 4) as usize;
                if weight_sum > 0.0 {
                    image[idx] = (r_sum / weight_sum).round() as u8;
                    image[idx + 1] = (g_sum / weight_sum).round() as u8;
                    image[idx + 2] = (b_sum / weight_sum).round() as u8;
                    image[idx + 3] = (a_sum / weight_sum).round() as u8;
                }
                // Если нет соседей (radius очень мал) — оставляем пиксель без изменений
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::params::Params;

    fn params(radius: u32, iterations: u32) -> Params {
        Params { radius, iterations }
    }

    /// Равномерное изображение 3×3 одного цвета.
    fn uniform_image(pixel: [u8; 4]) -> Vec<u8> {
        pixel.iter().cycle().take(4 * 9).cloned().collect()
    }

    #[test]
    fn test_zero_radius_no_change() {
        // При radius=0 изображение не должно изменяться
        let original = vec![10u8, 20, 30, 255, 40, 50, 60, 255];
        let mut image = original.clone();
        blur(&mut image, 2, 1, &params(0, 3));
        assert_eq!(image, original);
    }

    #[test]
    fn test_zero_iterations_no_change() {
        // При iterations=0 изображение не должно изменяться
        let original = vec![10u8, 20, 30, 255, 40, 50, 60, 255];
        let mut image = original.clone();
        blur(&mut image, 2, 1, &params(2, 0));
        assert_eq!(image, original);
    }

    #[test]
    fn test_single_pixel_unchanged() {
        // Единственный пиксель не имеет соседей — weight_sum=0, остаётся без изменений
        let original = vec![123u8, 45, 67, 200];
        let mut image = original.clone();
        blur(&mut image, 1, 1, &params(3, 5));
        assert_eq!(image, original);
    }

    #[test]
    fn test_uniform_color_unchanged() {
        // Однородное изображение: взвешенное среднее всех соседей равно тому же цвету
        let mut image = uniform_image([80, 120, 200, 255]);
        let original = image.clone();
        blur(&mut image, 3, 3, &params(2, 3));
        assert_eq!(image, original);
    }

    #[test]
    fn test_blur_row_of_three() {
        // 1×3: [255,0,0,255] | [0,0,0,255] | [255,0,0,255], radius=1.
        //
        // Алгоритм работает in-place (слева направо):
        //   пиксель 0: сосед (1,0) оригинальный [0,0,0,255] → новый [0,0,0,255]
        //   пиксель 1: сосед (0,0) уже [0,0,0,255] + (2,0) оригинальный [255,0,0,255]
        //              r = (0+255)/2 = 127.5 → 128
        //   пиксель 2: сосед (1,0) уже [128,0,0,255] → новый [128,0,0,255]
        let mut image = vec![255u8, 0, 0, 255, 0, 0, 0, 255, 255u8, 0, 0, 255];
        blur(&mut image, 3, 1, &params(1, 1));
        assert_eq!(image, vec![0, 0, 0, 255, 128, 0, 0, 255, 128, 0, 0, 255,]);
    }
}
