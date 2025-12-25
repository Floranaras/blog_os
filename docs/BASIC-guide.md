# CarlOS BASIC Programming Language Reference

Documentation for the **CarlOS BASIC Interpreter**, a minimal implementation designed for the CarlOS kernel. It supports classic BASIC commands with line numbers for stored programs and immediate execution for direct commands.

---

## Getting Started

### Two Modes of Operation
1.  **Immediate Mode**: Commands without line numbers execute immediately.
    ```basic
    PRINT "HELLO"
    ```
2.  **Program Mode**: Commands with line numbers are stored in memory for execution.
    ```basic
    10 PRINT "HELLO"
    20 END
    RUN
    ```

---

## System Commands

| Command | Description |
| :--- | :--- |
| **`LIST`** | Displays all lines in the current program, sorted by line number. |
| **`RUN`** | Executes the current program from the first line. |
| **`NEW`** | Clears the current program and resets all variables to 0. |
| **`SAVE name`** | Saves the current program with the specified name (max 8 programs). |
| **`LOAD name`** | Loads a previously saved program. |
| **`DIR`** | Lists all saved programs. |
| **`DELETE n`** | Deletes the specified line number from the program. (Shorthand: `DEL n`) |
| **`EXIT`** | Exits the BASIC interpreter. |

**Examples:**
```basic
SAVE MYPROGRAM
LOAD MYPROGRAM
DELETE 20          # Remove line 20
DEL 30             # Remove line 30 (shorthand)
```

### Editing Programs
There are two ways to edit program lines:

1. **Overwrite a line**: Type the same line number with new code.
   ```basic
   10 PRINT "OLD"
   10 PRINT "NEW"     # Line 10 is now replaced
   ```

2. **Delete a line**: Use the `DELETE` or `DEL` command.
   ```basic
   DELETE 20          # Removes line 20 completely
   ```

---

## Programming Commands

### `PRINT expression`
Prints a string (in quotes), variable, or the result of an expression.
```basic
PRINT "HELLO WORLD"
PRINT X
PRINT X + 5
```

### `LET variable = expression`
Assigns a value to a variable (A-Z). Supports arithmetic expressions.
```basic
LET A = 10
LET B = A + 5
LET C = A * B / 2
```

### `GOTO line_number`
Jumps execution to the specified line number.
```basic
10 PRINT "LOOP"
20 GOTO 10
```

### `IF condition THEN statement`
Executes the statement only if the condition is true.
```basic
IF X > 5 THEN PRINT "BIG"
IF A = B THEN GOTO 100
```

### `FOR variable = start TO end` / `NEXT`
Creates a loop that increments a variable from start to end (inclusive).
```basic
10 FOR I = 1 TO 10
20 PRINT I
30 NEXT
```

### `INPUT variable`
Prompts for user input and stores it in a variable. 
*(Note: Currently sets to 0 - full input functionality not yet implemented).*

### `END`
Stops program execution normally.

### `STOP`
Immediately halts program execution and displays "Program stopped".
```basic
10 IF X > 100 THEN STOP
20 LET X = X + 1
30 GOTO 10
```

---

## Variables
- **Naming**: Single letters **A-Z** (case insensitive).
- **Type**: All variables are integers (**i32**).
- **Initialization**: All variables are initialized to **0**.
- **Capacity**: 26 variables available (one for each letter).

---

## Expressions and Operators

### Arithmetic Operators
| Operator | Description | Note |
| :---: | :--- | :--- |
| `+` | Addition | |
| `-` | Subtraction | |
| `*` | Multiplication | |
| `/` | Division | Integer division; division by zero returns 0. |

### Comparison Operators
| Operator | Description |
| :---: | :--- |
| `>` | Greater than |
| `<` | Less than |
| `>=` | Greater than or equal to |
| `<=` | Less than or equal to |
| `=` | Equal to |
| `<>` | Not equal to |

---

## Safety Features

### Infinite Loop Protection
The interpreter automatically stops programs after **1,000,000 instructions** to prevent infinite loops from freezing the system.

**What happens:**
```basic
10 GOTO 10         # Infinite loop
RUN
# After 1,000,000 iterations:
# ERROR: Program stopped - too many instructions (possible infinite loop)
```

**Why this helps:**
- Prevents system freezes from accidental infinite loops
- Allows you to regain control without rebooting
- Makes debugging safer and easier

