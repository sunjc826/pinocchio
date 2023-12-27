import CircuitParams

class BitWidth:
	'''
	Let N := self.width. We will use N in documenting BitWidth methods.
	BitWidth doesn't store any computational state (other than width and overflow configurations). 
	It provides utility functions for numerical operations that may involving clamping results.
	'''
	def __init__(self, width, ignore_overflow): # type: (int, bool) -> None
		self.width = width
		if (ignore_overflow):
			self.overflow_limit = None
		else:
			self.overflow_limit = CircuitParams.ACTIVE_BIT_CONSTRAINT

	def ignoring_overflow(self):
		return self.overflow_limit==None

	def get_width(self):
		return self.width

	def get_sign_bit(self):
		'''returns the position of the sign bit'''
		return self.width - 1

	def get_neg1(self):
		'''
		Think 2's complement arithmetic.
		(1 << N) - 1 is equal to -1 modulo (1 << N)
		Effectively, this acts as a bitmask of N bits.
		'''
		return (1<<self.width)-1

	def leftshift(self, a, b):
		'''
		Computes a << b and clamps the result by the bitmask (1 << N)-1
		'''
		return (a<<b) & self.get_neg1()

	def rightshift(self, a, b):
		return ((a & self.get_neg1()) >> b)
		# NB TODO rightshift is *always* unsigned in our world; we're not
		# really honoring the sign bit.

	def truncate(self, bits): # type: (int) -> int
		'''Computes `min(self.width, bits)` if overflow_limit is enabled, otherwise returns `bits`.'''
		if (self.overflow_limit != None
			and bits >= self.get_width()):
			return self.get_width()
		else:
			return bits

