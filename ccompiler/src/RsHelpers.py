
from BitWidth import BitWidth
from TypeHintHelpers import always_false
if always_false:
	from typing import Literal

RS_SCALAR_ZERO = "Self::ZERO"
RS_SCALAR_ONE = "Self::ONE"
RS_VAR_ONE = "CS::one()"
INVERT_TYPE_TWOS_COMPLEMENT = 0
INVERT_TYPE_SCALAR_FIELD = 1 
class RsConstantCache:
	
	def __init__(self, bitwidth): # type: (BitWidth) -> None
		self.cache = set() # type: set[int]
		self.bitwidth = bitwidth
	
	def get_constant(self, value, lst, invert_type = INVERT_TYPE_SCALAR_FIELD): # type: (int, list[str], int) -> str
		'''
		Get a constant field element representing `value`.
		If `invert_type` is `INVERT_TYPE_TWOS_COMPLEMENT`, then a negative `value` will be converted to
		its corresponding 2's complement unsigned value.
		If `invert_type` is `INVERT_TYPE_SCALAR_FIELD`, then a negative `value` is treated an additive inverse
		in the field.

		Examples: 
		- `ConstantMultiplyBus` already does INVERT_TYPE_TWOS_COMPLEMENT in its constructor
		- `ArithmeticConditionalBus`: When inverting a boolean value, such as when computing if-else, `INVERT_TYPE_SCALAR_FIELD` and
		  `INVERT_TYPE_TWOS_COMPLEMENT` are equivalent up to the first 32 bits (assuming 32-bit 2's complement), however
		  we prefer `INVERT_TYPE_SCALAR_FIELD` so that we don't overflow as easily.
		'''
		if value < 0:
			if invert_type == INVERT_TYPE_TWOS_COMPLEMENT:
				value = value & self.bitwidth.get_neg1()
				var_name = "constant_%s" % value
			else:
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

def push_num(s): # type: (str) -> str
	return "nums.push(%s);" % s

def push_multiple_nums(s): # type: (str) -> str
	return \
"""
let multiple_nums = %s;
nums.extend(multiple_nums);
""" % s
