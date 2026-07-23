use fitness::GymWorkout;

/* 

 
- Define a `new` constructor function on the GymWorkout struct.
  The function should invoke ALL 3 of the `ask_about_program`
  functions, then return an instance of the GymWorkout struct.
  Pick arbitrary values for the required struct fields.
 
Finally, in the binary crate root:
- Invoke the `GymWorkout::new` function and print out the
  GymWorkout struct in Debug format.
 
---
 
Here is what a sample output might look like:
 
The nutritionist is Norah Nutrition
The cardio trainer is Carl Cardio
The weightlifting trainer is Will Weight
GymWorkout {
    weightlifting: Exercise {
        name: "Bench Press",
        reps: 8,
    },
    cardio: Exercise {
        day: "Thursday",
        tool: Bike,
        minutes: 30,
    },
}
*/
fn main() {
    let workout = GymWorkout::new();
    println!("{:#?}", workout);
}
