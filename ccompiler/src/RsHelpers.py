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
			invert = ""
			if value < 0:
				invert = ".invert().unwrap()"
			lst.append(
"""
let %s = AllocatedNum::alloc(cs.namespace(|| "constant_%s"), || {
	Ok(%s.pow_vartime([%s])%s)
})?;
""" % (var_name, value, RS_SCALAR_ONE, abs(value), invert)
			)
			self.cache.add(value)
		return var_name

rs_constant_cache = RsConstantCache()	

def alloc_num(idx):
	return "num[%s]" % idx