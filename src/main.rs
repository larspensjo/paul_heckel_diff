use std::collections::HashMap;

type SymbolTable = HashMap<String, u32>;

fn main() {
	let x : i32 = 0;
	let start_sym = 'Ä';
	let end_sym = 'Ö';
	let v1 = vec![start_sym, 'a', 'b', 'c', 'd', 'e', end_sym];
	let v2 = vec![start_sym, 'a', 'b', 'c', 'd', 'e', end_sym];
	let mut symbol_table : SymbolTable = HashMap::new();
	symbol_table.insert(String::from("Daniel"), 0);
    println!("Hello, world! {:?}", symbol_table);
	count(&mut symbol_table, v1);
}

fn count(s : &mut SymbolTable,  v : Vec<char>) {
}