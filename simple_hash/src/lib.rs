use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut res: HashMap<&str, usize> = HashMap::new();
    for word in words {
        let count = match res.get(word){
            Some(c) => c+1,
            None => 0, 
        };
        res.insert(word,count);
    }
    res
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        const SENTENCE: &str = "this is a very basic sentence with only a few repetitions. once again this is very basic but it should be enough for basic tests";

        let words = SENTENCE.split_ascii_whitespace().collect::<Vec<_>>();
        let frequency_count = word_frequency_counter(&words);

        println!("{:?}", frequency_count);
        println!("{}", nb_distinct_words(&frequency_count));
    }
}
