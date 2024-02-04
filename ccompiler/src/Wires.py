import types

'''
This file defines the set of core field objects (Wires) and
operations (FieldOps) available in the underlying encoding.
We build higher-level functions up by creating bus objects that
apply one or more FieldOps to groups of wires.
'''

class Wire:
	def __init__(self, idx): # type: (int) -> None
		assert(type(idx)==types.IntType)
		self.idx = idx

	def __repr__(self):
		return "%d" % self.idx
	
	def __hash__(self):
		return self.idx

	def __cmp__(self, other):
		return cmp(self.idx, other.idx)

	def rs_allocated_num(self):
		return "nums[%s]" % self.idx

	def rs_allocated_num_borrow(self):
		return "&nums[%s]" % self.idx

	def rs_value(self):
		return "nums[%s].get_value().unwrap()" % self.idx

	def rs_variable(self):
		return "nums[%s].get_variable()" % self.idx

class WireList:
	def __init__(self, wires): # type: (list[Wire]) -> None
		assert(type(wires)==types.ListType)
		self.wires = wires

	def __repr__(self):
		'''e.g. `2 <10 9>`'''
		l = " ".join(map(repr, self.wires))
		return "%d <%s>" % (len(self.wires), l)

	def __len__(self):
		return len(self.wires)

	def __getitem__(self, j): # type: (int) -> Wire
		return self.wires[j]
