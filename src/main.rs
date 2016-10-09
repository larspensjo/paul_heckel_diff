use std::collections::HashMap;

type Symbol = char;
type SymbolTable = HashMap<Symbol, u32>;

fn main() {
	let start_sym = 'Ä';
	let end_sym = 'Ö';
	
	let v1 = vec![start_sym, 'A', 'B', 'C', 'D', 'E', 'G', end_sym];
	let v2 = vec![start_sym, 'D', 'E', 'F', 'G', 'A', 'C', end_sym];
	let mut symbol_table_old : SymbolTable = HashMap::new();
	let mut symbol_table_new : SymbolTable = HashMap::new();
	count(&mut symbol_table_old, v1);
	count(&mut symbol_table_new, v2);
    println!("old symbol table {:?}", symbol_table_old);
    println!("New symbol table {:?}", symbol_table_new);
}

fn count(s : &mut SymbolTable,  v : Vec<char>) {
	for symbol in v {
		let old = match s.get(&symbol) {
			Some(number) => *number,
			_ => 0,
		};
		s.insert(symbol, old+1);
		println!("Symbol: {}", symbol);
	}
}
