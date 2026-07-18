    #[derive(Debug)]
    struct GlobalCoffee {
        price: f64,
        name: String,
        is_hot: bool,
    }

    struct Movies {
        title: String,
        release_year: u32,
        duration_min: u32,
    }

impl Movies {
    // fn display_movie_info (self) {
    //     println!("Title is {}", self.title);
    //     println!("release year is {}", self.release_year);
    //     println!("duration is {}", self.duration_min);

    // }

    // fn display_movie_info (self: &Self) {
    //     println!("Title is {}", self.title);
    //     println!("release year is {}", self.release_year);
    //     println!("duration is {}", self.duration_min);

    // }

    fn new(title: String, release_year: u32, duration_min: u32) -> Self {
        Movies {
            title,
            release_year,
            duration_min,
        }
    }

    fn display_movie_info (&self) {
        println!("Title is {}", self.title);
        println!("release year is {}", self.release_year);
        println!("duration is {}", self.duration_min);
        println!("Years since release is {}", self.years_since_release());

    }

    // fn double_length (mut self) {
    //     self.duration_min = self.duration_min*2;
    // }

    // fn double_length (self: &mut Movies) {
    //     self.duration_min = self.duration_min*2;
    // }

    fn double_length (&mut self) {
        self.duration_min = self.duration_min*2;
    }

    fn is_longer_than (&self, other:&Self) -> bool {
        self.duration_min > other.duration_min
    }

    fn years_since_release(&self) -> u32 {
        2026 - self.release_year
    }
}


fn main() {

    let film = Movies {
        title: String::from("Upendra"),
        release_year: 1999,
        duration_min: 150,
    };

    let film_2 = Movies {
        title: String::from("A"),
        release_year: 2000,
        duration_min: 149,
    };

    let film_3 = Movies::new(String::from("Om"), 1996, 160);



    film.display_movie_info();

    film_2.display_movie_info();

    film_3.display_movie_info();

    if film.is_longer_than(&film_2) {
        println!("{} is longer than {}", film.title, film_2.title);
    }

    struct Coffee {
        price: f64,
        name: String,
        is_hot: bool,
    }

    let mocha = Coffee {
        name: String::from("Mocha"),
        price: 2.99,
        is_hot: true,
    };


    let mut beverage = Coffee {
        name: String::from("Mocha"),
        price: 2.99,
        is_hot: true,
    };

    beverage.name = String::from("Macchiato");
    beverage.price = 3.99;

    println!("Today morning I had {} and it costed {} and was it hot {}", mocha.name, mocha.price, mocha.is_hot);

    println!("Today morning I had {} and it costed {} and was it hot {}", beverage.name, beverage.price, beverage.is_hot);

    let x_name = String::from("Latte");

    let cf = make_coffee(x_name, 2.5, true);

    let milk = GlobalCoffee {
        name: String::from("Milk"),
        ..cf
    };

    println!("Today morning I had {} and it costed {} and was it hot {}", cf.name, cf.price, cf.is_hot);

    println!("Today morning I had {} and it costed {} and was it hot {}", milk.name, milk.price, milk.is_hot);

    no_transfer_ownership(&cf);

    transfer_ownership(cf);

    println!("{:?}", milk);
    println!("{:#?}", milk);

}

fn make_coffee (name:String, price:f64, is_hot:bool ) -> GlobalCoffee {
    GlobalCoffee { price:price, name:name, is_hot:is_hot }
}

fn transfer_ownership (mut global_coffee: GlobalCoffee) {

    global_coffee.price = 5.99;

    println!("Today morning I had {} and it costed {} and was it hot {}", global_coffee.name, global_coffee.price, global_coffee.is_hot);

}

fn no_transfer_ownership (global_coffee: &GlobalCoffee) {

    println!("Today morning I had {} and it costed {} and was it hot {}", global_coffee.name, global_coffee.price, global_coffee.is_hot);

}