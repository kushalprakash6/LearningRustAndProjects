use crate::Milk::WholeMilk;

#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal{username: String, password: String},
}

enum OperatingSystems {
    Windows,
    MacOs,
    Linux,
}

enum LaundryCycle {
    Cold, 
    Hot{temp: u32},
    Delicate(String)
}

impl LaundryCycle {
    fn wash_laundry (&self) {
        match self {
            LaundryCycle::Cold => {
                println!("Running the laundry with cold temperature");
            },
            LaundryCycle::Hot { temp } => {
                println!("Running the laundry with {temp} degrees");
            },
            LaundryCycle::Delicate(fabric_type) => {
                println!("Running the laundry with delicate for cloth type {fabric_type} ");
            }
        }       
    }
}
#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlineOrderStatus {
    fn check(&self) {
        match self {
            OnlineOrderStatus::Ordered | OnlineOrderStatus::Packed => {
                println!("Orderd is being prepared for shipent");
            }
            OnlineOrderStatus::Delivered => {
                println!("Your ordred is delivered");
            }
            other_status    => {
                println!("Your order status {other_status:?}");
            }
        }
    }
}

enum Milk {
    LowFat(u32),
    WholeMilk,
}

impl Milk {
    fn drink (self) {
        match self {
            Milk::LowFat(2) => {
                println!("Delicious, 2% is my favourite");
            }
            Milk::LowFat(percentage) => {
                println!("Nice!! you have {percentage} % of fat in milk");
            }
            Milk::WholeMilk => {
                println!("You've got the whole milk");
            }
        }
    }
}

fn main() {

    let first_card = CardSuit::Hearts;
    let second_card = CardSuit::Diamonds;

    println!("{:#?}", first_card);

    let visa = PaymentMethodType::CreditCard(String::from("1234 5678 1234 5678"));

    let paypal = PaymentMethodType::PayPal { username: (String::from("abc@xyz.com")), password: (String::from("123456")) };

    let my_comp = OperatingSystems::MacOs;

    let os_age = years_since_release(my_comp);

    println!("My computers operating systems age is {os_age} years");

    // wash_laundry(LaundryCycle::Cold);
    // wash_laundry(LaundryCycle::Hot { temp: 100 });
    // wash_laundry(LaundryCycle::Delicate(String::from("Satin")));

    LaundryCycle::Cold.wash_laundry();
    let hot_cycle = LaundryCycle::Hot { temp: 100 };
    hot_cycle.wash_laundry();
    let delicate_cycle = LaundryCycle::Delicate(String::from("Linen"));
    delicate_cycle.wash_laundry();

    OnlineOrderStatus::Shipped.check();
    OnlineOrderStatus::Ordered.check();
    OnlineOrderStatus::Packed.check();
    OnlineOrderStatus::Delivered.check();

    let my_beverage = Milk::LowFat(2);
    if let Milk::LowFat(percent) = my_beverage {
        println!("you have {percent} % milk fat");
    }


}

fn years_since_release (os: OperatingSystems) -> u32 {
    match os {
        OperatingSystems::Linux => 35,
        OperatingSystems::MacOs => {
            println!("Quiet an old operating system");
            42
        }
            ,
        OperatingSystems::Windows => 41,
    }
}

// fn wash_laundry (cycle: LaundryCycle) {
//     match cycle {
//         LaundryCycle::Cold => {
//             println!("Running the laundry with cold temperature");
//         },
//         LaundryCycle::Hot { temp } => {
//             println!("Running the laundry with {temp} degrees");
//         },
//         LaundryCycle::Delicate(fabric_type) => {
//             println!("Running the laundry with delicate for cloth type {fabric_type} ");
//         }
//     }
// }