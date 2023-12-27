def get_false(): # type: () -> bool
	return False
always_false = get_false()
'''
This variable is purposely not a constant to trick
the language server into statically importing the type information.
This avoid unnecessary imports and dependency cycles during runtime.

EXAMPLE
```
if always_false:
	import XXXLib

# We can then use the type information from XXXLib, such as
def fn(param): # type: (XXXLib.SomeType) -> None
	pass
```
'''

if always_false: # We need this since python2 will most likely not accept the following statements
	from typing import Any, TypeVar
	C = TypeVar('C')
	T = TypeVar('T')
def force_cast(item, item_type): # type: (Any, type[T]) -> T
	'''Fool the Intellisense, even if `isinstance(item, item_type)` is False'''
	return item
def force_cast_list(item, item_type): # type: (Any, type[T]) -> list[T]
	return item
def force_cast_container(item, container_type, item_type): # type: (Any, type[C], type[T]) -> C[T]
	return item
	