**Example of safe loop:**
```basic
10 LET X = 1
20 PRINT X
30 LET X = X + 1
40 IF X <= 100 THEN GOTO 20
50 END
# This runs fine - only 100 iterations
```

---

## Complete Program Examples

### Example 1: Simple Counter
```basic
10 LET X = 1
20 PRINT X
30 LET X = X + 1
40 IF X <= 10 THEN GOTO 20
50 END
```

### Example 2: Multiplication Table
```basic
10 FOR I = 1 TO 10
20 LET R = I * 5
30 PRINT R
40 NEXT
50 END
```

### Example 3: Conditional Logic
```basic
10 LET A = 5
20 LET B = 10
30 IF A < B THEN PRINT "A IS SMALLER"
40 IF A > B THEN PRINT "A IS BIGGER"
50 END
```

### Example 4: Editing a Program
```basic
# Create a program with a bug
10 PRINT "START"
20 LET X = 1
30 PRINT X
40 GOTO 30         # Oops! Infinite loop

# Fix it without starting over
DELETE 40          # Remove the bad line
40 LET X = X + 1   # Add correct line
45 IF X <= 10 THEN GOTO 30
50 END
LIST               # Verify the changes
RUN                # Now it works correctly!
```

---

## Current Limitations
- **Lines**: Maximum 256 program lines.
- **Line Length**: Maximum 80 characters per line.
- **Storage**: Maximum 8 saved programs.
- **Variables**: 26 variables (A-Z only).
- **Math**: Integer arithmetic only (no decimals).
- **Input**: `INPUT` command not fully implemented.
- **Strings**: No string variables (only literals in `PRINT`).
- **Nesting**: Maximum 8 nested `FOR` loops.
- **Instructions**: Programs stop after 1,000,000 instructions (infinite loop protection).

---

## Programming Tips
1.  **Line Numbering**: Use increments of 10 (10, 20, 30...) to leave room for future insertions.
2.  **Verification**: Use `LIST` to verify your program before running it.
3.  **Loops**: Ensure every `FOR` has a matching `NEXT` statement.
4.  **Backups**: `SAVE` your program before testing complex changes.
5.  **Case Sensitivity**: Commands are case-insensitive (`PRINT`, `print`, and `Print` all work).
6.  **Editing**: Use `DELETE` to remove lines or just retype the line number to replace it.
7.  **Loop Safety**: The interpreter will stop infinite loops automatically after 1M instructions.

---

## Common Mistakes and How to Fix Them

### Infinite Loop (Now Protected!)
**Problem:**
```basic
10 GOTO 10         # Runs forever
```
**What happens:** Program stops automatically after 1,000,000 iterations with an error message.

**Fix:** Add a termination condition:
```basic
10 LET X = X + 1
20 IF X > 100 THEN END
30 GOTO 10
```

### Missing NEXT Statement
**Problem:**
```basic
10 FOR I = 1 TO 10
20 PRINT I
30 END             # Missing NEXT!
```
**Fix:** Always match FOR with NEXT:
```basic
10 FOR I = 1 TO 10
20 PRINT I
30 NEXT            # Added NEXT
40 END
```

### Fixing Mistakes Without Restarting
**Don't do this:**
```basic
NEW                # Loses all your work!
# Start typing everything again...
```

**Do this instead:**
```basic
LIST               # See what you have
DELETE 20          # Remove just the bad line
20 PRINT "FIXED"   # Add the correct version
LIST               # Verify the fix
```

---

## Quick Reference Card

**System Commands**
`LIST` | `RUN` | `NEW` | `SAVE name` | `LOAD name` | `DIR` | `DELETE n` | `DEL n` | `EXIT`

**Programming Commands**
`PRINT` | `LET` | `GOTO` | `IF...THEN` | `FOR...TO...NEXT` | `INPUT` | `END` | `STOP`

**Operators**
- **Arithmetic**: `+`, `-`, `*`, `/`
- **Comparison**: `>`, `<`, `>=`, `<=`, `=`, `<>`

**Editing**
- Replace line: Just type the line number again with new code
- Delete line: `DELETE line_number` or `DEL line_number`

**Safety**
- Programs automatically stop after 1,000,000 instructions
- Use `STOP` command to manually halt execution
