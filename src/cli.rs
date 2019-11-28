use crate::helpers::interest_to_monthly;
use structopt::StructOpt;
use std::fmt;
use crate::lotus::LotusBuilder;

#[derive(Debug, StructOpt)]
#[structopt(name = "my finance")]
pub struct Calculator {
    #[structopt(short, long)]
    pub interest: f32,

    #[structopt(short, long)]
    pub discount_rate: f32,

    #[structopt(skip)]
    pub monthly_discount_rate: f32,

    #[structopt(short, long)]
    pub loan: u32,

    #[structopt(short, long)]
    pub amortization: u32,

    #[structopt(skip)]
    pub payments: Vec<Payment>,
}

#[derive(Debug)]
pub struct Payment {
    pub month: u32,
    pub interest: f32,
    pub amortization: u32,
    pub interest_npv: f32,
}

impl fmt::Display for Calculator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kr = LotusBuilder::default()
            .symbol("Kr")
            .precision(2)
            .format_positive("{value} {symbol} ")
            .format_negative("{value} {symbol}")
            .format_zero("0.00 {symbol}")
            .decimal_str(".")
            .thousand_str(" ")
            .build()
            .expect("Failed to create currency formatting.");

        write!(f, "Months: {}\n", self.get_months()).unwrap();
        write!(f, "Years: {}\n", self.get_years()).unwrap();
        write!(
            f,
            "Total interest cost: {}\n",
            kr.format(self.get_total_interest())
        ).unwrap();
        write!(
            f,
            "Total interest NPV: {}\n",
            kr.format(self.get_total_interest_npv())
        ).unwrap();
        write!(
            f,
            "Total installments: {}\n",
            kr.format(self.get_total_installments())
        ).unwrap();
        write!(
            f,
            "Total installments NPV: {}",
            kr.format(self.get_total_installments_npv())
        )
    }
}

impl Calculator {
    pub fn new(interest: f32, discount_rate: f32, loan: u32, amortization: u32) -> Self {
        Calculator {
            interest,
            discount_rate,
            loan,
            amortization,
            monthly_discount_rate: interest_to_monthly(discount_rate),
            payments: vec![],
        }
    }
    pub fn new_from_cli() -> Self {
        let mut calculator = Calculator::from_args();
        calculator.monthly_discount_rate = interest_to_monthly(calculator.discount_rate);
        calculator
    }
    pub fn calculate_payments(mut self) -> Self {
        self.payments = vec![];
        let mut current_loan = self.loan;
        let mut index = 1;

        while current_loan > 0 {
            let interest = current_loan as f32 * self.interest / 12.0;
            let curr_amortization = if current_loan < self.amortization {
                current_loan
            } else {
                self.amortization
            };
            self.payments.push(Payment {
                month: index % 12,
                interest,
                amortization: curr_amortization,
                interest_npv: (interest / (1.0 + self.discount_rate).powf(index as f32 + 1.0)),
            });
            current_loan -= curr_amortization;
            index += 1;
        }
        self
    }
    pub fn get_total_interest(&self) -> f32 {
        self.payments
            .iter()
            .fold(0.0, |sum, payment| sum + payment.interest)
    }
    pub fn get_total_interest_npv(&self) -> f32 {
        self.payments
            .iter()
            .enumerate()
            .fold(0.0, |sum, (index, payment)| {
                sum as f32
                    + (payment.interest
                        / (1.0 + self.monthly_discount_rate).powf(index as f32 + 1.0))
            })
    }
    pub fn get_total_installments(&self) -> u32 {
        self.payments
            .iter()
            .fold(0, |sum, payment| sum + payment.amortization)
    }
    pub fn get_total_installments_npv(&self) -> f32 {
        self.payments
            .iter()
            .enumerate()
            .fold(0.0, |sum, (index, payment)| {
                sum as f32
                    + (payment.amortization as f32
                        / (1.0 + self.monthly_discount_rate).powf(index as f32 + 1.0))
            })
    }
    pub fn get_months(&self) -> usize {
        self.payments.len()
    }
    pub fn get_years(&self) -> usize {
        self.get_months() / 12
    }
}
