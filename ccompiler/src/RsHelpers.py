RS_SCALAR_ZERO = "Self::ZERO"
RS_SCALAR_ONE = "Self::ONE"
RS_VAR_ONE = "CS::one()"

class RsConstantCache:
	def __init__(self):
		self.cache = set() # type: set[int]
	
	def get_constant(self, value, lst): # type: (int, list[str]) -> str
		if value < 0:
			var_name = "constant_neg_%s" % -value
		else:
			var_name = "constant_%s" % value
		if value not in self.cache:
			additive_invert = ""
			if value < 0:
				additive_invert = "-"
			lst.append(
"""
let %s = AllocatedNum::alloc(cs.namespace(|| "constant_%s"), || {
	Ok(%sG::Scalar::from(%s))
})?;
""" % (var_name, value, additive_invert, abs(value))
			)
			self.cache.add(value)
		return var_name

rs_constant_cache = RsConstantCache()	

def push_num(s): # type: (str) -> str
	return "nums.push(%s);" % s

def push_multiple_nums(s): # type: (str) -> str
	return \
"""
let multiple_nums = %s;
nums.extend(multiple_nums);
""" % s
