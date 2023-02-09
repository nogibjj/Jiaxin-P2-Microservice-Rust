/*A library that returns back a random movie name froom the list of the world top 10 best movies */

use rand::Rng;

//create an const array of the top 10 best movies around the world
pub const MOVIES: [&str; 10] = [
    "The Shawshank Redemption (1994)",
    "The Godfather (1972)",
    "The Godfather: Part II (1974)",
    "The Dark Knight (2008)",
    "12 Angry Men (1957)",
    "Schindler's List (1993)",
    "The Lord of the Rings: The Return of the King (2003)",
    "Pulp Fiction (1994)",
    "The Good, the Bad and the Ugly (1966)",
    "Forrest Gump (1994)",
];

//create a function that returns a random movie in the list above
pub fn random_movie() -> &'static str {
    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..MOVIES.len());
    MOVIES[random_index]
}
