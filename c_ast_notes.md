The ast of
```cpp
struct Struct1
{
    int x;
};
struct Struct1 struct1_var;

typedef struct Struct2
{
    int y;
} Struct2;

typedef struct
{
    int z;
} Struct3;
Struct3 struct3_var;


struct Struct4
{
    int w;
} struct_4_var;

struct Struct5;
```
is
```yaml
FileAST: 
  Decl: None, [], [], []
    Struct: Struct1
      Decl: x, [], [], []
        TypeDecl: x, []
          IdentifierType: ['int']
  Decl: struct1_var, [], [], []
    TypeDecl: struct1_var, []
      Struct: Struct1
  Typedef: Struct2, [], ['typedef']
    TypeDecl: Struct2, []
      Struct: Struct2
        Decl: y, [], [], []
          TypeDecl: y, []
            IdentifierType: ['int']
  Typedef: Struct3, [], ['typedef']
    TypeDecl: Struct3, []
      Struct: None
        Decl: z, [], [], []
          TypeDecl: z, []
            IdentifierType: ['int']
  Decl: struct3_var, [], [], []
    TypeDecl: struct3_var, []
      IdentifierType: ['Struct3']
  Decl: struct_4_var, [], [], []
    TypeDecl: struct_4_var, []
      Struct: Struct4
        Decl: w, [], [], []
          TypeDecl: w, []
            IdentifierType: ['int']
  Decl: None, [], [], []
    Struct: Struct5
```

Consider the following kinds of statements:

```
typedef Type AliasType;
Type variable;
```

Notice that both `AliasType` and `variable` are on the right of the `Type`. If we ignore the typedef, the AST of `Type AliasType` and `Type variable` are identical, that is
```
TypeDecl: AliasType
    ...
TypeDecl: variable
    ...
```

But since `Type AliasType` has a typedef in front, the `TypeDecl`'s parent is `Typedef`. Whereas for `Type variable` the parent is `Decl`.



The ast of
```cpp
long fn1(int);
long fn2(int x);
long fn3(int, int y);
```
is
```yaml
FileAST: 
  Decl: fn1, [], [], []
    FuncDecl: 
      ParamList: 
        Typename: []
          TypeDecl: None, []
            IdentifierType: ['int']
      TypeDecl: fn1, []
        IdentifierType: ['long']
  Decl: fn2, [], [], []
    FuncDecl: 
      ParamList: 
        Decl: x, [], [], []
          TypeDecl: x, []
            IdentifierType: ['int']
      TypeDecl: fn2, []
        IdentifierType: ['long']
  Decl: fn3, [], [], []
    FuncDecl: 
      ParamList: 
        Typename: []
          TypeDecl: None, []
            IdentifierType: ['int']
        Decl: y, [], [], []
          TypeDecl: y, []
            IdentifierType: ['int']
      TypeDecl: fn3, []
        IdentifierType: ['long']

```

As we can see, the children are either `Typename` or `Decl`, where `Typename` is used when the parameter is not named.

Refs:

The ast of
```cpp
int x[4];
struct Y { int z; } y1, *y2;
void f()
{
    x[2];
    y1.z;
    y2->z;
}
```
is
```yaml
FileAST: 
  Decl: x, [], [], []
    ArrayDecl: 
      TypeDecl: x, []
        IdentifierType: ['int']
      Constant: int, 4
  Decl: y1, [], [], []
    TypeDecl: y1, []
      Struct: Y
        Decl: z, [], [], []
          TypeDecl: z, []
            IdentifierType: ['int']
  Decl: y2, [], [], []
    PtrDecl: []
      TypeDecl: y2, []
        Struct: Y
          Decl: z, [], [], []
            TypeDecl: z, []
              IdentifierType: ['int']
  FuncDef: 
    Decl: f, [], [], []
      FuncDecl: 
        TypeDecl: f, []
          IdentifierType: ['void']
    Compound: 
      ArrayRef: 
        ID: x
        Constant: int, 2
      StructRef: .
        ID: y1
        ID: z
      StructRef: ->
        ID: y2
        ID: z
```