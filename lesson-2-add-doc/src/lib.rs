//! Математическая библиотека

/// Складывает два целых числа
///
/// # Примеры
///
/// Базовое использование:
/// ```
/// let result = my_math::add(2, 3);
/// assert_eq!(result, 5);
/// ```
///
/// Работа с отрицательными числами:
/// ```
/// let result = my_math::add(-5, 10);
/// assert_eq!(result, 5);
/// ```
///
/// Можно использовать в выражениях:
/// ```
/// if my_math::add(1, 2) == 3 {
///     println!("Математика работает!");
/// }
/// ```
pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
