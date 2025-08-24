use rand::seq::SliceRandom;

pub fn get_random_words(words: &[String], count: usize) -> Vec<String> {
    let mut rng = rand::thread_rng();
    words.choose_multiple(&mut rng, count).cloned().collect()
}