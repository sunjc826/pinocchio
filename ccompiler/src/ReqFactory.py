import operator
import os
import subprocess
import sys
import traceback
import CaseHelper
import jinja
from Collapser import Collapser
from Board import Board
from Report import Report
from DFG import *
from BusReq import *
from TypeHintHelpers import always_false
if always_false:
	import Buses
	import ArithBuses
	import BooleanBuses
	import ArithBusReq
	import BooleanBusReq

'''
Transformation sequence:
C AST -> DFGExpr -> BusReq -> Bus ---(Assign wires to buses)---> FieldOps -> Output file (e.g. .arith file)
'''

class ReqFactory(Collapser):
	def __init__(self, app_name, output_filename, nova_circuit_rs_dir, circuit_inputs, circuit_nizk_inputs, circuit_outputs, bit_width):
		# type: (str, str, str, list[Input], list[NIZKInput], list[tuple[Key, DFGExpr]], BitWidth) -> None
		Collapser.__init__(self)

		# Setup
		self._board = Board(max_width=252, bit_width=bit_width)
		self.buses = set() # type: set[Bus]
		self.truncated_bus_cache = {} # type: dict[DFGExpr, ArithBuses.JoinBus]

		self.add_extra_bus(self.get_board().get_one_bus())
		zero_bus = self.make_zero_bus()
		self._board.set_zero_bus(zero_bus)
		self.add_extra_bus(zero_bus)

		# Generation of Buses from DFGExprs
		self.process_outputs(circuit_outputs)
		self.process_inputs(circuit_inputs, circuit_nizk_inputs)
		
		# Generation of FieldOps from Buses
		self.layout_buses()

		# Generation of output file from FieldOps
		self.emit(output_filename)

		# Generation of rust files
		self.emit_rs(app_name, nova_circuit_rs_dir)

		# Sanity checking and optimization statistics collection
		self.report()
		self.lint()

	def type(self): # type: () -> TraceType
		raise Exception("abstract")

	def make_zero_bus(self): # type: () -> ArithBuses.ArithZero | BooleanBuses.BooleanZero 
		raise Exception("abstract")

	def make_output_bus(self, expr_bus, idx): # type: (Bus, int) -> ArithBuses.ArithmeticOutputBus | BooleanBuses.BooleanOutputBus
		raise Exception("abstract")
	
	def make_input_req(self, expr): # type: (DFGExpr) -> ArithBusReq.ArithmeticInputReq | BooleanBusReq.BooleanInputReq
		raise Exception("abstract")

	def make_req(self, expr, type): # type: (DFGExpr, TraceType) -> BusReq
		'''
		These requests generate `BOOLEAN_TYPE` buses, even in an `ArithFactory`,
		so they're common code.
		'''
		if (isinstance(expr, BitAnd)):
			result = BitAndReq(self, expr, type)
		elif (isinstance(expr, BitOr)):
			result = BitOrReq(self, expr, type)
		elif (isinstance(expr, BitNot)):
			result = BitNotReq(self, expr, type)
		elif (isinstance(expr, LogicalNot)):
			result = LogicalNotReq(self, expr, type)
		elif (isinstance(expr, Xor)):
			result = XorReq(self, expr, type)
		elif (isinstance(expr, LeftShift)):
			result = LeftShiftReq(self, expr, type)
		elif (isinstance(expr, RightShift)):
			result = RightShiftReq(self, expr, type)
		else:
			print(expr.__class__)
			assert(False)
		return result


	# Generate all the outputs
	def process_outputs(self, circuit_outputs): # type: (list[tuple[Key, DFGExpr]]) -> None
		for idx in range(len(circuit_outputs)):
			(name,expr) = circuit_outputs[idx]
			expr_bus = self.collapse_tree(
				self.make_req(expr, self.type())) # type: Bus
			bus = self.make_output_bus(expr_bus, idx)
			self.add_extra_bus(bus)

	# Ensure all inputs are represented in circuit,
	# even if they're unused.
	def process_inputs_core(self, input_list, method):  # type: (list[InputBase], Callable[[DFGExpr], BusReq]) -> None
		for idx in range(len(input_list)):
			#print "processing %s %d/%d" % (method.__name__, idx, len(input_list))
			expr = input_list[idx]
			req = method(expr)
			try:
				bus = self.lookup(req) # type: ArithBuses.ArithmeticInputBaseBus | BooleanBuses.BooleanInputBus
				bus.set_used(True)
			except KeyError:
				bus = self.collapse_tree(req)
				bus.set_used(False)

	def process_inputs(self, circuit_inputs, circuit_nizk_inputs): # type: (list[InputBase], list[InputBase]) -> None
		self.process_inputs_core(circuit_inputs, self.make_input_req)
		self.process_inputs_core(circuit_nizk_inputs, self.make_nizk_input_req)
		
	def debug_investigate_buses(self, bus):
		print("--------------------------------------------------")
		print("add_extra_bus(%s)" % bus.__class__.__name__)
		try:
			raise Exception()
		except:
			sys.stdout.flush()
			sys.stderr.flush()
			traceback.print_stack()
			sys.stdout.flush()
			sys.stderr.flush()
		self.debug_print_buses()
		print

	def add_extra_bus(self, bus): # type: (Bus) -> None
		self.buses.add(bus)
		#self.debug_investigate_buses(bus)

	def get_board(self):
		return self._board

	def collapser(self):
		return self

	def get_dependencies(self, key): # type: (BusReq) -> list[BusReq]
		return key.get_dependencies()

	def collapse_impl(self, key): # type: (BusReq) -> Bus
		bus = key.collapse_impl()
		self.buses.add(bus)
		#self.debug_investigate_buses(bus)
		return bus

	def collapse_req(self, req): # type: (BusReq) -> Bus
		raise Exception("abstract")
	
	def debug_print_buses(self):
		bus_list = list(self.buses)
		bus_list.sort()
		for bus_i in range(len(bus_list)):
			print("  bus[%d] %s" % (
				bus_i, bus_list[bus_i].__class__.__name__))

	# layout phase, after the wiring diagram has been collected.
	def layout_buses(self):
		# sort buses into a sensible order (so references are
		# defined before they're used), and then allocate wire ids.
		bus_list = list(self.buses)
		bus_list.sort()
		#self.debug_print_buses()

		next_idx = 0
		for bus in bus_list:
			#print "layout_buses visits %s" % bus
			bus_wire_count = bus.get_wire_count()
			allocated_wires = map(Wire,
				range(next_idx, next_idx+bus_wire_count))
			bus.assign_wires(WireList(allocated_wires))
			next_idx += bus_wire_count

		self.bus_list = bus_list
		self.total_wire_count = next_idx

	def emit(self, output_filename): # type: (str) -> None
		'''
Example:

total 14

input 0                                  # input

input 1                                  # input

input 2                                  # one-input

const-mul-0 in 1 <2> out 1 <3>           # zero

const-mul-2 in 1 <1> out 1 <4>           # multiply-by-constant 2

const-mul-5 in 1 <2> out 1 <5>           # constant 5

add in 2 <0 5> out 1 <6>                 # ArithBusReq.AddReq(DFG.Input,DFG.Constant)

const-mul-neg-1 in 1 <4> out 1 <7>       # zerop subtract negative

add in 2 <6 7> out 1 <8>                 # zerop diff

zerop in 1 <8> out 2 <10 9>              # zerop <ArithBuses.ArithAddBus instance at 0x7f0f3678b550>

const-mul-neg-1 in 1 <9> out 1 <11>      # zerop inverse

add in 2 <2 11> out 1 <12>               # zerop result

mul in 2 <2 12> out 1 <13>               # output-cast

output 13                                # 

		'''
		tmp_output_filename = output_filename+".tmp"
		ofp = open(tmp_output_filename, "w")
		ofp.write("total %d\n" % self.total_wire_count)
		for bus in self.bus_list:
			field_ops = bus.get_field_ops()
			for op in field_ops:
				ofp.write("%s\n" % repr(op))
		ofp.close()
		os.rename(tmp_output_filename, output_filename)

	def emit_rs(self, app_name, nova_circuit_rs_dir): # type: (str, str) -> None
		print(app_name)
		camelcase_app_name = CaseHelper.kebab_to_camel(app_name)
		snakecase_app_name = CaseHelper.kebab_to_snake(app_name)
		rs_circuit_struct_name = "%sCircuit" % camelcase_app_name
		
		# The following logic assumes that the bus list is laid out in the format
		# input..., one-input, logic buses, output...
		# We also assume that one input bus corresponds to one input wire
		num_input_wires = 0
		while num_input_wires < len(self.bus_list) and self.bus_list[num_input_wires].is_runtime_input():
			num_input_wires += 1
		
		num_output_wires = 0
		while num_output_wires < len(self.bus_list) and self.bus_list[len(self.bus_list) - num_output_wires - 1].is_output():
			num_output_wires += 1
				
		rs_arity = num_input_wires

		rs_synthesize = ""

		
		
		environment = jinja.Environment(loader=jinja.FileSystemLoader(nova_circuit_rs_dir))
		template = environment.get_template("pinocchio.template")
		content = template.render(
			rs_circuit_struct_name = rs_circuit_struct_name,
			rs_arity = rs_arity,
			rs_synthesize = rs_synthesize
		)

		output_file_name = "%s/%s.rs" % (nova_circuit_rs_dir, snakecase_app_name)
		print("Generating file in " + output_file_name)
		with open(output_file_name, mode = "w") as output:
			output.write(content)


	def report(self):
		'''Make a summary of the number of various operations used.'''
		r = Report()
		for bus in self.bus_list:
			field_ops = bus.get_field_ops()
			for op in field_ops:
				op.report(r)
		print(r)

	def lint(self):
		wire_to_field_op = {}	# maps wires to the field_op that output it
		specified_outputs = {}	# maps wires to buses
		field_ops = []
		# check that each wire is specified no more than once
		for bus in self.bus_list:
			for field_op in bus.get_field_ops():
				field_ops.append(field_op)
				output_wires = field_op.output_wires()
