import types
from Wires import *

class FieldOp:
	def __init__(self, comment):
		self._comment = comment

	def field_command(self): # type: () -> str
		'''e.g. `add in 2 <0 5> out 1 <6>`'''
		raise Exception("abstract")
	def input_wires(self): # type: () -> WireList
		'''e.g. `in 2 <0 5>`'''
		raise Exception("abstract")
	def output_wires(self): # type: () -> WireList
		'''e.g. `out 1 <6>`'''
		raise Exception("abstract")

	def report(self, r): pass

	def __repr__(self):
		'''e.g. `add in 2 <0 5> out 1 <6>                 # ArithBusReq.AddReq(DFG.Input,DFG.Constant)`'''
		return "%-40s # %s" % (self.field_command(), self._comment)

	def assert_int(self, value):
		assert(type(value)==types.IntType or type(value)==types.LongType)
		
class FieldInputBase(FieldOp):
	def __init__(self, command, comment, out_wire): # type: (str, str, Wire) -> None
		FieldOp.__init__(self, comment)
		self._command = command
		assert(isinstance(out_wire, Wire))
		self.out_wire = out_wire

	def field_command(self):
		'''e.g. `input 0`'''
		return "%s %s" % (self._command, self.out_wire)

	def input_wires(self): return WireList([])
	def output_wires(self): return WireList([self.out_wire])

class FieldInput(FieldInputBase):
	def __init__(self, comment, out_wire): # type: (str, Wire) -> None
		FieldInputBase.__init__(self, "input", comment, out_wire)

class FieldNIZKInput(FieldInputBase):
	def __init__(self, comment, out_wire): # type: (str, Wire) -> None
		FieldInputBase.__init__(self, "nizkinput", comment, out_wire)

class FieldOutput(FieldOp):
	def __init__(self, comment, in_wire): # type: (str, Wire) -> None
		FieldOp.__init__(self, comment)
		assert(isinstance(in_wire, Wire))
		self.in_wire = in_wire

	def field_command(self):
		'''e.g. `output 13`'''
		return "output %s" % (self.in_wire)

	def input_wires(self): return WireList([self.in_wire])
	def output_wires(self): return WireList([])
		# This line marks an output, it doesn't specify an output.
		# Otherwise, we'd detect a duplicate output.
