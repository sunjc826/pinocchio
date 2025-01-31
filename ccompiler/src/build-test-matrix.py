#!/usr/bin/python

import subprocess
import os
import sys
mypath = os.path.dirname(__file__)
sys.path.append("%s/../../common" % mypath)
from App import App

BUILD="build/"
GCC="gcc -g -I../../common -Ibuild/".split(' ')

def dash_to_under(s): # type: (str) -> str
	if (s[0]=="-"):
		s = "_"+s[1:]
	return s

class OneLiner:
	'''
	Emits
	<line>
	'''
	def __init__(self, line): # type: (str) -> None
		self.line = line

	def emit(self):
		return self.line

class Comment(OneLiner):
	'''
	Emits
	<line>
	'''
	def __init__(self, line): # type: (str) -> None
		OneLiner.__init__(self, line)


class Directive(OneLiner):
	'''
	Emits
	<line>
	'''
	def __init__(self, line): # type: (str) -> None
		OneLiner.__init__(self, line)

class Make:
	'''
	Emits
	<output>: <inputs...>
		<cmdlist...>
	'''
	def __init__(self, output, inputs, cmdlist): # type: (str, list[str], list[str]) -> None
		self.output = output
		self.inputs = inputs
		self.cmdlist = cmdlist

	def emit(self):
		if (len(self.inputs) > 3):
			dependencies = " \\\n\t".join(self.inputs)
		else:
			dependencies = " ".join(self.inputs)

		if (self.cmdlist==[]):
			rule = ""
		else:
			rule = "	%s\n" % " ".join(self.cmdlist)

		return "%s: %s\n%s\n" % (
			self.output,
			dependencies,
			rule)

class BuildTestMatrix:
	'''
	The output Makefile is called make.matrix
	make_rules: The individual Makefile targets
	precious_targets: The result of the Makefile matrix-all default target
	'''
	def __init__(self):
		apps = App.defaultApps() 
		self.make_rules = [] # type: list[str]
		self.precious_targets = [] # type: list[str]
		self.random_headers_created = set() # type: set[str]

		self.make_random_header = BUILD+"make-random-header"
