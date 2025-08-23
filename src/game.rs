use rand::seq::SliceRandom;

pub fn get_random_words<'a>(words: &'a [&'a str], count: usize) -> Vec<&'a str> {
    let mut rng = rand::thread_rng();
    words.choose_multiple(&mut rng, count).cloned().collect()
}
