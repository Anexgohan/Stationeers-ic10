# Section A:
# Links:
[stationeering.com/tools/ic](https://stationeering.com/tools/ic)

Stationeers Instruction Set
Comments are made by using the # symbol, either at the start of a line or after an instruction.

Device IO
Instruction	Fields
bdns	d?	a(r?|num)	 	 
» Branch to line a if device d isn't set
bdnsal	d?	a(r?|num)	 	 
» Jump execution to line a and store next line number if device is not set
bdse	d?	a(r?|num)	 	 
» Branch to line a if device d is set
bdseal	d?	a(r?|num)	 	 
» Jump execution to line a and store next line number if device is set
brdns	d?	a(r?|num)	 	 
» Relative jump to line a if device is not set
brdse	d?	a(r?|num)	 	 
» Relative jump to line a if device is set
l	r?	d?	var	 
» Loads device var to register.
lb	r?	type	var	batchMode
» Loads var from all output network devices with provided type hash using the provide batch mode. Average (0), Sum (1), Minimum (2), Maximum (3). Can use either the word, or the number.
lr	r?	d?	reagentMode	reagent
» Loads reagent of device's reagentMode to register. Contents (0), Required (1), Recipe (2). Can use either the word, or the number.
ls	r?	d?	int	var
» Loads slot var on device to register.
s	d?	var	r?	 
» Stores register value to var on device.
sb	type	var	r?	 
» Stores register value to var on all output network devices with provided type hash.
Flow Control, Branches and Jumps
Instruction	Fields
bap	a(r?|num)	b(r?|num)	c(r?|num)	d(r?|num)
» Branch to line d if abs(a - b) <= max(c * max(abs(a), abs(b)), float.epsilon * 8)
bapal	a(r?|num)	b(r?|num)	c(r?|num)	d(r?|num)
» Branch to line c if a != b and store next line number in ra
bapz	a(r?|num)	b(r?|num)	c(r?|num)	 
» Branch to line c if abs(a) <= float.epsilon * 8
bapzal	a(r?|num)	b(r?|num)	c(r?|num)	 
» Branch to line c if abs(a) <= float.epsilon * 8
beq	a(r?|num)	b(r?|num)	c(r?|num)	 
» Branch to line c if a == b
beqal	a(r?|num)	b(r?|num)	c(r?|num)	 
» Branch to line c if a == b and store next line number in ra
beqz	a(r?|num)	b(r?|num)	 	 
» Branch to line b if a == 0
beqzal	a(r?|num)	b(r?|num)	 	 
» Branch to line b if a == 0 and store next line number in ra
bge	a(r?|num)	b(r?|num)	c(r?|num)	 
» Branch to line c if a >= b
bgeal	a(r?|num)	b(r?|num)	c(r?|num)	 
» Branch to line c if a >= b and store next line number in ra
bgez	a(r?|num)	b(r?|num)	 	 
» Branch to line b if a >= 0
bgezal	a(r?|num)	b(r?|num)	 	 
» Branch to line b if a >= 0 and store next line number in ra
bgt	a(r?|num)	b(r?|num)	c(r?|num)	 
» Branch to line c if a > b
bgtal	a(r?|num)	b(r?|num)	c(r?|num)	 
» Branch to line c if a > b and store next line number in ra
bgtz	a(r?|num)	b(r?|num)	 	 
» Branch to line b if a > 0
bgtzal	a(r?|num)	b(r?|num)	 	 
» Branch to line b if a > 0 and store next line number in ra
ble	a(r?|num)	b(r?|num)	c(r?|num)	 
» Branch to line c if a <= b
bleal	a(r?|num)	b(r?|num)	c(r?|num)	 
» Branch to line c if a <= b and store next line number in ra
blez	a(r?|num)	b(r?|num)	 	 
» Branch to line b if a <= 0
blezal	a(r?|num)	b(r?|num)	 	 
» Branch to line b if a <= 0 and store next line number in ra
blt	a(r?|num)	b(r?|num)	c(r?|num)	 
» Branch to line c if a < b
bltal	a(r?|num)	b(r?|num)	c(r?|num)	 
» Branch to line c if a < b and store next line number in ra
bltz	a(r?|num)	b(r?|num)	 	 
» Branch to line b if a < 0
bltzal	a(r?|num)	b(r?|num)	 	 
» Branch to line b if a < 0 and store next line number in ra
bna	a(r?|num)	b(r?|num)	c(r?|num)	d(r?|num)
» Branch to line d if abs(a - b) > max(c * max(abs(a), abs(b)), float.epsilon * 8)
bnaal	a(r?|num)	b(r?|num)	c(r?|num)	d(r?|num)
» Branch to line d if abs(a - b) <= max(c * max(abs(a), abs(b)), float.epsilon * 8) and store next line number in ra
bnaz	a(r?|num)	b(r?|num)	c(r?|num)	 
» Branch to line c if abs(a) > float.epsilon * 8
bnazal	a(r?|num)	b(r?|num)	c(r?|num)	 
» Branch to line c if abs(a) > float.epsilon * 8
bne	a(r?|num)	b(r?|num)	c(r?|num)	 
» Branch to line c if a != b
bneal	a(r?|num)	b(r?|num)	c(r?|num)	 
» Branch to line c if a != b and store next line number in ra
bnez	a(r?|num)	b(r?|num)	 	 
» branch to line b if a != 0
bnezal	a(r?|num)	b(r?|num)	 	 
» Branch to line b if a != 0 and store next line number in ra
brap	a(r?|num)	b(r?|num)	c(r?|num)	d(r?|num)
» Relative branch to line d if abs(a - b) <= max(c * max(abs(a), abs(b)), float.epsilon * 8)
brapz	a(r?|num)	b(r?|num)	c(r?|num)	 
» Relative branch to line c if abs(a) <= float.epsilon * 8
breq	a(r?|num)	b(r?|num)	c(r?|num)	 
» Relative branch to line c if a == b
breqz	a(r?|num)	b(r?|num)	 	 
» Relative branch to line b if a == 0
brge	a(r?|num)	b(r?|num)	c(r?|num)	 
» Relative jump to line c if a >= b
brgez	a(r?|num)	b(r?|num)	 	 
» Relative branch to line b if a >= 0
brgt	a(r?|num)	b(r?|num)	c(r?|num)	 
» relative jump to line c if a > b
brgtz	a(r?|num)	b(r?|num)	 	 
» Relative branch to line b if a > 0
brle	a(r?|num)	b(r?|num)	c(r?|num)	 
» Relative jump to line c if a <= b
brlez	a(r?|num)	b(r?|num)	 	 
» Relative branch to line b if a <= 0
brlt	a(r?|num)	b(r?|num)	c(r?|num)	 
» Relative jump to line c if a < b
brltz	a(r?|num)	b(r?|num)	 	 
» Relative branch to line b if a < 0
brna	a(r?|num)	b(r?|num)	c(r?|num)	d(r?|num)
» Relative branch to line d if abs(a - b) > max(c * max(abs(a), abs(b)), float.epsilon * 8)
brnaz	a(r?|num)	b(r?|num)	c(r?|num)	 
» Relative branch to line c if abs(a) > float.epsilon * 8
brne	a(r?|num)	b(r?|num)	c(r?|num)	 
» Relative branch to line c if a != b
brnez	a(r?|num)	b(r?|num)	 	 
» Relative branch to line b if a != 0
j	int	 	 	 
» Jump execution to line a
jal	int	 	 	 
» Jump execution to line a and store next line number in ra
jr	int	 	 	 
» Relative jump to line a
Variable Selection
Instruction	Fields
sap	r?	a(r?|num)	b(r?|num)	c(r?|num)
» Register = 1 if abs(a - b) <= max(c * max(abs(a), abs(b)), float.epsilon * 8), otherwise 0
sapz	r?	a(r?|num)	b(r?|num)	 
» Register = 1 if |a| <= float.epsilon * 8, otherwise 0
sdns	r?	d?	 	 
» Register = 1 if device is not set, otherwise 0
sdse	r?	d?	 	 
» Register = 1 if device is set, otherwise 0.
select	r?	a(r?|num)	b(r?|num)	c(r?|num)
» Register = b if a is non-zero, otherwise c
seq	r?	a(r?|num)	b(r?|num)	 
» Register = 1 if a == b, otherwise 0
seqz	r?	a(r?|num)	 	 
» Register = 1 if a == 0, otherwise 0
sge	r?	a(r?|num)	b(r?|num)	 
» Register = 1 if a >= b, otherwise 0
sgez	r?	a(r?|num)	 	 
» Register = 1 if a >= 0, otherwise 0
sgt	r?	a(r?|num)	b(r?|num)	 
» Register = 1 if a > b, otherwise 0
sgtz	r?	a(r?|num)	 	 
» Register = 1 if a > 0, otherwise 0
sle	r?	a(r?|num)	b(r?|num)	 
» Register = 1 if a <= b, otherwise 0
slez	r?	a(r?|num)	 	 
» Register = 1 if a <= 0, otherwise 0
slt	r?	a(r?|num)	b(r?|num)	 
» Register = 1 if a < b, otherwise 0
sltz	r?	a(r?|num)	 	 
» Register = 1 if a < 0, otherwise 0
sna	r?	a(r?|num)	b(r?|num)	c(r?|num)
» Register = 1 if abs(a - b) > max(c * max(abs(a), abs(b)), float.epsilon * 8), otherwise 0
snaz	r?	a(r?|num)	b(r?|num)	 
» Register = 1 if |a| > float.epsilon, otherwise 0
sne	r?	a(r?|num)	b(r?|num)	 
» Register = 1 if a != b, otherwise 0
snez	r?	a(r?|num)	 	 
» Register = 1 if a != 0, otherwise 0
Mathematical Operations
Instruction	Fields
abs	r?	a(r?|num)	 	 
» Register = the absolute value of a
acos	r?	a(r?|num)	 	 
» Returns the cosine of the specified angle
add	r?	a(r?|num)	b(r?|num)	 
» Register = a + b.
asin	r?	a(r?|num)	 	 
» Returns the angle whos sine is the specified value
atan	r?	a(r?|num)	 	 
» Returns the angle whos tan is the specified value
ceil	r?	a(r?|num)	 	 
» Register = smallest integer greater than a
cos	r?	a(r?|num)	 	 
» Returns the cosine of the specified angle
div	r?	a(r?|num)	b(r?|num)	 
» Register = a / b
exp	r?	a(r?|num)	 	 
» Register = exp(a)
floor	r?	a(r?|num)	 	 
» Register = largest integer less than a
log	r?	a(r?|num)	 	 
» Register = log(a)
max	r?	a(r?|num)	b(r?|num)	 
» Register = max of a or b
min	r?	a(r?|num)	b(r?|num)	 
» Register = min of a or b
mod	r?	a(r?|num)	b(r?|num)	 
» Register = a mod b (note: NOT a % b)
mul	r?	a(r?|num)	b(r?|num)	 
» Register = a * b
rand	r?	 	 	 
» Register = a random value x with 0 <= x < 1
round	r?	a(r?|num)	 	 
» Register = a rounded to nearest integer
sin	r?	a(r?|num)	 	 
» Returns the sine of the specified angle
sqrt	r?	a(r?|num)	 	 
» Register = square root of a
sub	r?	a(r?|num)	b(r?|num)	 
» Register = a - b.
tan	r?	a(r?|num)	 	 
» Returns the tan of the specified angle
trunc	r?	a(r?|num)	 	 
» Register = a with fractional part removed
Logic
Instruction	Fields
and	r?	a(r?|num)	b(r?|num)	 
» Register = 1 if a and b not zero, otherwise 0
nor	r?	a(r?|num)	b(r?|num)	 
» Register = 1 if a and b are 0, otherwise 0
or	r?	a(r?|num)	b(r?|num)	 
» Register = 1 if a and/or b not 0, otherwise 0
xor	r?	a(r?|num)	b(r?|num)	 
» Register = 1 if either a or b not 0, otherwise 0
Stack
Instruction	Fields
peek	r?	 	 	 
» Register = the value at the top of the stack
pop	r?	 	 	 
» Register = the value at the top of the stack and decrements sp
push	a(r?|num)	 	 	 
» Pushes the value of a to the stack at sp and increments sp
Misc
Instruction	Fields
alias	str	r?|d?	 	 
» Labels register or device reference with name, device references also affect what shows on the screws on the IC base.
define	str	num	 	 
» Creates a label that will be replaced throughout the program with the provided value.
hcf	 	 	 	 
» Halt and catch fire
move	r?	a(r?|num)	 	 
» Register = provided num or register value.
sleep	a(r?|num)	 	 	 
» Pauses execution on the IC for a seconds
yield	 	 	 	 
» Pauses execution for 1 tick



# Section B:
Jump Tags
To assist the creation of larger and more complex programs, the simulator can handle jump tags as supported by the IC. This means where you would put a jump destination you may put a tag.

For example:

start:
move o 1
yield
j start
Set/Unset Devices
You can toggle if the IC would see the emulated device as connected by toggling with the  icon.

Labelling Devices
To label a device (in simulator and in game), use the follow instruction:

alias GasSensor d1
Note Stationeering.com's labels will no longer work for devices, use the alias instruction.

Labelling Registers
To label a register, use the following instruction:

alias CurrentPressure r1

You will then be able to refer to the label name rather than the register in code.

Note The aliases will not appear on the registers until the instruction has been executed.

You can also still use Stationeering.com's legacy method of labeling registers.

#:internal:0:If it's safe?
Note These comments will still count as line numbers for the interpreter, so addresses for jumps needs to be adjusted.

# Section C:
Indirect referencing:  
This is a way of accessing a register by using another register as a pointer. Adding an additional r in front of the register turns on this behaviour. The value stored in the register being used as the pointer must be between 0 to 15, this will then point to a register from r0 to r15, higher or lower values will cause an error.

move r0 5 # stores the value 5 in r0
move rr0 10 
# is now the same as 'move r5 10' 
# since r0 has the value 5, rr0 points at the register r5

Additional r's can be added to do indirect referencing multiple times in a row.

move r1 2
move r2 3
move rrr1 4
# is now the same as 'move r3 4'
# since r1 points at r2 which points at r3

This also works with devices

move r0 2 # stores the value 2 in r0
s dr0 On 1 
# is now the same as 's d2 On 1'
# r0 has the value 2 so dr0 points at d2


Dynamically changing LogicType when interacting with Device Registers:  
When the l or s instructions are used to read from or write to a Device Register, the LogicType is the variable that will be interacted with (example: in l r0 myDevice Temperature the LogicType is the Temperature). In most scripts the LogicType will be hardcoded. But it's also possible to change the LogicType dynamically. The LogicType is an enum, where each type is identified by a unique integer value. This makes it possible to cycle through them in various ways, either as a list placed on the Stack, or by increasing the LogicType value via addition in each loop.

#loop through a list of LogicTypes
push LogicType.RatioOxygen
push LogicType.RatioVolatiles
push LogicType.Temperature
loop:
pop r1
l r0 myDevice r1
...
bgtz sp loop #loop until the Stack is empty


# Section D: (might be outdated)  
# Link:  
[stationeers-wiki.com/IC10](https://stationeers-wiki.com/IC10)


Unofficial Stationeers Wiki
Search
Navigation
Main page
Recent changes
Random page
Help
Toolbox
What links here
Related changes
Upload file
Special pages
Printable version
Permanent link
Page information
Log in
Unofficial Stationeers Wiki
Search
Navigation
Main page
Recent changes
Random page
Help
Toolbox
What links here
Related changes
Upload file
Special pages
Printable version
Permanent link
Page information
Actions
IC10
Contents
1 Scripting language for IC10 housings / chips
1.1 Registers
1.1.1 Logic and algorithmic with Internal registers
1.1.2 IO to Device registers
1.1.3 batch IO to - Device registers
1.1.4 Examples
1.1.5 Special registers
1.2 Stack Memory
1.3 Comments
1.4 Device Ports
1.5 Labels
1.6 Constants
1.7 Numeric values
1.8 Indirect referencing
1.9 Dynamically changing LogicType when interacting with Device Registers
1.10 Network Referencing / Channels
1.11 Logic gates, Bitwise and Logical
1.12 Debugging advices
1.13 Learning IC10
1.14 Accessing devices via batch or ReferenceId
1.14.1 Batch instructions
1.14.2 Direct reference instructions
2 Instructions
2.1 Utility
2.2 Mathematical
2.2.1 Mathematical / Trigonometric
2.3 Stack
2.4 Slot/Logic
2.4.1 Slot/Logic / Batched
2.5 Bitwise
2.6 Comparison
2.6.1 Comparison / Device Pin
2.6.2 Comparison / Value
2.7 Branching
2.7.1 Branching / Device Pin
2.7.2 Branching / Comparison
2.8 Conditional functions cheatsheet
2.9 Device Variables
2.10 Slot Variables
3 Examples
3.1 Harvie automation
3.2 Solar Panel 2-axis tracking
3.3 Example experiment: how many lines of code are executed each tick?
3.4 Push & Pop return address when calling multiple levels of functions
4 Links
5 Index
Scripting language for IC10 housings / chips
MIPS is Stationeers' inspiration for the in-game scripting language called IC10. It runs on IC10 chips crafted at the Electronics Printer.

Registers
Internal registers r?: The IC contains 16 CPU registers, numbered r0 to r15. From now on referred to as r?.

Device registers d? logicType: Device registers are written to and from the IC. A device register is numbered d0 to d5 (select via screw), or db (connected device). From now on referred to as d?.

Logic and algorithmic with Internal registers
All calculations are exclusively performed to and from r? registers, or generally more understood as variables in programming. You can use aliases to give convenient names with the alias string r?|d?command (see below).

Internal registers can be manipulated in various ways.

Write constant values move r? (r?|num): Example: move r0 2 sets r0 to the number 2.
Calculate: Calculations are done to- and from these registers, like add r? a(r?|num) b(r?|num). Example: add r1 r0 3 adds 3 to r0, and writes to r1.
Note, for any kind of if statements or loop behaviours, knowing about labels, branching, and jumps is essential knowledge. See below.

IO to Device registers
Acronym d? stands for device, where ? is a number corresponding to the screw device selector on the socket. You can also read/write to the device where the IC is planted in using device db.

Generally, there are up to 6 devices which can be set using the screwdriver d0 to d5. A special device register db is the device wherever the IC is mounted upon. Very convenient for atmospheric devices where no separate IC socket is required.

Note, the IC is completely unaware where d? is actually connected to. So if you get a logicType error, check d? number, or check if the screw has been set on the socket. An alias is only convenient to convey what is expected to be set on the d? screw, it does not actually set or program the screw.

Read from device (load) l r? d? logicType: Reads logicType, like Pressure from a gas sensor, from device d? to register r?. Values can be read from connected devices and put into the register using the l (load) command. For example, if you want to load the state of a door.
Example: l r0 Door Open reads the 'Open' field of an object named 'Door', that would be connected to the IC housing of the chip.
Write to a device (set) s d? logicType r?: Write a value from a register back to a device using the command s d? logicType r?. For example, if d0 is set to a door using the screwdriver, s d0 Open 0 sets the 'Open' status of the d0 (a door) to 0, effectively closing the door.
batch IO to - Device registers
Batch writing needs to be done to a specific deviceHash instead of d?. Is unique per device type, which you can find in the Stationpedia entries.

lb r? deviceHash logicType batchMode
sb deviceHash logicType r?
Additionally, using the following batch commands, a nameHash can be provided to only modify devices with a certain name.

lbn r? deviceHash nameHash logicType batchMode
sbn deviceHash nameHash logicType r?
All hashes used in the game are CRC-32 checksums computed from the strings they represent.

batchMode is a parameter equal to 0, 1, 2, or 3. These are also defined as the constants Average, Sum, Minimum, and Maximum respectively. The word or number can be used.

Combining one of these functions with the HASH() function can be advantageous:

lbn r0 HASH("StructureGasSensor") HASH("Sensor 1") Temperature Average

This code will load the average temperature of all gas sensors on the network named "Sensor 1" onto register r0

If the batch read (lb/lbn) is done on a network without any matching devices the results will be as specified in the table:

Batch read with no devices
Batch Mode	Result
Average (0)	nan
Sum (1)	0
Minimum (2)	inf
Maximum (3)	ninf
Examples
Here are some examples demonstrating all three operations:

move r0 10
Sets register r0 to the value 10

move r0 r1
Copies the value of register r1 to register r0

l r0 d0 Temperature
Reads the Temperature parameter from device d0 and places the value in register r0. Note: not all devices have a Temperature parameter, check the in-game stationpedia.

To set a device specific value (like On), you can write into this value.

s d0 On r0
Writes the value from register r0 out to On parameter of device d0. In this example the device will be turned On, if valve of register r0 equals 1, otherwise (register r0 equals 0) it will turned off. See section Device Variables.

It's recommended to use labels (like: someVariable) instead of a direct reference to the register. See alias in section Instructions.

Special registers
There are two more registers. One called ra (return address) and one called sp (stack pointer). The ra is used by certain jump and branching instructions (those ending with -al) to remember which line in the script it should return to. The sp tracks the next index within the stack (a memory that can store up to 512 values) to be pushed (written) to or popped (read) from. Neither ra or sp is protected, their values can be changed by instructions like any other register.

Stack Memory
The Stack is a memory that can hold 512 different values. Each IC10 has its own Stack, and some devices (like the Logical Sorter) also have a Stack.

push r?
adds the value r? and increments the sp by 1.
pop r?
loads the value in the stack memory at index sp-1 into register r? and decrements the sp by 1.
peek r?
loads the value in the stack memory at index sp-1 into register r?.
get r? d? address(r?|num)
loads the value in the stack memory at index address on provided device into register r?.
getd r? id(r?|num) address(r?|num)
loads the value in the stack memory at index address on provided device id into register r?.
put d? address(r?|num) value(r?|num)
adds the value to the stack memory off the provided device at index address.
putd id(r?|num) address(r?|num) value(r?|num)
adds the value to the stack memory off the provided device id at index address.
As mentioned previously, sp can be both written to and read from any time. When reading (peek or pop), sp must be between 1 and 512, inclusive. While writing (push), sp must be between 0 and 511, inclusive.

Stack memory is persistent on logic chips. This means that if you have a logic chip and push values to the stack, the code that pushes those values can be removed and the stack will retain those values.

Note that this does not carry over to any other logic chips which receive the program of the original; They will need to have their stack memories programmed individually.

Stack Traversing

Traversing the stack can be done similarly to how an array would be traversed in some other languages:

#this will traverse indices {min value} through {max value}-1
move sp {min value}
loop:
add sp sp 1
peek r0

#do something here with your stack values (loaded into r0)

blt sp {max value} loop

#continue on

Alternatively, you can use the pop function's decrementing to make a more efficient loop:

move sp {max value}
add sp sp 1
loop:
pop r0

#do something here with your stack values (loaded into r0)

bgt sp {min value} loop

#continue on


Comments
Comments can be placed using a # symbol. All comments are ignored by the game when it reads commands. Below is an example of valid code with two comments.

alias MyAlias r0 # Text after the hash tag will be ignored to the end of the line.
# You can also write comments on their own lines, like this.


Device Ports
ICs can interact with up to 6 other devices via d0 - d5, as well as the device it's attached to via db. To change or set a device, use a screwdriver and adjust the device in the IC housing. You can read or set any of the device's properties, so it is possible to do things like read the pressure or oxygen content of a room on the same Device port.

Additionally, is possible to set other IC housings as devices, allowing you to create programs that run across multiple ICs together. For example, an Gas Mixing IC could check the Setting field of a Atmosphere Sensor IC and act based on the value of the sensor chip.

The l (load) or s (set) instructions you have to read or set these values to your device. Examples:

#Reads the 'Temperature' from an atmosphere sensor
# at device port 'd0' into register 'r0'.
l r0 d0 Temperature


# Writes the value of the register 'r0' to the
# device on port 'd1' into the variable 'Setting'.
s d1 Setting r0


Labels
Labels are used to make it easier to jump between lines in the script. The label will have a numerical value that is the same as its line number. Even though it's possible to use a labels value for calculations, doing so is a bad idea since any changes to the code can change the line numbers of the labels.

main: # define a jump mark with label 'main'
j main # jumps back to 'main'


Constants
Instead of using a register to store a fixed value, a constant can be made. Using this name will refer to the assigned value. With the help of Constants you can save register places.

# defines a Constant with name 'pi'
# and set its value to 3.14159
define pi 3.14159

You can use these constants like any other variables (see: alias in section Instructions). Example:

# set the value of register 'r0' to the value of constant named 'pi'.
move r0 pi


Numeric values
Registers and constants are usually decimal values using double-precision floating point (confirmed?).

Unlike real CPU architectures, integers are not supported as a distinct type, but double FP can represent integers up to about 54 bits before rounding causes problems (the exact number depending what bit patterns you happen to have).

Numbers can be written in hexadecimal by preceding the value with a $ symbol. Values larger than 54 bits might get corrupted. Hex numbers are typically used for ReferenceId values.

Examples:

move r0 12345
move r1 123.456
move r2 $E1B2


Indirect referencing
This is a way of accessing a register by using another register as a pointer. Adding an additional r in front of the register turns on this behaviour. The value stored in the register being used as the pointer must be between 0 to 15, this will then point to a register from r0 to r15, higher or lower values will cause an error.

move r0 5 # stores the value 5 in r0
move rr0 10 
# is now the same as 'move r5 10' 
# since r0 has the value 5, rr0 points at the register r5

Additional r's can be added to do indirect referencing multiple times in a row.

move r1 2
move r2 3
move rrr1 4
# is now the same as 'move r3 4'
# since r1 points at r2 which points at r3

This also works with devices

move r0 2 # stores the value 2 in r0
s dr0 On 1 
# is now the same as 's d2 On 1'
# r0 has the value 2 so dr0 points at d2


Dynamically changing LogicType when interacting with Device Registers
When the l or s instructions are used to read from or write to a Device Register, the LogicType is the variable that will be interacted with (example: in l r0 myDevice Temperature the LogicType is the Temperature). In most scripts the LogicType will be hardcoded. But it's also possible to change the LogicType dynamically. The LogicType is an enum, where each type is identified by a unique integer value. This makes it possible to cycle through them in various ways, either as a list placed on the Stack, or by increasing the LogicType value via addition in each loop.

#loop through a list of LogicTypes
push LogicType.RatioOxygen
push LogicType.RatioVolatiles
push LogicType.Temperature
loop:
pop r1
l r0 myDevice r1
...
bgtz sp loop #loop until the Stack is empty


Network Referencing / Channels
All cable networks have 8 Channels which can have data loaded from/stored to via a device and connection reference. Connections for each supported device are listed in the stationpedia. All 'connections' a device can make are a connection (pipe, chute, cable), but only cable networks have channels.

The 8 channels (Channel0 to Channel7) are however volatile, in that data is destroyed if any part of the cable network is changed, removed, or added to, and also whenever the world is exited. All these channels default to NaN. Strictly speaking, they default to what we would call "quiet NaN", in that its not an error it simply means its not a number yet. Recommend you use these channels for reading and writing between networks, rather than as a data store. This effectively means an IC can read all the networks for all devices to connected to it, so not just their own local network, but any networks any device they can reference is connected to.

# d0 is device zero, and the :0 refers
# to that device's 0 connection
l r0 d0:0 Channel0

For example: on an IC Housing, the 0 connection is the data port and 1 is power, so you could write out r0 to Channel0 of the power network of the Housing using s db:1 Channel0 r0

#read all 8 channels with a loop and
#place the values in r0 to r7
move r15 LogicType.Channel0 #LogicType integer
move r14 0 #pointer for indirect referencing
loop:
l rr14 db:0 r15
add r15 r15 1 #next channel
add r14 r14 1 #next register
ble r15 LogicType.Channel7 loop


Logic gates, Bitwise and Logical
All logic gates in MIPS have a bitwise behavior. The available gates are NOT, AND, OR, XOR and NOR (XNOR and NAND are missing).

In Bitwise operations, each bit is matched separately, which includes the sign bit.

To understand what is going on with bitwise operations, a little bit of computer theory is needed. In Stationeers each register uses 64 bits for integer values (a number without decimals), where the 64th bit is the sign-bit (0 for positive and 1 for negative). Since the number 0 is counted as a positive value, this gives each register a range of (2^63 - 1) to -2^63. Negative numbers also behave according to Two's complement (https://en.wikipedia.org/wiki/Two%27s_complement). Which means that a number with a sign-bit of 1 will have all of its number bits flipped as well, so that the decimal value of -1 is represented by a binary value of sixtyfour 1's (this is the smallest possible negative integer since zero counts as a positive integer).

MIPS have binary notation (https://en.wikipedia.org/wiki/Binary_number) that is activated by placing a % in front of the number. The _ characters are ignored and only used for readability.

not r0 0
# 0 = %00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000
# flip all bits
# -1 = %11111111_11111111_11111111_11111111_11111111_11111111_11111111_11111111
# r0 equals -1

and r0 3 6
# 3 = %011
# 6 = %110
# only the bits in the "2" position are matching
# r0 equals %010 = 2

Logical operations can still be performed via alternative instructions. But these are not perfect substitutes, they treat negative values differently, and some can produce non-binary outputs. When using these, keep in mind that devices that wants a binary value will treat any non-binary values like this: >= 1 counts as "1" and <1 counts as "0".

# r0 = result
# r1 = input A
# r2 = input B

Logical NOT = seqz r0 r1
Logical AND = min r0 r1 r2
Logical OR = max r0 r1 r2

Logical XOR = sne r0 r1 r2 (only for binary inputs)
Logical NAND = not and
Logical NOR = not or
Logical XNOR = not xor

Debugging advices
The value stored in a register or variable can easily be displayed by writing it to the Setting parameter of the IC housing. This has no side effects. To see the value, just stand close to the IC housing and look directly at the housing.
s db Setting r0. # sets/writes the value of register r0 into the parameter Setting of the IC Housing(db)

To check if a certain block of code is executed, use the above trick but with a random number that you choose, like the line number.
This example will display the number 137 on the IC housing.
s db Setting 137 # sets/writes the number 137 into the parameter Setting of the IC Housing(db)

Always use unique names for labels. When a label is named after a IC10 keyword like "Temperature:" or "Setting:" the original meaning of the keyword is overwritten, so when an instruction tries to use it an error will occur.

A configuration cartridge installed in a tablet can be used to see all available values and configuration parameter for all devices you focus on.

Learning IC10
IC10 can be difficult to get started with. So here is a list of instructions that are useful for beginners. These can be used to write many different scripts.

General:

alias make the script easier to read by assigning a name to a register or device, example: alias rTemperature r15
label: where "label" can be replaced with almost any word, jump and branch instructions can use these in place of line numbers, example: start:
yield pause for 1-tick and then resume, if not used the script will automatically pause for 1-tick after 128 lines


Jumps:

j someLabelName jump to line with someLabelName
jal someLabelName stores the next line number into the register ra (return address) and then jump to someLabelName
j ra jump to register ra (return address)


Branching (jump-if):
beq a(r?|num) b(r?|num) c(r?|num) if a is equal to b goto c (label or linenumber)
bne a(r?|num) b(r?|num) c(r?|num) if a not-equal b goto c (label or linenumber)
bgt a(r?|num) b(r?|num) c(r?|num) if a greater than b goto c (label or linenumber)
blt a(r?|num) b(r?|num) c(r?|num) if a less than b goto c (label or linenumber)
The suffix -al can be added to each of these (example: beqal) to save the next line number into the "return address" register. this is called using j ra


Device interactions:

l (load)
lb (load batch, requires one of the following: 0(Average) / 1(Sum) / 2(Minimum) / 3(Maximum))
ls (load slot)
s (store)
sb (store batch)

Logic and Math:

seqz (common NOT-gate: turns 0 into 1, and all other values into 0)
move
add (addition)
sub (subtraction)
mul (multiplication)
div (division)

Common device variables:

On (1 is on, 0 is off)
Open (1 is open, 0 is closed)
Setting (meaning varies between devices, example: a LED display(console) will show this value)
Activate (1 usually means running, example: a Daylight sensor is 1 when the sun shines on it)
Temperature (in Kelvin, Celsius - 273.15)
Pressure (in kPa)

Notes:
-All instructions and variables can be seen in-game in the IC editor window by clicking the "f", "x" and "s(x)" buttons on the top right.
-The stationpedia is the best source to see which variables are available to each device.
-Most scripts are loops, they end with a jump instruction that leads back up to the start. Otherwise they will just run once and then stop.

Two practice scripts:
Automatic Night Light: Load "Activate" from a Daylight sensor, flip the value with a NOT-gate, store the value to the "On" variable of one or more lights.
Automatic Wall Cooler: Read "Temperature" from a Gas Sensor. Branch if the value is greater than X, turn on the cooler. Branch if the value is less than Y, turn off the cooler. (Wall coolers need a minimum of 12.5 kPa pressure in the connected pipe)



Accessing devices via batch or ReferenceId
The IC housing has 6 pins you can use to configure the devices it uses. This provides flexibility to let the installer configure which devices will be controlled by the IC.

Alternatives for accessing devices include the batch load/store and the ReferenceId load/store instructions.



# get the average charge ratio across station batteries
lb r0 HASH("StructureBattery") Ratio Average




# get the ReferenceId for the sorter named "Sorter Corn"
lbn r1 HASH("StructureLogicSorter") HASH("Sorter Corn") ReferenceId Maximum
ble r1 ninf ra
#use the ReferenceId to set that sorter's mode.
sd r1 Mode 1

Using the 6 configuration pins makes it easy to write reusable MIPS scripts where the installer uses the pins to select the devices that will be managed.

Using batch-name instructions frees you from the hassle of adjusting the pins, but requires you to name the devices via the Labeller. It can also allow you to control more than 6 devices.

Batch instructions
The batch instructions can address multiple devices only via their PrefabHash generated from the prefab name using the `HASH("Name")` macro or copied directly from the Stationpedia. A prefab hash is always an integer. All devices that can be read with logic contain the logic value PrefabHash and NameHash.

See Batched instructions for a comprehensive list of all batch instructions.

sb, sbn, sbs, (no sbns)
lb, lbs, lbn, lbns

Direct reference instructions
Direct reference instructions can address a specific device via its ReferenceId.

clrd, getd, putd,
ld, sd, (no slot access via reference ID)

Instructions
See IC10/instructions



Utility
§
alias str r?|d? 
Labels register or device reference with name, device references also affect what shows on the screws on the IC base.

Example:

alias dAutoHydro1 d0
alias vTemperature r0


§
define str num 
Creates a label that will be replaced throughout the program with the provided value.

Example:

define ultimateAnswer 42
move r0 ultimateAnswer # Store 42 in register 0


§
hcf 
Halt and catch fire





§
sleep a(r?|num) 
Pauses execution on the IC for a seconds





§
yield 
Pauses execution for 1 tick





Mathematical
§
abs r? a(r?|num) 
Register = the absolute value of a

Example:

define negativeNumber -10
abs r0 negativeNumber # Compute the absolute value of -10 and store it in register 0


§
add r? a(r?|num) b(r?|num) 
Register = a + b.

Example:

add r0 r0 1 # increment r0 by one


define num1 10
define num2 20
add r0 num1 num2 # Add 10 and 20 and store the result in register 0


§
ceil r? a(r?|num) 
Register = smallest integer greater than a

Example:

define floatNumber 10.3
ceil r0 floatNumber # Compute the ceiling of 10.3 and store it in register 0


§
div r? a(r?|num) b(r?|num) 
Register = a / b





§
exp r? a(r?|num) 
exp(a) or e^a





§
floor r? a(r?|num) 
Register = largest integer less than a





§
log r? a(r?|num) 
base e log(a) or ln(a)





§
max r? a(r?|num) b(r?|num) 
Register = max of a or b





§
min r? a(r?|num) b(r?|num) 
Register = min of a or b





§
mod r? a(r?|num) b(r?|num) 
Register = a mod b (note: NOT a % b)

Example:



§
move r? a(r?|num) 
Register = provided num or register value.

Example:

move r0 42 # Store 42 in register 0


§
mul r? a(r?|num) b(r?|num) 
Register = a * b





§
rand r? 
Register = a random value x with 0 <= x < 1





§
round r? a(r?|num) 
Register = a rounded to nearest integer





§
sqrt r? a(r?|num) 
Register = square root of a





§
sub r? a(r?|num) b(r?|num) 
Register = a - b.





§
trunc r? a(r?|num) 
Register = a with fractional part removed





Mathematical / Trigonometric
§
acos r? a(r?|num) 
Returns the angle (radians) whos cos is the specified value





§
asin r? a(r?|num) 
Returns the angle (radians) whos sine is the specified value





§
atan r? a(r?|num) 
Returns the angle (radians) whos tan is the specified value





§
atan2 r? a(r?|num) b(r?|num) 
Returns the angle (radians) whose tangent is the quotient of two specified values: a (y) and b (x)





§
cos r? a(r?|num) 
Returns the cosine of the specified angle (radians)





§
sin r? a(r?|num) 
Returns the sine of the specified angle (radians)





§
tan r? a(r?|num) 
Returns the tan of the specified angle (radians)





Stack
§
clr d? 
Clears the stack memory for the provided device.





§
clrd id(r?|num) 
Seeks directly for the provided device id and clears the stack memory of that device





§
get r? d? address(r?|num) 
Using the provided device, attempts to read the stack value at the provided address, and places it in the register.





§
getd r? id(r?|num) address(r?|num) 
Seeks directly for the provided device id, attempts to read the stack value at the provided address, and places it in the register.





§
peek r? 
Register = the value at the top of the stack





§
poke address(r?|num) value(r?|num) 
Stores the provided value at the provided address in the stack.





§
pop r? 
Register = the value at the top of the stack and decrements sp





§
push a(r?|num) 
Pushes the value of a to the stack at sp and increments sp





§
put d? address(r?|num) value(r?|num) 
Using the provided device, attempts to write the provided value to the stack at the provided address.





§
putd id(r?|num) address(r?|num) value(r?|num) 
Seeks directly for the provided device id, attempts to write the provided value to the stack at the provided address.





Slot/Logic
§
l r? d? logicType 
Loads device LogicType to register by housing index value.

Example:
Read from the device on d0 into register 0

l r0 d0 Setting
Read the pressure from a sensor

l r1 d5 Pressure
This also works with aliases. For example:

alias Sensor d0
l r0 Sensor Temperature


§
ld r? id(r?|num) logicType 
Loads device LogicType to register by direct ID reference.





§
lr r? d? reagentMode int 
Loads reagent of device's ReagentMode where a hash of the reagent type to check for. ReagentMode can be either Contents (0), Required (1), Recipe (2). Can use either the word, or the number.





§
ls r? d? slotIndex logicSlotType 
Loads slot LogicSlotType on device to register.

Example:
Read from the second slot of device on d0, stores 1 in r0 if it's occupied, 0 otherwise.

ls r0 d0 2 Occupied
And here is the code to read the charge of an AIMeE:

alias robot d0
alias charge r10
ls charge robot 0 Charge


§
s d? logicType r? 
Stores register value to LogicType on device by housing index value.

Example:

s d0 Setting r0


§
sd id(r?|num) logicType r? 
Stores register value to LogicType on device by direct ID reference.





§
ss d? slotIndex logicSlotType r? 
Stores register value to device stored in a slot LogicSlotType on device.





§
rmap r? d? reagentHash(r?|num) 
Given a reagent hash, store the corresponding prefab hash that the device expects to fulfill the reagent requirement. For example, on an autolathe, the hash for Iron will store the hash for ItemIronIngot.





Slot/Logic / Batched
§
lb r? deviceHash logicType batchMode 
Loads LogicType from all output network devices with provided type hash using the provide batch mode. Average (0), Sum (1), Minimum (2), Maximum (3). Can use either the word, or the number.

Example:

lb r0 HASH("StructureWallLight") On Sum


§
lbn r? deviceHash nameHash logicType batchMode 
Loads LogicType from all output network devices with provided type and name hashes using the provide batch mode. Average (0), Sum (1), Minimum (2), Maximum (3). Can use either the word, or the number.





§
lbns r? deviceHash nameHash slotIndex logicSlotType batchMode 
Loads LogicSlotType from slotIndex from all output network devices with provided type and name hashes using the provide batch mode. Average (0), Sum (1), Minimum (2), Maximum (3). Can use either the word, or the number.





§
lbs r? deviceHash slotIndex logicSlotType batchMode 
Loads LogicSlotType from slotIndex from all output network devices with provided type hash using the provide batch mode. Average (0), Sum (1), Minimum (2), Maximum (3). Can use either the word, or the number.





§
sb deviceHash logicType r? 
Stores register value to LogicType on all output network devices with provided type hash.

Example:

sb HASH("StructureWallLight") On 1


§
sbn deviceHash nameHash logicType r? 
Stores register value to LogicType on all output network devices with provided type hash and name.





§
sbs deviceHash slotIndex logicSlotType r? 
Stores register value to LogicSlotType on all output network devices with provided type hash in the provided slot.





Bitwise
§
and r? a(r?|num) b(r?|num) 
Performs a bitwise logical AND operation on the binary representation of two values. Each bit of the result is determined by evaluating the corresponding bits of the input values. If both bits are 1, the resulting bit is set to 1. Otherwise the resulting bit is set to 0.





§
nor r? a(r?|num) b(r?|num) 
Performs a bitwise logical NOR (NOT OR) operation on the binary representation of two values. Each bit of the result is determined by evaluating the corresponding bits of the input values. If both bits are 0, the resulting bit is set to 1. Otherwise, if at least one bit is 1, the resulting bit is set to 0.





§
not r? a(r?|num) 
Performs a bitwise logical NOT operation flipping each bit of the input value, resulting in a binary complement. If a bit is 1, it becomes 0, and if a bit is 0, it becomes 1.

Note:
This is a bitwise operation, the NOT of 1 => -2, etc. You may want to use seqz instead



§
or r? a(r?|num) b(r?|num) 
Performs a bitwise logical OR operation on the binary representation of two values. Each bit of the result is determined by evaluating the corresponding bits of the input values. If either bit is 1, the resulting bit is set to 1. If both bits are 0, the resulting bit is set to 0.





§
sla r? a(r?|num) b(r?|num) 
Performs a bitwise arithmetic left shift operation on the binary representation of a value. It shifts the bits to the left and fills the vacated rightmost bits with zeros (note that this is indistinguishable from 'sll').





§
sll r? a(r?|num) b(r?|num) 
Performs a bitwise logical left shift operation on the binary representation of a value. It shifts the bits to the left and fills the vacated rightmost bits with zeros.





§
sra r? a(r?|num) b(r?|num) 
Performs a bitwise arithmetic right shift operation on the binary representation of a value. It shifts the bits to the right and fills the vacated leftmost bits with a copy of the sign bit (the most significant bit).





§
srl r? a(r?|num) b(r?|num) 
Performs a bitwise logical right shift operation on the binary representation of a value. It shifts the bits to the right and fills the vacated leftmost bits with zeros





§
xor r? a(r?|num) b(r?|num) 
Performs a bitwise logical XOR (exclusive OR) operation on the binary representation of two values. Each bit of the result is determined by evaluating the corresponding bits of the input values. If the bits are different (one bit is 0 and the other is 1), the resulting bit is set to 1. If the bits are the same (both 0 or both 1), the resulting bit is set to 0.





Comparison
§
select r? a(r?|num) b(r?|num) c(r?|num) 
Register = b if a is non-zero, otherwise c

Note:
This operation can be used as a simple ternary condition

Example:
1)
move r0 0
select r1 r0 10 200

move r0 0
select r1 r0 10 200

after run, r1 = 200

2)
move r0 5
select r1 r0 10 200

move r0 1
select r1 r0 10 100

after run, r1 = 10


Comparison / Device Pin
§
sdns r? d? 
Register = 1 if device is not set, otherwise 0





§
sdse r? d? 
Register = 1 if device is set, otherwise 0.





Comparison / Value
§
sap r? a(r?|num) b(r?|num) c(r?|num) 
Register = 1 if abs(a - b) <= max(c * max(abs(a), abs(b)), float.epsilon * 8), otherwise 0





§
sapz r? a(r?|num) b(r?|num) 
Register = 1 if abs(a) <= max(b * abs(a), float.epsilon * 8), otherwise 0





§
seq r? a(r?|num) b(r?|num) 
Register = 1 if a == b, otherwise 0





§
seqz r? a(r?|num) 
Register = 1 if a == 0, otherwise 0





§
sge r? a(r?|num) b(r?|num) 
Register = 1 if a >= b, otherwise 0





§
sgez r? a(r?|num) 
Register = 1 if a >= 0, otherwise 0





§
sgt r? a(r?|num) b(r?|num) 
Register = 1 if a > b, otherwise 0





§
sgtz r? a(r?|num) 
Register = 1 if a > 0, otherwise 0





§
sle r? a(r?|num) b(r?|num) 
Register = 1 if a <= b, otherwise 0





§
slez r? a(r?|num) 
Register = 1 if a <= 0, otherwise 0





§
slt r? a(r?|num) b(r?|num) 
Register = 1 if a < b, otherwise 0





§
sltz r? a(r?|num) 
Register = 1 if a < 0, otherwise 0





§
sna r? a(r?|num) b(r?|num) c(r?|num) 
Register = 1 if abs(a - b) > max(c * max(abs(a), abs(b)), float.epsilon * 8), otherwise 0





§
snan r? a(r?|num) 
Register = 1 if a is NaN, otherwise 0





§
snanz r? a(r?|num) 
Register = 0 if a is NaN, otherwise 1





§
snaz r? a(r?|num) b(r?|num) 
Register = 1 if abs(a) > max(b * abs(a), float.epsilon), otherwise 0





§
sne r? a(r?|num) b(r?|num) 
Register = 1 if a != b, otherwise 0





§
snez r? a(r?|num) 
Register = 1 if a != 0, otherwise 0





Branching
§
j int 
Jump execution to line a

Example:

j 0 # jump line 0


j label # jump to a label

label:
# your code here


§
jal int 
Jump execution to line a and store next line number in ra

Example:
jal provides a way to do function calls in IC10 mips

move r0 1000
move r1 0
start:
jal average
s db Setting r0
yield
j start

average:
add r0 r0 r1
div r0 r0 2
j ra # jump back


§
jr int 
Relative jump to line a





Branching / Device Pin
§
bdns d? a(r?|num) 
Branch to line a if device d isn't set





§
bdnsal d? a(r?|num) 
Jump execution to line a and store next line number if device is not set





§
bdse d? a(r?|num) 
Branch to line a if device d is set





§
bdseal d? a(r?|num) 
Jump execution to line a and store next line number if device is set

Example:

#Store line number and jump to line 32 if d0 is assigned.
bdseal d0 32


#Store line in ra and jump to label HarvestCrop if device d0 is assigned.
bdseal d0 HarvestCrop


§
brdns d? a(r?|num) 
Relative branch to line a if device is not set





§
brdse d? a(r?|num) 
Relative branch to line a if device is set





Branching / Comparison
§
bap a(r?|num) b(r?|num) c(r?|num) d(r?|num) 
Branch to line d if abs(a - b) <= max(c * max(abs(a), abs(b)), float.epsilon * 8)





§
brap a(r?|num) b(r?|num) c(r?|num) d(r?|num) 
Relative branch to line d if abs(a - b) <= max(c * max(abs(a), abs(b)), float.epsilon * 8)





§
bapal a(r?|num) b(r?|num) c(r?|num) d(r?|num) 
Branch to line c if a != b and store next line number in ra





§
bapz a(r?|num) b(r?|num) c(r?|num) 
Branch to line c if abs(a) <= max(b * abs(a), float.epsilon * 8)





§
brapz a(r?|num) b(r?|num) c(r?|num) 
Relative branch to line c if abs(a) <= max(b * abs(a), float.epsilon * 8)





§
bapzal a(r?|num) b(r?|num) c(r?|num) 
Branch to line c if abs(a) <= max(b * abs(a), float.epsilon * 8) and store next line number in ra





§
beq a(r?|num) b(r?|num) c(r?|num) 
Branch to line c if a == b





§
breq a(r?|num) b(r?|num) c(r?|num) 
Relative branch to line c if a == b





§
beqal a(r?|num) b(r?|num) c(r?|num) 
Branch to line c if a == b and store next line number in ra





§
beqz a(r?|num) b(r?|num) 
Branch to line b if a == 0





§
breqz a(r?|num) b(r?|num) 
Relative branch to line b if a == 0





§
beqzal a(r?|num) b(r?|num) 
Branch to line b if a == 0 and store next line number in ra





§
bge a(r?|num) b(r?|num) c(r?|num) 
Branch to line c if a >= b





§
brge a(r?|num) b(r?|num) c(r?|num) 
Relative branch to line c if a >= b





§
bgeal a(r?|num) b(r?|num) c(r?|num) 
Branch to line c if a >= b and store next line number in ra





§
bgez a(r?|num) b(r?|num) 
Branch to line b if a >= 0





§
brgez a(r?|num) b(r?|num) 
Relative branch to line b if a >= 0





§
bgezal a(r?|num) b(r?|num) 
Branch to line b if a >= 0 and store next line number in ra





§
bgt a(r?|num) b(r?|num) c(r?|num) 
Branch to line c if a > b

Example:
An example of a Schmitt trigger, turning on a device if the temperature is too low, and turning it off if it's too high and finally doing nothing if the temperature is within the desired range.

alias sensor d0
alias device d1

define mintemp 293.15
define maxtemp 298.15

start:
yield
l r0 sensor Temperature
# If the temperature < mintemp, turn on the device
blt r0 mintemp turnOn
# If the temperature > maxtemp, turn off the device
bgt r0 maxtemp turnOff
j start

turnOn:
s device On 1
j start
turnOff:
s device On 0
j start


§
brgt a(r?|num) b(r?|num) c(r?|num) 
relative branch to line c if a > b





§
bgtal a(r?|num) b(r?|num) c(r?|num) 
Branch to line c if a > b and store next line number in ra





§
bgtz a(r?|num) b(r?|num) 
Branch to line b if a > 0





§
brgtz a(r?|num) b(r?|num) 
Relative branch to line b if a > 0





§
bgtzal a(r?|num) b(r?|num) 
Branch to line b if a > 0 and store next line number in ra





§
ble a(r?|num) b(r?|num) c(r?|num) 
Branch to line c if a <= b





§
brle a(r?|num) b(r?|num) c(r?|num) 
Relative branch to line c if a <= b





§
bleal a(r?|num) b(r?|num) c(r?|num) 
Branch to line c if a <= b and store next line number in ra





§
blez a(r?|num) b(r?|num) 
Branch to line b if a <= 0





§
brlez a(r?|num) b(r?|num) 
Relative branch to line b if a <= 0





§
blezal a(r?|num) b(r?|num) 
Branch to line b if a <= 0 and store next line number in ra





§
blt a(r?|num) b(r?|num) c(r?|num) 
Branch to line c if a < b

Example:
An example of a Schmitt trigger, turning on a device if the temperature is too low, and turning it off if it's too high and finally doing nothing if the temperature is within the desired range.

alias sensor d0
alias device d1

define mintemp 293.15
define maxtemp 298.15

start:
yield
l r0 sensor Temperature
# If the temperature < mintemp, turn on the device
blt r0 mintemp turnOn
# If the temperature > maxtemp, turn off the device
bgt r0 maxtemp turnOff
j start

turnOn:
s device On 1
j start
turnOff:
s device On 0
j start


§
brlt a(r?|num) b(r?|num) c(r?|num) 
Relative branch to line c if a < b





§
bltal a(r?|num) b(r?|num) c(r?|num) 
Branch to line c if a < b and store next line number in ra





§
bltz a(r?|num) b(r?|num) 
Branch to line b if a < 0





§
brltz a(r?|num) b(r?|num) 
Relative branch to line b if a < 0





§
bltzal a(r?|num) b(r?|num) 
Branch to line b if a < 0 and store next line number in ra





§
bna a(r?|num) b(r?|num) c(r?|num) d(r?|num) 
Branch to line d if abs(a - b) > max(c * max(abs(a), abs(b)), float.epsilon * 8)





§
brna a(r?|num) b(r?|num) c(r?|num) d(r?|num) 
Relative branch to line d if abs(a - b) > max(c * max(abs(a), abs(b)), float.epsilon * 8)





§
bnaal a(r?|num) b(r?|num) c(r?|num) d(r?|num) 
Branch to line d if abs(a - b) <= max(c * max(abs(a), abs(b)), float.epsilon * 8) and store next line number in ra





§
bnan a(r?|num) b(r?|num) 
Branch to line b if a is not a number (NaN)





§
brnan a(r?|num) b(r?|num) 
Relative branch to line b if a is not a number (NaN)





§
bnaz a(r?|num) b(r?|num) c(r?|num) 
Branch to line c if abs(a) > max (b * abs(a), float.epsilon * 8)





§
brnaz a(r?|num) b(r?|num) c(r?|num) 
Relative branch to line c if abs(a) > max(b * abs(a), float.epsilon * 8)





§
bnazal a(r?|num) b(r?|num) c(r?|num) 
Branch to line c if abs(a) > max (b * abs(a), float.epsilon * 8) and store next line number in ra





§
bne a(r?|num) b(r?|num) c(r?|num) 
Branch to line c if a != b





§
brne a(r?|num) b(r?|num) c(r?|num) 
Relative branch to line c if a != b





§
bneal a(r?|num) b(r?|num) c(r?|num) 
Branch to line c if a != b and store next line number in ra





§
bnez a(r?|num) b(r?|num) 
branch to line b if a != 0





§
brnez a(r?|num) b(r?|num) 
Relative branch to line b if a != 0





§
bnezal a(r?|num) b(r?|num) 
Branch to line b if a != 0 and store next line number in ra




Other examples

Conditional functions cheatsheet
suffix	description	branch to line	branch and store return address	relative jump to line	set register
prefix:		b-	b-al	br-	s-
unconditional	j	jal	jr	
-eq	if a == b	beq	beqal	breq	seq
-eqz	if a == 0	beqz	beqzal	breqz	seqz
-ge	if a >= b	bge	bgeal	brge	sge
-gez	if a >= 0	bgez	bgezal	brgez	sgez
-gt	if a > b	bgt	bgtal	brgt	sgt
-gtz	if a > 0	bgtz	bgtzal	brgtz	sgtz
-le	if a <= b	ble	bleal	brle	sle
-lez	if a <= 0	blez	blezal	brlez	slez
-lt	if a < b	blt	bltal	brlt	slt
-ltz	if a < 0	bltz	bltzal	brltz	sltz
-ne	if a != b	bne	bneal	brne	sne
-nez	if a != 0	bnez	bnezal	brnez	snez
-nan	if a == NaN	bnan		brnan	snan
-nanz	if a != NaN				snanz
-dns	if device d is not set	bdns	bdnsal	brdns	sdns
-dse	if device d is set	bdse	bdseal	brdse	sdse
-ap	if a approximately equals b	bap	bapal	brap	sap
-apz	if a approximately equals 0	bapz	bapzal	brapz	sapz
-na	if a not approximately equals b	bna	bnaal	brna	sna
-naz	if a not approximately equals 0	bnaz	bnazal	brnaz	snaz
All b- commands require target line as last argument, all s- commands require register to store result as first argument. All br- commands require number to jump relatively as last argument. e.g. breq a b 3 means if a=b then jump to 3 lines after.

All approximate functions require additional argument denoting how close two numbers need to be considered equal. E.g.: sap r0 100 101 0.01 will consider 100 and 101 almost equal (not more than 1%=0.01 different) and will set r0 to 1. The exact formula is if abs(a - b) <= max(c * max(abs(a), abs(b)), float.epsilon * 8) for -ap and is similar for other approximate functions.

https://en.wikipedia.org/wiki/Machine_epsilon
Example:

 FLT_EPSILON = 2^(−23) ≈ 1.19e−07;        float (32 bit)
 DBL_EPSILON = 2^(−52) ≈ 2.20e−16;        double (64 bit)


 if abs(100 - 101) <= max(0.01 * max(abs(100), abs(101)), float.epsilon * 8)
 if abs(-1) <= max(0.01 * 101, float.epsilon * 8)
 if 1 <= max(0.01 * 101, float.epsilon * 8)


 if 1 <= max(1.01, FLT_EPSILON * 8)
 if 1 <= max(1.01, DBL_EPSILON * 8)


 if 1 <= max(1.01, 1.19e−07 * 8)
 if 1 <= max(1.01, 2.20e−16 * 8)


 if 1 <= max(1.01, 0.000000952)
 if 1 <= max(1.01, 0.00000000000000176)


 if 1 <= 1.01   TRUE   1
 if 1 <= 1.01   TRUE   1
Device Variables
Activate
1 if device is activated (usually means running), otherwise 0
l r0 d0 Activate # sets r0 to 1 if on or 0 if off
AirRelease
Charge
The current charge the device has.
ClearMemory
When set to 1, clears the counter memory (e.g. ExportCount). Will set itself back to 0 when triggered.
Color
 0 (or lower) = Blue
 1 = Grey
 2 = Green
 3 = Orange
 4 = Red
 5 = Yellow
 6 = White
 7 = Black
 8 = Brown
 9 = Khaki
 10 = Pink
 11 (or higher) = Purple
CompletionRatio
ElevatorLevel
ElevatorSpeed
Error
1 if device is in error state, otherwise 0
ExportCount
How many items exporfted since last ClearMemory.
Filtration
The current state of the filtration system. For example filtration = 1 for a Hardsuit when filtration is On.
Harvest
Performs the harvesting action for any plant based machinery.
s d0 Harvest 1 # Performs 1 harvest action on device d0
Horizontal
HorizontalRatio
Idle
ImportCount
Lock
Maximum
Mode
On
Open
Output
Plant
Performs the planting operation for any plant based machinery.
s d0 Plant 1 # Plants one crop in device d0
PositionX
PositionY
PositionZ
Power
PowerActual
PowerPotential
PowerRequired
Pressure
PressureExternal
PressureInteral
PressureSetting
Quantity
Total quantity in the device.
Ratio
Context specific value depending on device, 0 to 1 based ratio.
RatioCarbonDioxide
RatioNitrogen
The ratio of nitrogen in device atmosphere.
RatioOxygen
The ratio of oxygen in device atmosphere.
RatioPollutant
The ratio of pollutant in device atmosphere.
RatioVolatiles
The ratio of volatiles in device atmosphere.
RatioWater
The ratio of water in device atmosphere.
Reagents
RecipeHash
ReferenceId
Unique Identifier of a Device, this value is different for every device in a save.
RequestHash
RequiredPower
Setting
SolarAngle
Solar angle of the device.
l r0 d0 SolarAngle # Sets r0 to the solar angle of d0.
Temperature
TemperatureSettings
TotalMoles
VelocityMagnitude
VelocityRelativeX
VelocityRelativeY
VelocityRelativeZ
Vertical
Vertical setting of the device.
VerticalRatio
Ratio of vertical setting for device.
Volume
Returns the device atmosphere volume
Slot Variables
In general (exceptions exist such as filtration units) slots are assigned as follows.

Slot 0: Import
Slot 1: Export
Slot 2: Inside Machine


Occupied
ls r0 d0 2 Occupied #Stores 1 in r0 if d0 has more seeds
ls vOccupied dThisVictim 2 Occupied #stores 1 in vOccupied if dThisVictim has more seeds
OccupantHash
Quantity
Damage
Efficiency
Health
Growth
ls r0 d0 0 Growth # Store the numerical growth stage of d0 in r0
Pressure
Temperature
Charge
ChargeRatio
Class
PressureWaste
PressureAir
MaxQuantity
Mature
ls r0 d0 0 Mature # Store 1 in r0 if d0 has a mature crop
ls vMature dThisVictim 0 Mature # Store 1 in vMature if dThisVictim has a mature crop
ReferenceId
Unique Identifier of a Device, this value is different for every device in a save.
Examples
Previous examples were obsolete due to game changes, or confusing, they have been moved into the Discussions section

Harvie automation
This script uses the batch instruction sb ... to control all Harvie devices on the network. But only one Harvie and one Tray will be the master and have their values read, the rest of the Harvies will repeat exactly what this unit does. Some problems with this design is that different types of crops mature at different speeds, and if seeds were manually planted and the master unit recieved the first seed, the harvesting action will be performed too early on all the other plants since they are growing a few seconds slower.



Solar Panel 2-axis tracking


Example experiment: how many lines of code are executed each tick?
To determine this, a script without yield will be used. It should have as few lines as possible (so no labels are used, but a reset value at the top will be needed) and count the number of lines, the IC Housing will be used to display the result.

move r0 1   #the first line has number 0
add r0 r0 3
s db Setting r0
j 1


Result (the numbers appears every 0.5 seconds):
127
256 (+129)
385 (+129)
511 (+126)
640 (+129)
769 (+129)
895 (+126)
1024 (+129)
1153 (+129)

There is a repeating +129, +129, +126 sequence, a hint that the real value is 128. Which also happens to be the number of lines in a script, which makes sense. A variation of this experiment will show that empty rows are also counted towards this number.

Push & Pop return address when calling multiple levels of functions
More advanced scripts, or scripts that wish to be more generic, may want to allow calling more than one level of function. Allowing this requires pushing the current ra register before calling the function, then popping ra back afterward.

For example, imagine that the main loop of the code wants to call function orientPanelsToStar, which would calculate the panels' orientations, then place them in r0 and r1, and then in turn itself call orientPanelsTo, which would set the orientations of all panels based on the precomputed values of r0 and r1. Doing so requires orientPanelsToStar to push ra before calling orientPanelsTo, as show in the code below.



orientPanelsToStar:
# Save return address set by the 'jal' instruction
push ra

# ...Calculate panels' orientation, for example leaving the results in r0 and r1...

# Now call orientPanelsTo to actually set the panels' orientation
# based on the computed values of r0 and r1.
jal orientPanelsTo

# ...Call other functions here if desired...

# Restore the return address of orientPanelsToStar itself
pop ra
# Return to caller
j ra

##########

orientPanelsTo:
# ...Actually set panels' orientation...

# Return to caller
j ra

This code would behave incorrectly if push ra and pop ra were not present: within orientPanelsToStar, doing jal orientPanelsTo would replace the current value of the ra register, permanently erasing where orientPanelsToStar itself should jump back to once done. Pushing and popping ra effectively saves its value until we need it again.

(A tempting but wrong approach to 'saving' ra would be to move it within a different register (e.g. r15) before calling orientPanelsTo, however that only permits two levels of functions, since if orientPanelsTo itself wants to call another function, it would not be able to use r15 to save its ra register since r15 already saves orientPanelsToStar's return address. Just pushing/popping ra fixes all issues, allowing for a ginormous maximum function call depth of 512!)

As a last note, if the script will push/pop values like ra, starting the script by clearing the stack (which where push/pop move the data to) by running clr db is advisable, unless the IC10 chip is not inserted inside an IC Housing (e.g. inserted in the slot of an Air Conditioner), since clr db will cause an error in this case. To do so, run clr db before the script's main loop.