# obsoleted by include
#		output = self.make_random_header
#		inputs = [ "make-random-header.c" ]
#		self.make(output, inputs,
#			[ "gcc", "-o", output ] + inputs)

		#self.system(["make"])	# be sure common .os are built
		for app in apps:
			self.make_rules.append(Comment("# App %s\n" % app.name))
			for param in app.params:
				self.make_rules.append(Comment("#   Param %s\n" % param))
				for bitwidth in app.bitwidths:
					self.make_rules.append(Comment("#     Bitwidth %s\n" % bitwidth))
					self.build(app, param, bitwidth)

		all_rule = Make("matrix-all", ["build"]+self.precious_targets, [])
		self.make_rules = [all_rule] + self.make_rules
		self.make_rules.append(Directive("include make.in\n"))

		makefp = open("make.matrix", "w")
		makefp.write("PYTHON=python2\n\n")
		for makerule in self.make_rules:
			makefp.write(makerule.emit())
		makefp.close()
	
	def make(self, output, inputs, cmdlist): # type: (str, list[str], list[str]) -> None
		'''Adds a makefile target'''
		self.make_rules.append(Make(output, inputs, cmdlist))

	def precious(self, target): # type: (str) -> None
		'''Adds a target to the matrix-all target'''
		self.precious_targets.append(target)

	def get_random_config(self, app, param): # type: (App, int) -> dict[str, int]
		gcc = subprocess.Popen([
			"gcc",
			"-DQUERY_RANDOM_CONFIG",
			"-DPARAM=%d" % param,
			"-E",
			"%s.c" % app.name], stdout=subprocess.PIPE)
		grep = subprocess.Popen(["grep", "pragma message"],
			stdin=gcc.stdout, stdout=subprocess.PIPE)
		gcc.stdout.close()
		text = grep.communicate()[0]
		grep.stdout.close()
		lines = text.split('\n')
		config = {} # type: dict[str, int]
		for line in lines:
			fields = line.split()
			if (len(fields)<4):
				continue
			var = fields[2].replace("'", "")
			value = eval(fields[3]) # type: int
			config[var] = value
		return config

	def add_random_header(self, random_config): # type: (dict[str, int]) -> str
		random_header = BUILD+"random-header-%d-%d.h" % (
			random_config["RANDOM_SIZE"], random_config["RANDOM_REDUCE"])
		if (random_header not in self.random_headers_created):
			output = random_header
			inputs = [ self.make_random_header ]
			self.make(output, inputs,
				[ self.make_random_header,
					output,
					str(random_config["RANDOM_SIZE"]),
					str(random_config["RANDOM_REDUCE"]) ])
			self.random_headers_created.add(random_header)
		return random_header

	def build(self, app, param, bitwidth): # type: (App, int, int) -> None
		random_config = self.get_random_config(app, param)
		needs_random = "RANDOM_SIZE" in random_config

		app_base = "%s-p%s-b%s" % (app.name, param, bitwidth)
		DPARAM = "-DPARAM=%d" % param
		DBIT_WIDTH = "-DBIT_WIDTH=%d" % bitwidth
		random_header_cpparg = []
		rh_cpparg = [ ]

		if (bitwidth==32):
			qsp_o = BUILD+("qsp-test-%s-p%s.o" % (app.name, param))

			if (needs_random):
				# make the requisite .h
				random_header = self.add_random_header(random_config)
				rh_input_list = [ random_header ]
				rh_include_list = [ "-include", random_header ]
				rh_cpparg = [ dash_to_under("-include"), random_header ]
			else:
				random_header = None
				rh_input_list = [ ]
				rh_include_list = [ ]
				rh_cpparg = [ ]

			# build the C executable module
			output = BUILD+app_base+"-native.o"
			c_file = app.name+".c"
			inputs = [ c_file ] + rh_input_list
			self.make(output, inputs,
				GCC+["-c", "-o", output, c_file]+rh_include_list+["-DQSP_TEST", DPARAM, DBIT_WIDTH])

			# build the C test module
			output = BUILD+app_base+"-test.o"
			inputs = [ app.name+"-test.c" ]
			self.make(output, inputs,
				GCC+["-c", "-o", output]+inputs+["-DQSP_TEST", DPARAM, DBIT_WIDTH])

			# param-specific qsp module
			output = qsp_o
			inputs = ["qsp-test.c"]
			self.make(output, inputs,
				GCC+["-c", "-o", output]+inputs+[DPARAM])

			# link the C program
			output = BUILD+app_base
			inputs = [ BUILD+app_base+"-native.o",
						BUILD+app_base+"-test.o",
						qsp_o,
						BUILD+"wire-io.o",
						BUILD+"print-matrix.o" ]
			self.make(output, inputs,
				GCC +["-o", output] + inputs)
			self.precious(output)

		moreflags = []
		if (app.ignore_overflow):
			moreflags += ["--ignore-overflow", "True"]
		if os.environ["VC_LOOP_SANITY_LIMIT"]:
			moreflags += ["--loop-sanity-limit", os.environ["VC_LOOP_SANITY_LIMIT"]]
		if os.environ["VC_NOVA_CIRCUIT_RS_DIR"]:
			moreflags += ["--nova-circuit-rs-dir", os.environ["VC_NOVA_CIRCUIT_RS_DIR"]]

		output = BUILD+app_base+".arith"
		inputs = [ app.name+".c" ]
		self.make(output, inputs,
			["$(PYTHON) ../src/vercomp.py"]+inputs+[
				"--app-name", app.name,
				"--arith", output,
				"--bit-width", str(bitwidth),
				"--cpparg"]
				+map(dash_to_under, [
					"-Ibuild/", DPARAM, DBIT_WIDTH
					]+rh_cpparg)
				+moreflags
				)
		self.precious(output)


BuildTestMatrix()
