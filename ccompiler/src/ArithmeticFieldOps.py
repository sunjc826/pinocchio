from Wires import *
from FieldOps import *
from RsHelpers import RS_SCALAR_ZERO, RS_SCALAR_ONE, RS_VAR_ONE, rs_constant_cache, push_num, push_multiple_nums

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

	def rs_synthesize(self, lst):
		'''
		Reference: Nova/src/gadgets/utils.rs 

		Function: nova_snark::gadgets::utils::alloc_num_equals

		- alloc_num_equals's `a` maps to `X`
		- alloc_num_equals's `b` maps to `0`
		- alloc_num_equals's `r_value` maps to `Y inverted`
		- alloc_num_equals's `t` maps to `M`, which is actually `X inverted` when `X != 0`
		'''
		x = self.in_wire
		lst.append(
"""
let (y, m) = alloc_num_equals(cs, %s)?;
nums.push(y);
nums.push(m);
""" % x.rs_allocated_num_borrow()
		)


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

	def rs_synthesize(self, lst):
		rs_constant = rs_constant_cache.get_constant(self.value, lst)
		lst.append(
			push_num(
"""
%s.mul(cs.namespace(|| ""), &%s)?
""" % (self.in_wire.rs_allocated_num(), rs_constant)
			)
		)

class FieldConstMulWithOne(FieldConstMul):
	def __init__(self, comment, value, in_wire, out_wire): # type: (str, int, Wire, Wire) -> None
		FieldConstMul.__init__(self, comment, value, in_wire, out_wire)

	def rs_synthesize(self, lst):
		rs_constant = rs_constant_cache.get_constant(self.value, lst)
		lst.append(
			push_num(
"""
%s.clone()
""" % rs_constant
			)	
		)

	
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

	def rs_synthesize(self, lst):
		'''Nova'''
		assert(len(self.in_list) == 2)
		lst.append(
			push_num(
"""
%s.add(cs.namespace(|| ""), %s)?
""" % (self.in_list[0].rs_allocated_num(), self.in_list[1].rs_allocated_num_borrow())
			)
		)
class FieldMul(FieldBinaryOp):
	def __init__(self, comment, in_list, out_list): # type: (str, WireList, WireList) -> None
		FieldBinaryOp.__init__(self, comment, "mul", in_list, out_list)
		assert(len(in_list)>1)
		assert(len(out_list)==1)

	def report(self, r):
		r.add("mul", 1)
		r.add("raw_mul", 1)

	def rs_synthesize(self, lst): # type: (list[str]) -> None
		'''Nova'''
		assert(len(self.in_list) == 2)
		lst.append(
			push_num(
"""
%s.mul(cs.namespace(|| ""), %s)?
""" % (self.in_list[0].rs_allocated_num(), self.in_list[1].rs_allocated_num_borrow())
			)
		)

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

	def rs_synthesize(self, lst): # type: (list[str]) -> None
		'''Nova'''
		lst.append(
			push_multiple_nums(
"""
num_to_le_bit_nums(cs, %s, %s)?
""" % (len(self.out_list), self.in_list[0].rs_allocated_num_borrow())
			)
		)


