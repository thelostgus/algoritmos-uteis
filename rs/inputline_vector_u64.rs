fn inputline() -> String {
	let mut input_line = String::new();

	std::io::stdin()
		.read_line(&mut input_line)
		.expect("failed to read from stdin");

	return input_line.to_string().replace("\n", "");
}

fn splitline(string: String) -> Vec<String> {
	let itens: Vec<&str> = string.split_whitespace().collect();

	let mut result: Vec<String> = Vec::new();

	for i in itens {
		result.push(i.to_string());
	}
	
	return result;
}

fn convert_vector_string_to_u64(vetor: Vec<String>) -> Vec<u64> {
	let mut result: Vec<u64> = Vec::new();

	for i in vetor {
		result.push(i.to_string().parse::<u64>().unwrap());
	}

	return result;
}

fn inputline_vector_u64() -> Vec<u64> {
	return convert_vector_string_to_u64(splitline(inputline()));
}