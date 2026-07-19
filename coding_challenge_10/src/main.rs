#[derive(Debug)]
enum Tier {
    Gold,
    Silver,
    Platinum,
}

#[derive(Debug)]
enum Subscription {
    Free,
    Basic(f64, u32),
    Premuim {tier: Tier},
}

impl Subscription {
    fn summarize (&self) {
        match self {
            Subscription::Free => {
                println!("You have limited access to the site");
            },
            Subscription::Basic (price, months) => {
                println!("You have limited access to the site's premium features for {price} for {months} months");
            },
            Subscription::Premuim {tier} => {
                println!("You have full access to the site's premium features. Your tier is {tier:?}");
            },
        }
    }
}
fn main() {

    let s1 = Subscription::Free;
    s1.summarize();
    Subscription::Free.summarize();
    let s2 = Subscription::Basic(2.99, 12);
    s2.summarize();
    let s3 = Subscription::Premuim { tier: Tier::Platinum };
    s3.summarize();
}
