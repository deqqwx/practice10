fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    let n = n.rem_euclid(len as isize) as usize; // Обчислюємо дійсний зсув

    // Виконуємо обертання
    if n == 0 || len == 0 {
        return s; // Якщо зсув 0 або рядок порожній, повертаємо оригінал
    }

    let split_index = len - n; // Індекс для розподілу рядка
    let (left, right) = s.split_at(split_index); // Розділяємо рядок
    format!("{}{}", right, left) // З'єднуємо в правильному порядку
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh";
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        shifts.iter().for_each(|(n, exp)| {
            assert_eq!(rotate(s.to_string(), *n), exp.to_string());
        });
    }
}
