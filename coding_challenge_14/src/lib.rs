pub mod diet {
    pub const NUTRITIONIST: &str = "Norah Nutrition";

    pub fn ask_about_program () {
        println!("The nutritionist is {:?}",NUTRITIONIST);
    }
}
pub mod weightlifting;
pub mod cardio;

pub use cardio::{CardioTool, Exercise as CardioExercise};
pub use weightlifting::Exercise as WeightliftingExercise;

#[derive(Debug)]
pub struct GymWorkout {
    cardio: CardioExercise,
    weightlifting: WeightliftingExercise,
}

impl GymWorkout {
    pub fn new () -> Self {
        diet::ask_about_program();
        cardio::ask_about_program();
        weightlifting::ask_about_program();

        Self {
            cardio: CardioExercise::new(String::from("Friday"), CardioTool::Treadmill, 60),
            weightlifting: WeightliftingExercise::new(String::from("Bench press"), 12 ),
        }
    }
}