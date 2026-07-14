/*
Copyright 2026
Author : Kushal Prakash
Last updated : 11/07/2026
*/
const TOUCHDOWN_POINTS: i32 = 6;
fn main() {

    let season: &str = "winter";
    let mut points_scored: i32 = 28;

    points_scored = 35;

    let event_time = "06:00";

    let event_time = 6;
    
    println!("Season: {season} points: {points_scored}, event time: {event_time}, touchdown points: {TOUCHDOWN_POINTS}");
    println!("Season: {} points: {}, event time: {}, touchdown points: {}", season, points_scored, event_time, TOUCHDOWN_POINTS);
    println!("Season: {0} points: {1}, event time: {2}, touchdown points: {3}", season, points_scored, event_time, TOUCHDOWN_POINTS);

    let _favourite_beverage = "milk";

    #[allow(unused_variables)]
    let _favourite_beverage = "milk";


}
