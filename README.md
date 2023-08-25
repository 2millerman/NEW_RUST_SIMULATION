# NEW_RUST_SIMULATION
To run this program, you may have to install a virtual environment. You can use "pip install virtualenv" to achieve this. 
To activate the virtual environment on macos you can use "source myenv/bin/activate". For windows, ".\myenv\Scripts\activate" should work.
Make sure to replace "myenv" with whatever name you give it.

To update and compile the rust portion of the code, use type "maturin develop" into the terminal.
to run the simulation, type in "python3 simulation.py" It will then prompt you to: "Enter a room size probability between 0 and 1"
entering a number closer to 0 results in smaller rooms and a number closer to 1 results in larger rooms.

It will then ask users to "Enter a hallway factor between 0 and 1." a hallway factor closer to 0 results in less hallways between rooms while a factor closer to 1 
results in more hallways between rooms.

It will then ask users to "enter number of agents". The user can then enter the number of agents they want moving throughout the simulation.

T0 change the number of rooms, in the simulation, there is a CONST towards the start of lib.rs named NUMBER_OF_ROOMS which the user can change.
to change the size of the hallways, there is a CONST towards the start of lib.rs named HALLWAY_WIDTH which the user can change.
 
In simulation.py line 159 users can change the speed of some agents by entering a number inside random.randint. 
It will then randomly select some agents to move at the speeds you input.
