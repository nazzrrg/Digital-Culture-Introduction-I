use std::env;
use itertools::Itertools;
use std::fs;
use std::cmp;


fn read_files(file_1: &String, file_2: &String) -> (String, String) {
	let text = fs::read_to_string(file_1).expect("Error");
	let dictionary_string = fs::read_to_string(file_2).expect("Error");

	(text, dictionary_string)
}

fn split_string_into_dict_element(s: &str) -> (&str, usize) {
	let v: Vec<&str> = s.split(" ").collect();
	let word = v[0];
	let number: usize= v[1].parse().unwrap();

	(word, number)
}

fn capitalise(input: &str) -> String {
    let mut v: Vec<char> = input.chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    let result: String = v.into_iter().collect();
    result
}

fn calculations(text: &Vec<&str>, dict: &Vec<(&str, usize)>) {
	let words_count = text.len();
	let text_of_different_words: Vec<&&str> = text.into_iter().unique().collect();
	let different_words_count = text_of_different_words.len();

	let mut dictionary_words: Vec<&str> = Vec::new();

	for element in dict {
		dictionary_words.push(element.0);
	}


	let matches_in_dictionary = {
								 	let mut result: usize = 0;
								 	for s1 in &text_of_different_words {
								 		for s2 in &dictionary_words {
								 			if *s1 == s2 {
								 				result+=1;
								 			}
								 		}
								 	}
								 	result
								};

	println!("Word count: {:?}", words_count);
	println!("Different word count: {:?}", different_words_count);
	println!("Matches in dictionary: {:?}", matches_in_dictionary);
}

fn split_into_strings(s: &str, pos: usize) -> (String, String) {
	let mut v1: Vec<char> = Vec::new();
	let mut v2: Vec<char> = Vec::new();
	let mut index = 0;

	for symbol in s.chars() {
		if index < pos {
			v1.push(symbol);
		} else {
			v2.push(symbol);
		}
		index += 1;
	}

	let s1: String = v1.into_iter().collect();
	let s2: String = v2.into_iter().collect();

	(s1, s2)
}

fn levenstein_distance(word_1: &str, word_2: &str) -> usize{
	let word_1 = word_1.chars().collect::<Vec<char>>();
	let word_2 = word_2.chars().collect::<Vec<char>>();
	
	let m = word_1.len();
	let n = word_2.len();
	let mut d = vec![vec![0; n + 1]; m + 1];

	d[0][0] = 0;

	for j in 1..(n + 1) {
		d[0][j] = d[0][j - 1] + 1;
	}

	for i in 1..(m + 1) {
		d[i][0] = d[i-1][0] + 1;
		for j in 1..(n + 1) {
			if word_1[i - 1] != word_2[j - 1] {
				d[i][j] = cmp::min(d[i - 1][j], cmp::min(d[i][j-1], d[i-1][j-1]));
				d[i][j] += 1;
			} else {
				d[i][j] = d[i-1][j-1];
			}
		}
	}

	d[m][n]
}

fn redactors_distance (word: &str, dictionary: &Vec<(&str, usize)>) -> (usize, usize){
	let mut result: usize = 10;
	let m = word.len()/2;
	let mut frequency = 0;
	let mut index = 0;
	let mut t = 0;

	for element in dictionary {
		let referrence_word = element.0;
		let mut intermediate_result = 0;
		let levenstein_distance_0 = levenstein_distance(word, referrence_word);

		for i in 1..m {
			let (s1, s2) = split_into_strings(word, i);

			intermediate_result = levenstein_distance(&s1, referrence_word) +
                levenstein_distance(&s2, referrence_word) + 1;
		}

		if cmp::min(levenstein_distance_0, intermediate_result) < result {
			result = cmp::min(levenstein_distance_0, intermediate_result);
			index = t;
			frequency = element.1;
		} else if cmp::min(levenstein_distance_0, intermediate_result) <= result
                  && frequency < element.1 {
			frequency = element.1;
			index = t;
		}
		t += 1;
	}
	(index, result)
}

fn correct_text(text: &Vec<&str>, dict: &Vec<(&str, usize)>) -> Vec<(usize, usize, usize)>{
	let mut result: Vec<(usize, usize, usize)> = Vec::new();

	let mut dictionary_words: Vec<&str> = Vec::new();

	let mut text_index = 0;

	for element in dict {
		dictionary_words.push(element.0);
	}

	let mut count = 0;

	println!("Potential errors:");

	for word in text {
		let mut not_found = true;
		
		for ref_word in &dictionary_words {
			not_found &= !(word == ref_word);
		}

		if not_found {
			let (dict_index, distance) = redactors_distance(word, dict);
			print!("{} - ", word);
			if 0 < distance && distance < 3 {
				println!("{} - {}", dict[dict_index].0, distance);
			} else {
				println!("не найдено - >2");
			}

			if 0 < distance && distance < 3 {
				let mut flag = true;

				for mut element in &mut result {
					if text[element.0] == *word && element.1 == dict_index {
						(*element).2 += 1;
						flag = false;
					}
				}

				if flag {
					result.push((text_index, dict_index, 1));
				}
			}
			count += 1;
		}

		text_index += 1;
	}

	println!("Total: {}", count);

	result
}

fn main() {
	let args: Vec<String> = env::args().collect();

	let (text_string, dictionary_string) = read_files(&args[1], &args[2]);

	let mut result_text_string = text_string.clone();

	let text_string = text_string
		.replace(&['.', ',', '!', '?', ':', ';', '»', '«', '(', ')', '\r', '\n','\''][..], " ")
		.to_lowercase();

	let mut text: Vec<&str> = text_string.split(' ').collect();

	text.retain(|x| x.len() > 0);

	let dictionary_string = dictionary_string.replace('\r', "");

	let mut dictionary_elements: Vec<&str> = dictionary_string.split('\n').collect();
 
	dictionary_elements.retain(|x| x.len() > 0);

	let mut dictionary: Vec<(&str, usize)> = Vec::new();
	
	for s in &dictionary_elements {
		dictionary.push(split_string_into_dict_element(s));
	}

	calculations(&text, &dictionary);

	let correction_vector = correct_text(&text, &dictionary);

	for mistake in correction_vector {
		let mut from: String = text[mistake.0].to_string();
		let mut to: String = dictionary[mistake.1].0.to_string();
		from.insert(0, ' ');
		to.insert(0, ' ');

		result_text_string = result_text_string.replace(&from, &to);

		let mut from: String = capitalise(text[mistake.0]);
		let mut to: String = capitalise(dictionary[mistake.1].0);
		from.insert(0, ' ');
		to.insert(0, ' ');

		result_text_string = result_text_string.replace(&from, &to);
	}

	println!("\n\n{}\n\n", result_text_string);

	let result_text_string = result_text_string
		.replace(&['.', ',', '!', '?', ':', ';', '»', '«', '(', ')', '\r', '\n','\''][..], " ")
		.to_lowercase();

	let mut result_text: Vec<&str> = result_text_string.split(' ').collect();

	result_text.retain(|x| x.len() > 0);

	calculations(&result_text, &dictionary);
	print!("\n\n");
	let text_of_different_words: Vec<&str> = text.into_iter().unique().collect();
	for word in text_of_different_words {
		println!("{}", word);
	}
}
