import types
from Timing import Timing
from TypeHintHelpers import always_false
if always_false:
	import typing

from DFG import DFGExpr
from BusReq import BusReq
from Buses import Bus

class Collapser:
	'''
	Collapser is the superclass for both 
	`DFGExpr` -> `DFGExpr`|`int` collapses
	and
	`BusReq` -> `Bus` collapses

	The method typehints here are not quite accurate. I'm not going to change them for simplicity, i.e.
	(DFGExpr|BusReq) -> list[DFGExpr|int]|list[Bus] is too complex. 

	For e.g. `get_dependencies` could be `(DFGExpr)->list[DFGExpr|int]` or `(BusReq)->Bus` depending on the subclass.
	
	Also, I don't think I can add type hints for generic classes without breaking python 2 syntax.

	'''
	def __init__(self):
		self.table = {} # type: typing.Union[dict[DFGExpr, DFGExpr], dict[BusReq, Bus]]
		self.dbg_last_calc = None
		self.dbg_has_last = False

	def get_dependencies(self, key): # type: (DFGExpr) -> list[DFGExpr]
		raise Exception("abstract method")

	def collapse_impl(self, key): # type: (DFGExpr) -> DFGExpr
		'''Collapses a tree/subtree, assuming all dependencies have been resolved'''
		raise Exception("abstract method")

	def collapse_tree(self, key): # type: (DFGExpr) -> DFGExpr
		'''Collapses a tree, resolving dependencies along the way.'''
		timing = Timing("collapse_tree", enabled=False)
		stack = [key]
		loop_count = 0
		while (len(stack)>0):
			timing.phase("collapser loop # %s setup" % loop_count)
			loop_count += 1
			key = stack[-1]
			if (key in self.table):
				# oh. handy. we already did this dude: he was just
				# wanted multiple times.
				stack.pop()
				continue
			alldeps = self.get_dependencies(key) # type: list
			timing.phase("collapser loop # %s get_deps (%d) self %s" % (loop_count, len(alldeps), self.__class__))

#			def study(table):
#				keys = table.keys()
#				keys.sort()
#				hist = {}
#				for k in keys:
#					h = hash(k)
#					if (h not in hist):
#						hist[h] = []
#					hist[h].append(k)
#				def by_len(a,b):
#					return cmp(len(a), len(b))
#				v = hist.values()
#				v.sort(by_len)
#				v = v[::-1]
#				print "%d keys, %d unique hashes, worst duplication: %s" % (
#					len(keys), len(hist), v[:10])
#				for k in v[0]:
#					print "%d %s" % (hash(k), k)
#				raise Exception("done")

			def notintable(d):
#				if (len(self.table)>4000):
#					study(self.table)
				return d not in self.table
			newdeps = filter(notintable, alldeps)
			if (newdeps==[]):
				stack.pop()
				assert(key not in self.table)
				result = self.collapse_impl(key)
				self.table[key] = result
				timing.phase("collapser loop # %s collapse_impl" % loop_count)
			else:
				stack += newdeps
			timing.phase("collapser loop # %s end" % loop_count)
		timing.phase("done")
#		print "collapser loop_count: %d" % loop_count
		return self.table[key]

	def lookup(self, key): # type: (DFGExpr) -> DFGExpr
		# upcall from impl getting collapsed to find his dependencies' values
		return self.table[key]
