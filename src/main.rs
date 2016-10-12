use std::collections::HashMap;
use std::hash::Hash;
use std::clone::Clone;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
enum Reference {
	Unknown,
	// Delete,
	Multiple,
	Confirmed(usize), // Referring a line number
}

type SymbolTable<T> = HashMap<T, Reference>;

fn main() {
	let start_sym = 'Ä';
	let end_sym = 'Ö';

	let old_file = vec![start_sym, 'A', 'B', 'C', 'D', 'E', 'G', end_sym];
	let new_file = vec![start_sym, 'D', 'E', 'F', 'G', 'A', 'C', end_sym];
	let symbol_table_old = create_symbol_table(&old_file);
	let symbol_table_new = create_symbol_table(&new_file);
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

fn update_neighbors<T>(v1 : &Vec<T>, v2 : &Vec<T>, map1 : &mut Vec<Reference>, map2 : &mut Vec<Reference>) where T : Eq + Display {
	let mut upd = |i, neighbor : fn (x : usize) -> usize | {
		let line_new = match map1[i] {
			Reference::Confirmed(l) => l,
			_ => return,
		};
		// v1[i] and v2[line_new] are confirmed to be the same
		let i_delta = neighbor(i);
		let sym1 = match map1[i_delta] {
			Reference::Unknown => &v1[i_delta],
			_ => return,
		};
		let new_delta = neighbor(line_new);
		match map2[new_delta] {
			Reference::Unknown => if v2[new_delta] != *sym1 { return; },
			_ => return,
		}
		map1[i_delta] = Reference::Confirmed(new_delta);
		map2[new_delta] = Reference::Confirmed(i_delta);
		println!("Matched token {} one line {} with line {}", sym1,  i_delta, new_delta);
	};
	let v1_len = v1.len();
	for i in 0 .. v1_len-2 {
		fn incr(x : usize) -> usize {x+1}
		upd(i, incr);
		fn decr(x : usize) -> usize {x-1}
		upd(v1_len-i-1, decr);
	}
}

fn update_unique_mappings<T>(unique_symbols : &Vec<T>, old : &SymbolTable<T>, new : &HashMap<T, Reference>, old_mapping : &mut Vec<Reference>, new_mapping : &mut Vec<Reference>) where T : Eq + Hash {
	for symbol in unique_symbols {
		let ref ref_new = new[symbol];
		let ref ref_old = old[symbol];
		if let (&Reference::Confirmed(line_new), &Reference::Confirmed(line_old)) = (ref_new, ref_old) {
			// println!("update_unique_mappings for character '{}' from line {} to line {}", symbol, line_old, line_new);
			old_mapping[line_old] = Reference::Confirmed(line_new);
			new_mapping[line_new] = Reference::Confirmed(line_old);
		} else {
			panic!("Only expecting confirmed lines for unique symbols");
		}
	}
}

fn create_symbol_table<T>(v : &Vec<T>) -> SymbolTable<T> where T : Eq + Hash + Clone {
	let mut s : SymbolTable<T> = HashMap::new();
	let mut line = 0;
	for symbol in v {
		let new = match s.get(&symbol) {
			Some(_) => Reference::Multiple,
			_ => Reference::Confirmed(line),
		};
		s.insert(symbol.clone(), new);
		line = line + 1;
	}
	s
}

// Find all symbols that are  unique in both symbol tables
fn get_unique_symbols<T>(s1 : &HashMap<T, Reference>, s2 : &HashMap<T, Reference>) -> Vec<T> where T : Eq + Hash + Clone {
	let mut out = vec![];
	for (symbol, reference) in s1 {
		if let Reference::Confirmed(_) = *reference {
			if let Some(&Reference::Confirmed(_)) = s2.get(symbol) {
				out.push(symbol.clone());
			}
		}
	}
	out
}