#				print "field_op %s" % field_op
#				print "  %s" % output_wires
				for output in output_wires:
					if (output in specified_outputs):
						print("LINT: bus %s re-specifies wire %s, already set by bus %s" % (
							bus, output, specified_outputs[output]))
					else:
						specified_outputs[output] = bus
					wire_to_field_op[output] = field_op

		# check that each wire is specified at least once
		for idx in range(self.total_wire_count):
			wire = Wire(idx)
			if (wire not in specified_outputs):
				print("LINT: no bus specifies output %s" % wire)

		# check that each field_op computes a value that eventually reaches
		# an output.
		useful_field_ops = set(filter(
			lambda f: isinstance(f, FieldOutput), field_ops))
		done_field_ops = set()
		while (len(useful_field_ops)>0):
			#print "Considering %s field_ops" % len(useful_field_ops)
			required_wires = set()
			for field_op in useful_field_ops:
				#print "inputs of %s are %s" % (field_op, field_op.input_wires())
				required_wires.update(field_op.input_wires())
			done_field_ops.update(useful_field_ops)

			#print "Considering %s required_wires" % len(required_wires)
			useful_field_ops = set()
			for wire in required_wires:
				print(wire)
				field_op = wire_to_field_op[wire]
				if (field_op not in done_field_ops):
					useful_field_ops.add(field_op)

		all_field_ops = set(field_ops)
		unused_field_ops = all_field_ops.difference(done_field_ops)
		r = Report()
		for field_op in unused_field_ops:
			field_op.report(r)
		if (len(r)>0):
			print("LINT: %d unused field ops; cost:\n%s" % (len(unused_field_ops), r))
			# print "LINT: %s" % unused_field_ops

		print("(info) Linted %s field ops from %s buses" % (
			len(field_ops), len(self.bus_list)))

	def is_req_factory(self): return True	# just a pseudostatic type assertion

