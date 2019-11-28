use structopt::StructOpt;
extern crate Lotus;
use Lotus::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "my finance")]
struct Opt {
    #[structopt(short, long)]
    interest: f64,

    #[structopt(short, long)]
    discount_rate: f64,

    #[structopt(skip)]
    monthly_discount_rate: f64,

    #[structopt(short, long)]
    loan: u64,

    #[structopt(short, long)]
    amortization: u64,
}

#[derive(Debug, Clone)]
struct Payment {
    month: u64,
    interest: f64,
    amortization: u64
}

fn year_to_month_discount (yearly_rate: f64) -> f64 {
    (yearly_rate + 1.0).powf(1.0 / 12.0) - 1.0
}

fn main () {
  let kr = LotusBuilder::default()
    .symbol("Kr")
    .precision(1)
    .format_positive("{value} {symbol} ")
    .format_negative("{value} {symbol}")
    .format_zero("0.00 {symbol}")
    .decimal_str(".")
    .thousand_str(" ")
    .build()
    .unwrap();

    let mut opt = Opt::from_args();
    opt.monthly_discount_rate = year_to_month_discount(opt.discount_rate);

    let mut payments: Vec<Payment> = vec![];

    let mut curr_loan = opt.loan;
    let mut index = 0;

    while curr_loan > 0 {
      let amortization = if curr_loan < opt.amortization {
        curr_loan
      } else {
        opt.amortization
      };
      let payment = Payment {
        month: index % 12,
        interest: curr_loan as f64 * opt.interest / 12.0,
        amortization: amortization
      };

      curr_loan -= amortization;
      payments.push(payment);
      index += 1;
    }

    let total_interest_cost = payments.iter().fold(0.0, |sum, payment| sum + payment.interest);

    let mut index = 1.0;
    let npv = payments.iter().fold(0.0, |sum, payment| {
      let x = sum as f64 + (payment.interest / (1.0 + opt.monthly_discount_rate).powf(index + 1.0));
      index += 1.0;
      x
    });

    println!("Num months: {}", payments.len());
    println!("Num years: {}", payments.len() / 12);
    println!("Total interest cost: {}", kr.format(total_interest_cost));
    println!("NPV: {}", npv);
}