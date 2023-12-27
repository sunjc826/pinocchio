from Wires import *
from FieldOps import *

class FieldZeroP(FieldOp):
	"""
	Adds support for the zero-equals gate described in the paper
	
	Zero-Equality Gate. Another useful type of comparison
	functionality is checking whether a value is equal to zero,
	e.g., Y = (X! = 0)?1 : 0. We use a prior observation [28] that
	this is equivalent to satisfying the following two constraints:
	X * M - Y = 0 and (1 - Y) * X = 0 for some value M. We
	construct the following QAP, which takes as input wire X (an
	input from the client) and a wire M, an input from the worker.
	The output wire will be Y.
	... (See more details from the Pinocchio paper)
	"""

	def __init__(self, comment, in_wire, out_wire, m_wire): # type: (str, Wire, Wire, Wire) -> None
		FieldOp.__init__(self, comment)
		self.in_wire = in_wire
		'''X'''
		self.out_wire = out_wire
		'''Y'''
		self.m_wire = m_wire
		'''M'''

	def field_command(self):
		'''e.g. `zerop in 1 <8> out 2 <10 9>`'''
		return "zerop in %s out %s" % (WireList([self.in_wire]), WireList([self.m_wire, self.out_wire]))
	def input_wires(self): return WireList([self.in_wire])
	def output_wires(self): return WireList([self.out_wire])

class FieldConstMul(FieldOp):
	'''Multiply 1 in-wire with a constant `value`'''
	def __init__(self, comment, value, in_wire, out_wire): # type: (str, int, Wire, Wire) -> None
		FieldOp.__init__(self, comment)
		self.assert_int(value)
		assert(isinstance(in_wire, Wire))
		assert(isinstance(out_wire, Wire))
		self.value = value
		self.in_wire = in_wire
		self.out_wire = out_wire

	def field_command(self):
		'''
		e.g. `const-mul-0 in 1 <2> out 1 <3>`

		e.g. `const-mul-neg-1 in 1 <4> out 1 <7>`
		'''
		if (self.value >= 0):
			constant = "%x" % self.value
		else:
			constant = "neg-%x" % (-self.value)
		return "const-mul-%s in %s out %s" % (
			constant, WireList([self.in_wire]), WireList([self.out_wire]))

	def input_wires(self): return WireList([self.in_wire])
	def output_wires(self): return WireList([self.out_wire])

class FieldBinaryOp(FieldOp):
	def __init__(self, comment, verb, in_list, out_list): # type: (str, str, WireList, WireList) -> None
		FieldOp.__init__(self, comment)
		assert(isinstance(in_list, WireList))
		assert(isinstance(out_list, WireList))
		self.verb = verb
		self.in_list = in_list
		self.out_list = out_list

	def field_command(self):
		'''e.g. `add in 2 <0 5> out 1 <6>`'''
		return "%s in %s out %s" % (self.verb, self.in_list, self.out_list)

	def input_wires(self): return self.in_list
	def output_wires(self): return self.out_list

class FieldAdd(FieldBinaryOp):
	def __init__(self, comment, in_list, out_list): # type: (str, WireList, WireList) -> None
		FieldBinaryOp.__init__(self, comment, "add", in_list, out_list)
		assert(len(in_list)>1)
		assert(len(out_list)==1)

class FieldMul(FieldBinaryOp):
	def __init__(self, comment, in_list, out_list): # type: (str, WireList, WireList) -> None
		FieldBinaryOp.__init__(self, comment, "mul", in_list, out_list)
		assert(len(in_list)>1)
		assert(len(out_list)==1)

	def report(self, r):
		r.add("mul", 1)
		r.add("raw_mul", 1)

class FieldSplit(FieldBinaryOp):
	'''Splitting one in-wire into multiple out-wires'''
	def __init__(self, comment, in_list, out_list): # type: (str, WireList, WireList) -> None
		FieldBinaryOp.__init__(self, comment, "split", in_list, out_list)
		assert(len(in_list)==1)

	def report(self, r):
		r.add("split", 1)
		r.add("raw_mul", len(self.out_list)+1)

	def input_wires(self): return self.in_list
	def output_wires(self): return self.out_list

