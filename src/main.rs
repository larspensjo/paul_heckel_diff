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
		// println!("old symbol table {:?}", symbol_table_old);
		// println!("New symbol table {:?}", symbol_table_new);
		let mut diff = Diff {
			symbol_table_old:	symbol_table_old,
			symbol_table_new:	symbol_table_new,
			old_mapping:		vec![Reference::Unknown; old_file.len()],
			new_mapping:		vec![Reference::Unknown; new_file.len()]
		};
		diff.update_unique_mappings();
		diff.update_neighbors(&old_file, &new_file);
		diff
	}

	fn update_neighbors(&mut self, old_file : &Vec<T>, new_file : &Vec<T>) {
		let mut upd = |i, neighbor : fn (x : usize) -> usize | {
			let line_new = match self.old_mapping[i] {
				Reference::Confirmed(l) => l,
				_ => return,
			};
			// old_file[i] and new_file[line_new] are confirmed to be the same
			let i_delta = neighbor(i);
			let sym1 = match self.old_mapping[i_delta] {
				Reference::Unknown => &old_file[i_delta],
				_ => return,
			};
			let new_delta = neighbor(line_new);
			match self.new_mapping[new_delta] {
				Reference::Unknown => if new_file[new_delta] != *sym1 { return; },
				_ => return,
			}
			self.old_mapping[i_delta] = Reference::Confirmed(new_delta);
			self.new_mapping[new_delta] = Reference::Confirmed(i_delta);
			println!("Matched token {} one line {} with line {}", sym1,  i_delta, new_delta);
		};
		let old_file_len = old_file.len();
		for i in 0 .. old_file_len-2 {
			fn incr(x : usize) -> usize {x+1}
			upd(i, incr);
			fn decr(x : usize) -> usize {x-1}
			upd(old_file_len-i-1, decr);
		}
	}

	fn update_unique_mappings(&mut self) {
		for symbol in self.get_unique_symbols() {
			if let (Some(&Reference::Confirmed(line_new)), Some(&Reference::Confirmed(line_old))) = (self.symbol_table_new.get(&symbol), self.symbol_table_old.get(&symbol)) {
				// println!("update_unique_mappings for character '{}' from line {} to line {}", symbol, line_old, line_new);
				self.old_mapping[line_old] = Reference::Confirmed(line_new);
				self.new_mapping[line_new] = Reference::Confirmed(line_old);
			} else {
				panic!("Only expecting confirmed lines for unique symbols");
			}
		}
	}

	fn create_symbol_table(file : &Vec<T>) -> SymbolTable<T> {
		let mut s : SymbolTable<T> = HashMap::new();
		for (line, symbol) in file.iter().enumerate() {
			let new = match s.get(&symbol) {
				Some(_) => Reference::Multiple,
				_ => Reference::Confirmed(line),
			};
			s.insert(symbol.clone(), new);
		}
		s
	}

	// Find all symbols that are  unique in both symbol tables
	fn get_unique_symbols(&self) -> Vec<T> {
		let mut out = vec![];
		for (symbol, reference) in &self.symbol_table_new {
			if let &Reference::Confirmed(_) = reference {
				if let Some(&Reference::Confirmed(_)) = self.symbol_table_old.get(symbol) {
					out.push(symbol.clone());
				}
			}
		}
		out
	}
}
