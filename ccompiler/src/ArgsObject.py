class ArgsObject:
	'''
	For intellisense purposes.
	'''
	def __init__(self, cfile, app_name, print_exprs, il_file, json_file, arith_file, bit_width, bool_file, ignore_overflow, cpp_arg, loop_sanity_limit, progress, nova_circuit_rs_dir): 
		# type: (str, str, str, str, str, str, int|str, str, str, list[str], int|str, bool|str, str) -> None
		self.cfile = cfile
		'''a C file to compile'''
		self.app_name = app_name
		'''app name, conventionally in kebab case'''
		self.print_exprs = print_exprs
		'''print output expressions on stdout'''
		self.il_file = il_file
		'''intermediate circuit output file'''
		self.json_file = json_file
		'''json version of intermediate circuit output file'''
		self.arith_file = arith_file
		'''arithmetic circuit output file'''
		self.bit_width = bit_width
		'''bit width -- affects bitwise operator semantics and arithmetic circuit output'''
		self.bool_file = bool_file
		'''boolean circuit output file'''
		self.ignore_overflow = ignore_overflow
		'''ignore field-P overflows; never truncate'''
		self.cpp_arg = cpp_arg
		'''extra arguments to C preprocessor'''
		self.loop_sanity_limit = loop_sanity_limit
		'''limit on statically-measured loop unrolling'''
		self.progress = progress
		'''print progress messages during compilation'''
		self.nova_circuit_rs_dir = nova_circuit_rs_dir
		'''directory in which Rust output files are written'''


