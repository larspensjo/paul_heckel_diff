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

struct Diff<T> {
	symbol_table_old : SymbolTable<T>,
	symbol_table_new : SymbolTable<T>,
	old_mapping : Vec<Reference>,
	new_mapping : Vec<Reference>,
}

fn main() {
	let start_sym = 'Ä';
	let end_sym = 'Ö';

	let old_file = vec![start_sym, 'A', 'B', 'C', 'D', 'E', 'G', end_sym];
	let new_file = vec![start_sym, 'D', 'E', 'F', 'G', 'A', 'C', end_sym];
	let diff = Diff::new(old_file, new_file);
	println!("Mapping from old: {:?}", diff.old_mapping);
	println!("Mapping from new: {:?}", diff.new_mapping);
}

impl<T> Diff<T> where T : Eq + Hash + Clone + Display {
	fn new(old_file : Vec<T>, new_file : Vec<T>) -> Diff<T> {
		let symbol_table_old = Diff::create_symbol_table(&old_file);
		let symbol_table_new = Diff::create_symbol_table(&new_file);
		let unique_symbols = Diff::get_unique_symbols(&symbol_table_old, &symbol_table_new);
		// println!("old symbol table {:?}", symbol_table_old);
		// println!("New symbol table {:?}", symbol_table_new);
		let old_mapping : Vec<Reference> = vec![Reference::Unknown; old_file.len()];
		let new_mapping : Vec<Reference> = vec![Reference::Unknown; new_file.len()];
		let mut diff = Diff{symbol_table_old:symbol_table_old, symbol_table_new:symbol_table_new, old_mapping:old_mapping, new_mapping:new_mapping};
		diff.update_unique_mappings(&unique_symbols);
		diff.update_neighbors(&old_file, &new_file);
		diff
	}

	fn update_neighbors(&mut self, v1 : &Vec<T>, v2 : &Vec<T>) {
		let mut upd = |i, neighbor : fn (x : usize) -> usize | {
			let line_new = match self.old_mapping[i] {
				Reference::Confirmed(l) => l,
				_ => return,
			};
			// v1[i] and v2[line_new] are confirmed to be the same
			let i_delta = neighbor(i);
			let sym1 = match self.old_mapping[i_delta] {
				Reference::Unknown => &v1[i_delta],
				_ => return,
			};
			let new_delta = neighbor(line_new);
			match self.new_mapping[new_delta] {
				Reference::Unknown => if v2[new_delta] != *sym1 { return; },
				_ => return,
			}
			self.old_mapping[i_delta] = Reference::Confirmed(new_delta);
			self.new_mapping[new_delta] = Reference::Confirmed(i_delta);
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

	fn update_unique_mappings(&mut self, unique_symbols : &Vec<T>) {
		for symbol in unique_symbols {
			let ref ref_new = self.symbol_table_new[symbol];
			let ref ref_old = self.symbol_table_old[symbol];
			if let (&Reference::Confirmed(line_new), &Reference::Confirmed(line_old)) = (ref_new, ref_old) {
				// println!("update_unique_mappings for character '{}' from line {} to line {}", symbol, line_old, line_new);
				self.old_mapping[line_old] = Reference::Confirmed(line_new);
				self.new_mapping[line_new] = Reference::Confirmed(line_old);
			} else {
				panic!("Only expecting confirmed lines for unique symbols");
			}
		}
	}

	fn create_symbol_table(v : &Vec<T>) -> SymbolTable<T> {
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
	fn get_unique_symbols(s1 : &HashMap<T, Reference>, s2 : &HashMap<T, Reference>) -> Vec<T> {
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
}
