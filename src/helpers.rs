pub fn interest_to_monthly(yearly_rate: f32) -> f32 {
    (yearly_rate + 1.0).powf(1.0 / 12.0) - 1.0
}
