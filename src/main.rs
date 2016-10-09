use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Reference {
	Unknown,
	// Delete,
	Multiple,
	Confirmed(usize),
}

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
	let mut symbol_table_old = HashMap::new();
	let mut symbol_table_new = HashMap::new();
	count(&mut symbol_table_old, &old_file);
	count(&mut symbol_table_new, &new_file);
    // println!("old symbol table {:?}", symbol_table_old);
    // println!("New symbol table {:?}", symbol_table_new);
	let unique_symbols = get_unique_symbols(&symbol_table_old, &symbol_table_new);
	println!("Unique symbols: {:?}", unique_symbols);
	
	let mut old_mapping : Vec<Reference> = vec![Reference::Unknown; old_file.len()];
	let mut new_mapping : Vec<Reference> = vec![Reference::Unknown; new_file.len()];
	
	update_unique_mappings(&unique_symbols, &symbol_table_old, &symbol_table_new, &mut old_mapping, &mut new_mapping);
	update_neighbors(&old_file, &new_file, &mut old_mapping, &mut new_mapping);
	
	println!("Mapping from old: {:?}", old_mapping);
	println!("Mapping from new: {:?}", new_mapping);
}

fn update_neighbors(v1 : &Vec<Symbol>, v2 : &Vec<Symbol>, map1 : &mut Vec<Reference>, map2 : &mut Vec<Reference>) {
	for i in 0 .. v1.len()-2 {
		let line_new = match map1[i] {
			Reference::Confirmed(l) => l,
			_ => continue,
		};
		// v1[i] and v2[line_new] are conrimed to be the same
		let sym1 = match map1[i+1] {
			Reference::Unknown => v1[i+1],
			_ => continue,
		};
		match map2[line_new+1] {
			Reference::Unknown => if v2[line_new+1] != sym1 { continue; },
			_ => continue,
		}
		map1[i+1] = Reference::Confirmed(line_new+1);
		map2[line_new+1] = Reference::Confirmed(i+1);
		println!("Matched token {} one line {} with line {}", sym1,  i+1, line_new+1);
	}
}

fn update_unique_mappings(unique_symbols : &Vec<Symbol>, old : &SymbolTable, new : &SymbolTable, old_mapping : &mut Vec<Reference>, new_mapping : &mut Vec<Reference>) {
	for symbol in unique_symbols {
		let TokenInfo{count:_, pos:ref ref_new} = new[symbol];
		let TokenInfo{count:_, pos:ref ref_old} = old[symbol];
		let line_new = match ref_new {
			&Reference::Confirmed(l) => l,
			_ => panic!(""),
		};
		let line_old = match ref_old {
			&Reference::Confirmed(l) => l,
			_ => panic!(""),
		};
		// println!("update_unique_mappings for character '{}' from line {} to line {}", symbol, line_old, line_new);
		old_mapping[line_old] = Reference::Confirmed(line_new);
		new_mapping[line_new] = Reference::Confirmed(line_old);
	}
}

fn count(s : &mut SymbolTable,  v : &Vec<Symbol>) {
	let mut line : usize = 0;
	for symbol in v {
		let new : TokenInfo = match s.get(&symbol) {
			Some(info) => TokenInfo{count: info.count+1, pos: Reference::Multiple},
			_ => TokenInfo {count:1, pos: Reference::Confirmed(line)},
		};
		s.insert(*symbol, new);
		line = line + 1;
	}
}

fn get_unique_symbols(old : &SymbolTable, new : &SymbolTable) -> Vec<Symbol> {
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
