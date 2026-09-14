use std::fs::File;
use std::io::Write;
use std::collections::HashMap;
use crate::parser::Node;
use crate::parser::NodeValue;
use crate::error::ErrorSystem;

#[derive(PartialEq, Eq, Clone, Debug)]
enum InstParam {
	N4,
	N8,
	N16,
	Reg,
	Pair,
	DerefPair,
	DerefN16
}

#[derive(Clone, Debug)]
struct InstDef {
	pub name:   String,
	pub opc:    u8,
	pub params: Vec<InstParam>
}

impl InstDef {
	pub fn new(name: &str, opc: u8, params: Vec<InstParam>) -> InstDef {
		return InstDef {
			name:   name.to_string(),
			opc:    opc,
			params: params
		};
	}

	// excludes opcode
	pub fn calc_size(&self) -> u16 {
		return match self.params[..] {
			[InstParam::Pair]                            |
			[InstParam::Reg]                             |
			[InstParam::Reg,       InstParam::Reg]       |
			[InstParam::Pair,      InstParam::Pair]      |
			[InstParam::Pair,      InstParam::Reg]       |
			[InstParam::Reg,       InstParam::DerefPair] |
			[InstParam::DerefPair, InstParam::Reg]       |
			[InstParam::DerefPair, InstParam::Pair]      |
			[InstParam::Pair,      InstParam::DerefPair] => 1,

			[InstParam::N16]                      |
			[InstParam::Reg,       InstParam::N8] |
			[InstParam::DerefPair, InstParam::N8] => 2,

			[InstParam::Pair,      InstParam::N16] |
			[InstParam::DerefPair, InstParam::N16] => 3,

			_ => {
				panic!();
			}
		};
	}
}

#[derive(PartialEq, Eq)]
enum AsmBank {
	None,
	Program,
	Ram
}

pub struct Assembler<'a> {
	insts:     Vec<InstDef>,
	bank:      AsmBank,
	bankAddr:  u16,
	symbols:   HashMap<String, u16>,
	symbolOut: Option<File>,
	outFile:   File,
	size:      u16,
	progAddr:  u16,

	errorSys: &'a mut ErrorSystem
}

