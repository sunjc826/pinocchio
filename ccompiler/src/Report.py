class Report:
	'''Utility class used to count how many operations of each type is used.'''
	def __init__(self):
		self.table = {}

	def add(self, key, qty):
		if (key not in self.table):
			self.table[key] = 0
		self.table[key] += qty

	def __len__(self):
		return len(self.table)

	def __repr__(self):
		keys = self.table.keys()
		keys.sort()
		s = ""
		for key in keys:
			s += "%-20s: %s\n" % (key, self.table[key])
		return s
