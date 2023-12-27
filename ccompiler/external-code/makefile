all: pycparser-2.08/pycparser/plyparser.py ply-3.4/ply/yacc.py

pycparser-2.08/pycparser/plyparser.py:
	wget --no-check-certificate https://pypi.python.org/packages/source/p/pycparser/pycparser-2.08.tar.gz
	tar xzf pycparser-2.08.tar.gz

ply-3.4/ply/yacc.py:
	wget 'http://www.dabeaz.com/ply/ply-3.4.tar.gz'
	tar xzf ply-3.4.tar.gz

pydot:
	python2 -m pip install --target ./dep pyparsing==1.5.7 pydot==1.0.28