impl Assembler<'_> {
	pub fn new<'a>(errorSys: &'a mut ErrorSystem, outPath: &str) -> Option<Assembler<'a>> {
		let insts = vec![
			InstDef::new("halt", 0x00, Vec::new()),
			InstDef::new("nop",  0x01, Vec::new()),

			InstDef::new("mov", 0x10, vec![InstParam::Reg,       InstParam::Reg]),
			InstDef::new("mov", 0x11, vec![InstParam::Pair,      InstParam::Pair]),
			InstDef::new("mov", 0x12, vec![InstParam::Reg,       InstParam::N8]),
			InstDef::new("mov", 0x13, vec![InstParam::Pair,      InstParam::N16]),
			InstDef::new("mov", 0x14, vec![InstParam::Reg,       InstParam::DerefPair]),
			InstDef::new("mov", 0x15, vec![InstParam::Pair,      InstParam::DerefPair]),
			InstDef::new("mov", 0x17, vec![InstParam::Reg,       InstParam::DerefN16]),
			InstDef::new("mov", 0x18, vec![InstParam::Pair,      InstParam::DerefN16]),
			InstDef::new("mov", 0x19, vec![InstParam::DerefPair, InstParam::Reg]),
			InstDef::new("mov", 0x1A, vec![InstParam::DerefPair, InstParam::Pair]),
			InstDef::new("mov", 0x1B, vec![InstParam::DerefPair, InstParam::N8]),
			InstDef::new("mov", 0x1C, vec![InstParam::DerefPair, InstParam::N16]),

			InstDef::new("jmp", 0x20, vec![InstParam::N16]),
			InstDef::new("jmp", 0x21, vec![InstParam::DerefN16]),
			InstDef::new("jmp", 0x22, vec![InstParam::Pair]),
			InstDef::new("jmp", 0x23, vec![InstParam::DerefPair]),

			InstDef::new("call", 0x24, vec![InstParam::N16]),
			InstDef::new("call", 0x25, vec![InstParam::DerefN16]),
			InstDef::new("call", 0x26, vec![InstParam::Pair]),
			InstDef::new("call", 0x27, vec![InstParam::DerefPair]),

			InstDef::new("jz",  0x28, vec![InstParam::N16]),
			InstDef::new("jnz", 0x29, vec![InstParam::N16]),
			InstDef::new("js",  0x2A, vec![InstParam::N16]),
			InstDef::new("jns", 0x2B, vec![InstParam::N16]),
			InstDef::new("jc",  0x2C, vec![InstParam::N16]),
			InstDef::new("jnc", 0x2D, vec![InstParam::N16]),

			InstDef::new("ret", 0x2E, Vec::new()),

			InstDef::new("cmp", 0x40, vec![InstParam::Reg,  InstParam::Reg]),
			InstDef::new("cmp", 0x41, vec![InstParam::Reg,  InstParam::N8]),
			InstDef::new("cmp", 0x42, vec![InstParam::Pair, InstParam::Pair]),

			InstDef::new("add", 0x43, vec![InstParam::Reg,  InstParam::Reg]),
			InstDef::new("sub", 0x44, vec![InstParam::Reg,  InstParam::Reg]),
			InstDef::new("mul", 0x45, vec![InstParam::Reg,  InstParam::Reg]),
			InstDef::new("div", 0x46, vec![InstParam::Reg,  InstParam::Reg]),
			InstDef::new("add", 0x47, vec![InstParam::Pair, InstParam::Reg]),
			InstDef::new("sub", 0x48, vec![InstParam::Pair, InstParam::Reg]),
			InstDef::new("and", 0x49, vec![InstParam::Reg,  InstParam::Reg]),
			InstDef::new("or",  0x4A, vec![InstParam::Reg,  InstParam::Reg]),
			InstDef::new("xor", 0x4B, vec![InstParam::Reg,  InstParam::Reg]),

			InstDef::new("not", 0x4C, vec![InstParam::Reg]),

			InstDef::new("icmp", 0x4D, vec![InstParam::Reg,  InstParam::Reg]),
			InstDef::new("icmp", 0x4E, vec![InstParam::Reg,  InstParam::N8]),
			InstDef::new("icmp", 0x4F, vec![InstParam::Pair, InstParam::Pair]),

			InstDef::new("setz", 0x50, Vec::new()),
			InstDef::new("clz",  0x51, Vec::new()),
			InstDef::new("sets", 0x52, Vec::new()),
			InstDef::new("cls",  0x53, Vec::new()),
			InstDef::new("setc", 0x54, Vec::new()),
			InstDef::new("clc",  0x55, Vec::new()),

			InstDef::new("inc", 0x60, vec![InstParam::Reg]),
			InstDef::new("inc", 0x61, vec![InstParam::Pair]),
			InstDef::new("dec", 0x62, vec![InstParam::Reg]),
			InstDef::new("dec", 0x63, vec![InstParam::Pair]),

			InstDef::new("shl", 0x64, vec![InstParam::Reg,  InstParam::N4]),
			InstDef::new("shl", 0x65, vec![InstParam::Pair, InstParam::N4]),
			InstDef::new("shr", 0x66, vec![InstParam::Reg,  InstParam::N4]),
			InstDef::new("shr", 0x67, vec![InstParam::Pair, InstParam::N4]),
			InstDef::new("shl", 0x68, vec![InstParam::Reg,  InstParam::Reg]),
			InstDef::new("shl", 0x69, vec![InstParam::Pair, InstParam::Reg]),
			InstDef::new("shr", 0x6A, vec![InstParam::Reg,  InstParam::Reg]),
			InstDef::new("shr", 0x6B, vec![InstParam::Pair, InstParam::Reg])
		];

		let file = File::create(outPath);

		if file.is_err() {
			eprintln!("Failed to open output file {}", outPath);
			return None;
		}

		let symbolPath = format!("{}.sym", outPath);
		let symbolOut  = File::create(&symbolPath);

		if file.is_err() {
			eprintln!("Failed to open symbol file {}", symbolPath);
			return None;
		}

		return Some(Assembler {
			insts:     insts,
			bank:      AsmBank::None,
			bankAddr:  0,
			symbols:   HashMap::new(),
			symbolOut: symbolOut.ok(),
			outFile:   file.unwrap(),
			size:      0,
			progAddr:  0,
			errorSys:  errorSys
		});
	}

	fn add_error(&mut self, node: &Node, msg: &str) {
		self.errorSys.add(node.error().to_owned(), msg);
	}

	fn add_symbol(&mut self, name: &str, value: u16) -> Option<()> {
		if self.symbolOut.is_some() {
			let line = format!("{:#06x} {}\n", value, name);

			self.symbolOut.as_ref().unwrap().write_all(line.as_bytes()).ok()?;
		}

		self.symbols.insert(name.to_string(), value);
		return Some(());
	}

	fn node_reg_type(node: &Node) -> InstParam {
		return match node.unwrap_register().as_str() {
			"a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" => InstParam::Reg,
			"ab" | "cd" | "ef" | "sp"                     => InstParam::Pair,
			_ => panic!()
		}
	}

	fn match_instruction(&mut self, name: &str, params: &Vec<Node>) -> Option<&InstDef> {
		'outer: for def in self.insts.iter() {
			if (def.name != name) || (def.params.len() != params.len()) {
				continue;
			}

			for (i, param) in params.iter().enumerate() {
				match param.value() {
					NodeValue::Register(_) => {
						if (
							(def.params[i] == InstParam::Reg) &&
							(Self::node_reg_type(&param) != InstParam::Reg)
						) {
							continue 'outer;
						}
						if (
							(def.params[i] == InstParam::Pair) &&
							(Self::node_reg_type(&param) != InstParam::Pair)
						) {
							continue 'outer;
						}
					},
					NodeValue::Int(value) => {
						match def.params[i] {
							InstParam::N4 => {
								if *value > 15 {
									continue 'outer;
								}
							},
							InstParam::N8 => {
								if *value > 256 {
									continue 'outer;
								}
							},
							InstParam::N16 => {
								if *value > 65535 {
									continue 'outer;
								}
							},
							_ => {
								continue 'outer;
							}
						}
					},
					NodeValue::Identifier(name) => {
						let v = match self.symbols.get(name) {
							Some(value) => value,
							None => {
								self.errorSys.add(params[i].error().to_owned(), &format!(
									"Unknown identifier '{}'", name
								));
								return None;
							}
						};

						match def.params[i] {
							InstParam::N4 => {
								if *v > 15 {
									continue 'outer;
								}
							},
							InstParam::N8 => {
								if *v > 255 {
									continue 'outer;
								}
							},
							InstParam::N16 => {},
							_ => {
								continue 'outer;
							}
						}
					}
					_ => {
						continue 'outer;
					}
				}
			}

			return Some(def);
		}

		return None;
	}

	fn run_directive(&mut self, node: &Node, name: &str, params: &Vec<Node>) {
		match name {
			"bank" => {
				if params.len() != 1 {
					self.add_error(node, "bank directive requires 1 parameter");
					return;
				}

				match params[0].value() {
					NodeValue::Identifier(name) => {
						self.size = 0;

						match name.as_str() {
							"program" => {
								self.bank     = AsmBank::Program;
								self.bankAddr = self.progAddr;
							},
							"ram" => {
								self.bank     = AsmBank::Ram;
								self.bankAddr = 0x8000;
							},
							_ => {
								self.add_error(node, &format!("Invalid bank '{}'", name));
							}
						}
					},
					_ => {
						self.add_error(node, &format!("Unexpected '{}'", params[0].type_name()));
					}
				}
			},
			"res" => {
				if params.len() != 1 {
					self.add_error(node, "res directive requires 1 parameter");
					return;
				}

				match params[0].value() {
					NodeValue::Int(value) => {
						self.size += *value as u16;
					},
					_ => {
						self.add_error(node, &format!("Unexpected '{}'", params[0].type_name()));
					}
				}
			},
			_ => {
				self.add_error(node, &format!("Unknown directive: '{}'", name));
			}
		}
	}

	fn assemble_register(reg: &str) -> u8 {
		return match reg.to_lowercase().as_str() {
			"a" => 0,
			"b" => 1,
			"c" => 2,
			"d" => 3,
			"e" => 4,
			"f" => 5,
			"g" => 6,
			"h" => 7,
			_   => panic!("{}", reg)
		};
	}

	fn assemble_pair(reg: &str) -> u8 {
		return match reg.to_lowercase().as_str() {
			"ab" => 0,
			"cd" => 1,
			"ef" => 2,
			"sp" => 3,
			_    => panic!("{}", reg)
		};
	}

	fn assemble_n16(&self, param: &Node) -> Option<u16> {
		return match param.value() {
			NodeValue::Int(v)        => Some(*v as u16),
			NodeValue::Identifier(v) => Some(*(self.symbols.get(v)?)),
			_ => panic!()
		}
	}

	fn assemble_inst(&mut self, name: &str, params: &Vec<Node>) -> Option<()> {
		let inst = self.match_instruction(name, &params)?.clone();

		assert!(inst.params.len() == params.len());

		self.outFile.write_all(&[inst.opc]).ok()?;

		if inst.params.len() == 0 {
			return Some(());
		}

		let mut param: Option<u8> = None;

		match inst.params[0] {
			InstParam::Reg => {
				param = Some(Self::assemble_register(params[0].unwrap_register()) << 5);
			},
			InstParam::Pair | InstParam::DerefPair => {
				param = Some(Self::assemble_pair(params[0].unwrap_register()) << 6);
			},
			InstParam::N16 | InstParam::DerefN16 => {
				self.outFile.write_all(&(params[0].unwrap_int() as u16).to_le_bytes()).ok()?;

				assert!(inst.params.len() == 1);
			},
			_ => panic!()
		}

		if inst.params.len() >= 2 {
			match inst.params[1] {
				InstParam::Reg => {
					let shift = match inst.params[0] {
						InstParam::Reg  => 2,
						InstParam::Pair => 3,
						_               => panic!()
					};

					param = Some(param.unwrap() | (
						Self::assemble_register(params[1].unwrap_register()) << shift
					));

					self.outFile.write_all(&[param.unwrap()]).ok()?;
				},
				InstParam::Pair | InstParam::DerefPair => {
					let shift = match inst.params[0] {
						InstParam::Reg  => 3,
						InstParam::Pair => 4,
						_               => panic!()
					};

					param = Some(param.unwrap() | (Self::assemble_pair(
						params[1].unwrap_register()) << shift
					));

					self.outFile.write_all(&[param.unwrap()]).ok()?;
				},
				InstParam::N8 => {
					self.outFile.write_all(&[param.unwrap(), params[1].unwrap_int() as u8]).ok()?;
				},
				InstParam::N16 | InstParam::DerefN16 => {
					let n16 = self.assemble_n16(&params[1]);

					if n16.is_none() {
						self.add_error(&params[1], &format!(
							"Unknown identifier '{}'", params[1].unwrap_identifier()
						));
					}

					self.outFile.write_all(&[param.unwrap()]).ok()?;
					self.outFile.write_all(&n16.unwrap().to_le_bytes()).ok()?;
				},
				_ => panic!()
			}
		}

		self.outFile.flush().ok()?;

		return Some(());
	}

	pub fn assemble(&mut self, nodes: &Vec<Node>) -> bool {
		let mut success = true;

		// symbol pass
		for node in nodes.iter() {
			match node.value() {
				NodeValue::Instruction {name, params} => {
					if self.bank != AsmBank::Program {
						self.add_error(&node, "Instructions must be in program bank");
						continue;
					}

					let res = self.match_instruction(name, params);

					match res {
						Some(inst) => {
							self.size += 1 + inst.calc_size();
						},
						None => {
							self.add_error(node, "Invalid instruction");
							continue;
						}
					}
				},
				NodeValue::Label(labelName) => {
					self.add_symbol(labelName, self.size + self.bankAddr);
				},
				NodeValue::Directive {name, params} => {
					self.run_directive(node, &name, &params);
				},
				_ => {
					self.add_error(&node, &format!("Unexpected {}", node.type_name()));
				}
			}
		}

		// assembly pass
		for node in nodes.iter() {
			match node.value() {
				NodeValue::Instruction {name, params} => {
					if self.assemble_inst(name, params).is_none() {
						success = false;
					}
				},
				NodeValue::Label(_)                   => {},
				NodeValue::Directive {name, params}   => self.run_directive(node, &name, &params),

				_ => panic!()
			}
		}

		return success;
	}
}
