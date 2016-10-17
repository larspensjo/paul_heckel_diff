use std::collections::HashMap;
use std::hash::Hash;
use std::clone::Clone;
use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
enum Reference {
	Unknown,
	Delete,
	Insert,
	Multiple,
	Confirmed(usize), // Referring a line number
}

// Aggregate similar lines together into blocks
#[derive(Debug)]
enum BlockReference {
	Match {new_line_begin: usize, count: usize},
	Delete {old_line_begin: usize, count: usize},
	Dummy,
}

type SymbolTable<T> = HashMap<T, Reference>;

struct Diff<'a, T:'a> {
	symbol_table_old : SymbolTable<T>,
	symbol_table_new : SymbolTable<T>,
	old_mapping : Vec<Reference>,
	new_mapping : Vec<Reference>,
	old_file : &'a Vec<T>,
	new_file : &'a Vec<T>,
}

fn main() {
	let old_file : Vec<&str> = "BEGIN a mass of latin words falls upon the relevant facts like soft snow , covering up the details . END".split(" ").collect();
	let new_file : Vec<&str> = "BEGIN much writing is like snow , a mass of long words and phrases falls upon the relevant facts covering up the details . END".split(" ").collect();
	// let old_file = vec!["Ä", "A", "B", "C", "D", "E", "G", "Ö";
	// let new_file = vec!["Ä", "D", "E", "F", "G", "A", "C", "Ö"];
	let diff = Diff::new(&old_file, &new_file);
	println!("Mapping from old: {:?}", diff.old_mapping);
	println!("Mapping from new: {:?}", diff.new_mapping);
	diff.pretty_print();
	let aggr = diff.aggregate();
	println!("Debug: {:?}", aggr);
}

impl<'a, T> Diff<'a, T> where T : Eq + Hash + Clone + Display {
	fn new(old_file : &'a Vec<T>, new_file : &'a Vec<T>) -> Diff<'a, T> {
		// println!("old symbol table {:?}", symbol_table_old);
		// println!("New symbol table {:?}", symbol_table_new);
		let mut diff = Diff {
			symbol_table_old:	Diff::create_symbol_table(&old_file),
			symbol_table_new:	Diff::create_symbol_table(&new_file),
			old_mapping:		vec![Reference::Unknown; old_file.len()],
			new_mapping:		vec![Reference::Unknown; new_file.len()],
			old_file:			old_file,
			new_file:			new_file,
		};
		diff.update_unique_mappings();
		diff.update_neighbors();
		diff.replace_unknown();
		diff
	}
	
	fn aggregate(&self) -> Vec<BlockReference> {
		let mut out_old = vec![];
		let mut last_block = BlockReference::Dummy;
		for line in 1 .. &self.old_mapping.len() - 1 {
			let mapping = &self.old_mapping[line];
			match *mapping {
				Reference::Confirmed(new_line) => {
					if let BlockReference::Match{new_line_begin: prev_line, count: prev_count} = last_block {
						last_block = BlockReference::Match{new_line_begin: prev_line, count: prev_count+1};
					} else {
						out_old.push(last_block);
						last_block = BlockReference::Match{new_line_begin:new_line, count: 1};
					}
				},
				Reference::Delete => {
					if let BlockReference::Delete{old_line_begin: prev_line, count: prev_count} = last_block {
						last_block = BlockReference::Delete{old_line_begin: prev_line, count: prev_count+1};
					} else {
						out_old.push(last_block);
						last_block = BlockReference::Delete{old_line_begin:line, count: 1};
					}
				},
				Reference::Unknown => panic!(""),
				Reference::Insert => panic!(""),
				Reference::Multiple => panic!(""),
			}
			// println!("{:3}: {}", line, &self.old_file[line]);
		}
		out_old.push(last_block);
		out_old
	}
	
	fn pretty_print(&self) {
		fn pretty_print_file<T>(file : &Vec<T>, mapping : &Vec<Reference>) where T : Display {
			for line in 1 .. file.len() - 1 {
				let operation = format!("{:?}", mapping[line]);
				println!("{:3} {:15}: {}", line, operation, &file[line]);
			}
		}
		pretty_print_file(&self.old_file, &self.old_mapping);
		println!("");
		pretty_print_file(&self.new_file, &self.new_mapping);
	}

	fn replace_unknown(&mut self) {
		// Unknown references in the old mapping are deleted
		for r in self.old_mapping.iter_mut() {
			if let Reference::Unknown = *r {
				*r = Reference::Delete;
			}
		}
		// Unknown references in the old mapping are inserted
		for r in self.new_mapping.iter_mut() {
			if let Reference::Unknown = *r {
				*r = Reference::Insert;
			}
		}
	}

	fn update_neighbors(&mut self) {
		let old_file_len = self.old_file.len();
		let mut upd = |i, neighbor : fn (x : usize) -> usize | {
			let new_neighbor = match self.old_mapping[i] {
				Reference::Confirmed(l) => neighbor(l),
				_ => return,
			};
			// old_file[i] and new_file are confirmed to have corresponding lines.
			// Now check if the neighbor in both files are the same.
			let old_neighbor = neighbor(i);
			let sym1 = match self.old_mapping[old_neighbor] {
				Reference::Unknown => &self.old_file[old_neighbor],
				_ => return,
			};
			match self.new_mapping[new_neighbor] {
				Reference::Unknown => if self.new_file[new_neighbor] != *sym1 { return; },
				_ => return,
			}
			self.old_mapping[old_neighbor] = Reference::Confirmed(new_neighbor);
			self.new_mapping[new_neighbor] = Reference::Confirmed(old_neighbor);
			println!("Matched token '{}' one line {} with line {}", sym1,  old_neighbor, new_neighbor);
		};
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
