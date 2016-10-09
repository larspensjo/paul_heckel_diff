use std::collections::HashMap;

type Symbol = char;
type SymbolTable = HashMap<Symbol, u32>;

fn main() {
	let x : i32 = 0;
	let start_sym = 'Ä';
	let end_sym = 'Ö';
	let v1 = vec![start_sym, 'a', 'b', 'a', 'c', 'd', 'e', end_sym];
	let v2 = vec![start_sym, 'a', 'b', 'c', 'd', 'e', end_sym];
	let mut symbol_table : SymbolTable = HashMap::new();
    println!("Hello, world! {:?}", symbol_table);
	count(&mut symbol_table, v1);
    println!("New symbol table {:?}", symbol_table);
}

fn count(s : &mut SymbolTable,  v : Vec<char>) {
	for symbol in v {
		// let mut old : u32 = 0;
		let old = match s.get(&symbol) {
			Some(number) => *number,
			_ => 0,
		};
		s.insert(symbol, old+1);
		// s[&symbol] = s[&symbol] + 1;
		println!("Symbol: {}", symbol);
	}
}
