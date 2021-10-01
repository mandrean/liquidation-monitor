use rust_decimal::Decimal;

/// Calculates the liquidation price based on the amount of bLUNA collateral
pub fn liquidation_price(
    loan_amount: &Decimal,
    bluna_collateral: &Decimal,
    max_ltv: &Decimal,
) -> Decimal {
    loan_amount / (bluna_collateral * max_ltv)
}

/// A variation of `liquidation_price` that also takes bETH collateral into account,
/// assuming a fixed bETH price to calculate the bLUNA liquidation price
pub fn liquidation_price_multi(
    loan_amount: &Decimal,
    beth_collateral: &Decimal,
    beth_price: &Decimal,
    bluna_collateral: &Decimal,
    max_ltv: &Decimal,
) -> Decimal {
    ((loan_amount / max_ltv) - (beth_collateral * beth_price)) / bluna_collateral
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn calculates_correct_liquidation_price() {
        let loan_amount = Decimal::new(1_200_000_000, 6);
        let bluna_collateral = Decimal::new(100_000_000, 6);
        let max_ltv = Decimal::new(6, 1);
        let liq_price = liquidation_price(&loan_amount, &bluna_collateral, &max_ltv);
        assert_eq!(Decimal::new(20, 0), liq_price);
    }

    #[test]
    fn calculates_correct_multicollateral_liquidation_price() {
        let loan_amount = Decimal::new(10_000_000_000, 6);
        let beth_collateral = Decimal::new(5_000_000, 6);
        let beth_price = Decimal::new(2_800_000_000, 6);
        let bluna_collateral = Decimal::new(100_000_000, 6);
        let max_ltv = Decimal::new(6, 1);
        let liq_price = liquidation_price_multi(
            &loan_amount,
            &beth_collateral,
            &beth_price,
            &bluna_collateral,
            &max_ltv,
        );
        assert_eq!(Decimal::new(267, 1), liq_price.round_dp(1));
    }
}
