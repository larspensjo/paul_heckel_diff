use std::collections::HashMap;

type Reference = usize;

#[derive(Debug)]
struct TokenInfo {
	count: u32,
	pos: Reference, // -1 represents no position
}
type Symbol = char;
type SymbolTable = HashMap<Symbol, TokenInfo>;

fn main() {
	let start_sym = 'Ä';
	let end_sym = 'Ö';
	
	let old_file = vec![start_sym, 'A', 'B', 'C', 'D', 'E', 'G', end_sym];
	let new_file = vec![start_sym, 'D', 'E', 'F', 'G', 'A', 'C', end_sym];
	let mut symbol_table_old : SymbolTable = HashMap::new();
	let mut symbol_table_new : SymbolTable = HashMap::new();
	count(&mut symbol_table_old, old_file);
	count(&mut symbol_table_new, new_file);
    // println!("old symbol table {:?}", symbol_table_old);
    // println!("New symbol table {:?}", symbol_table_new);
	let unique_symbols = unique(&symbol_table_old, &symbol_table_new);
	println!("Unique symbols: {:?}", unique_symbols);
	
	let mut old_mapping : Vec<Reference> = vec![0; symbol_table_old.len()];
	let mut new_mapping : Vec<Reference> = vec![0; symbol_table_new.len()];
	
	for symbol in unique_symbols {
		let TokenInfo{count:_, pos:line_new} = symbol_table_new[&symbol];
		let TokenInfo{count:_, pos:line_old} = symbol_table_old[&symbol];
		old_mapping[line_old] = line_new;
		new_mapping[line_new] = line_old;
	}
	
	println!("Mapping from old: {:?}", old_mapping);
	println!("Mapping from new: {:?}", new_mapping);
}

fn count(s : &mut SymbolTable,  v : Vec<Symbol>) {
	let mut line : Reference = 0;
	for symbol in v {
		let new : TokenInfo = match s.get(&symbol) {
			Some(info) => TokenInfo{count: info.count+1, pos: line},
			_ => TokenInfo {count:1, pos: line},
		};
		s.insert(symbol, new);
		line = line + 1;
	}
}

fn unique(old : &SymbolTable, new : &SymbolTable) -> Vec<Symbol> {
	let mut out : Vec<Symbol> = vec![];
	for x in old {
		let (&symbol, info_new) = x;
		if info_new.count != 1 {
			continue;
		}
		match new.get(&symbol) {
			Some(&TokenInfo{count:1, pos:_}) => out.push(symbol),
			Some(&TokenInfo{count:_, pos:_}) => continue,
			None => continue,
		}
	}
	out
}
