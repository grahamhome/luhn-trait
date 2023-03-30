mod tests;

pub trait Verifiable: ToString {
    fn valid_luhn(&self) -> bool {
        self.to_string()
            .chars()
            .rev()
            .filter(|c| !c.is_whitespace())
            .try_fold((0, 0), |(sum, count), v| {
                v.to_digit(10)
                    .map(|n| if count % 2 == 1 { n * 2 } else { n })
                    .map(|n| if n > 9 { n - 9 } else { n })
                    .map(|n| (sum + n, count + 1))
            })
            .map_or(false, |(sum, count)| count > 1 && sum % 10 == 0)
    }
}

impl<T: ToString> Verifiable for T {}
