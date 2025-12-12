## Language

- Allowing underscores in numbers (ie 1_000_000)
- Ranges
- String interpolation

## Next Up!

- [ ] Literal compiler support (null/true/false)
- [ ] String support
  - Which means working out objects...end me pls.
- [ ] Global variables

**Global Variables**

- The basic idea is that, in the vm, globals are stored in a single vec
- GetGlobal and SetGlobal will have an opcode for the index into this vec, similar to locals
- For this the compile will need to do extra work.
- _2 Pass Compiler_
  - Run through tokens and only do global declarations
  - Means keeping track of the current depth etc.
  - When you declare a global
    - add it to the vec
    - and also have a hashmap (global_name => index)
  - Second pass does everything except global definitions
  - When it finds something trying to access a global, it uses the index in the hashmap.
- _Patching_
  - This lets the compiler run through everything
  - When it encounters trying to get an undefined identifier, it stores the info in a hashmap shown below
    - (IdentifierName => Vec<(byteOffset, token)>)
  - When it finds a global declaration, it stores it in hash (name => index)
  - At the end of compilation, it runs through the patch hashmap, and corrects all that it can.
    - If it encounters one it cannot patch, it raises an error using the token
