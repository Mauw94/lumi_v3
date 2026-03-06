ROADMAP (ty copilot)


🔧 Bytecode generator – missing/unfinished AST support
The visit_node matcher in generator.rs only handles a handful of node kinds.

Status	Node type	Notes
✅	Program, PrintStatement, ExpressionStatement, literals, Identifier	basic coverage
⚠️	BlockStatement	comments mention entering/leaving scope but no instructions emitted
❌	LogicalExpression, UnaryExpression	no generation at all
❌	ArrayLiteral, ObjectLiteral, MemberExpression?	not present but likely needed later
❌	ReturnStatement?	functions push Return via generator trait but statement node not handled
❌	Switch/While/DoWhile/Break/Continue	if those are part of the language spec
❌	Null/Undefined nodes	no pushing of those constants
Additionally:

call expression generation currently assumes the callee is an identifier; add support for function expressions/closures.
variable declarations don’t emit scope‑enter/leave instructions.
constant pool never deduplicates values (could be a later optimisation).
🖥 Virtual machine – instruction set & semantics
The Instruction enum declares operations that the VM doesn’t yet execute:

Mod, Inc, Dec – arithmetic is generated but VM ignores them.
Nop / Halt – useful for padding/terminating programs.
JumpIfTrue – not used in the generator, but VM supports it.
Other gaps:

no handling of Null/Undefined constants.
logical operators (&&, ||, !) aren’t implemented – they’ll need short‑circuiting.
unary operators (-, +, !) are generated in the parser but VM doesn’t know how to evaluate them.
the environment is extremely simple; globals are a fixed‑size vector and there’s no proper scope chain or closure support.
error reporting/stack traces are absent.
the function call model copies instruction vectors for every call; a proper frame pointer/heap for closures would be more efficient.
tests exist for generator only – there are no bytecode‑execution tests exercising the VM.
🔭 Language features & runtime
These are ideas pulled from the AST and what usually comes next:

Scopes & closures – implement lexical scoping, capture variables in FunctionObj.
Built‑in functions / standard library (e.g. print, clock).
Type system – the parser already reads types, but they’re ignored. Consider a semantic pass or emitting runtime checks.
Objects, arrays, indexing – there’s an ArrayLiteral node in the visitor.
Error handling – throw/catch, or at least runtime errors for undefined variables, division by zero, etc.
Garbage collection/heap – once you have objects/strings that live beyond the stack.
Optimisations – constant folding, dead code elimination, register allocation.
✅ Testing & tooling
Add VM integration tests: compile small programs, execute them, and assert on stack/globals/printed output.
Write property tests (e.g. “evaluating a + b in the VM gives same result as Rust arithmetic”).
Lint/formatting rules for the bytecode and AST crates.
Benchmarks for VM performance (there’s already a benchmark_tests.rs).
📅 Suggested roadmap
Short‑term (next sprint)

Implement missing instructions in Vm::execute (Mod, Inc, Dec, Nop, Halt).
Add generator cases for logical and unary expressions; add associated VM behaviour.
Write basic VM unit tests (arithmetic, if/for/print, simple functions).
Support Null/Undefined both in generator and VM.
Medium‑term

Flesh out scoping: enter/leave block, closures, proper environment chaining.
Expand AST/generator for arrays, objects, member access and calls on non‑identifier callees.
Add error handling/stack traces.
Begin deduplicating constants and maybe add a constant‑pool pass.
Long‑term

Build a standard library and registration of native functions.
Implement a GC/heap for non‑primitive values.
Optimise the bytecode (e.g. register-based VM, jump threading).
Add tooling: REPL, debugger, visualization of bytecode.
📝 Summary
Your engine has a working core but still needs:

more AST nodes supported in the generator,
a complete instruction implementation in the VM,
scoping/closure support,
richer language constructs, and
a proper suite of execution tests.