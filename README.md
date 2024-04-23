# Systems
Systems for the E-170/175

![logo](https://ouroborosjets.com/images/logo.png)

### What is a system?
A system refers to any task or aspect of the aircraft that requires constant or regular manipulation of simvars. These tasks are computationally intensive and should not run in JavaScript.

### Why don't these run in WASM?
The decision to not use WebAssembly (WASM) was made to leverage a multi-threaded async runtime for a more fluid simulation experience.

### How do they launch?
The [Infinity Launcher](https://github.com/infinity-MSFS) initiates the systems in the background upon receiving the event that you are loading the plane.

### How the systems work for the E-Jet:
1. **Systems Logic and Tasks:** Handling of systems logic and tasks (reading and writing) are divided into two separate sides of the codebase. This design ensures that any system can run independently with or without simvar presence, as the systems remain agnostic to their MSFS environment.
2. **Tokio Tasks:** Each system is spawned as a Tokio task. Interthread communication of data is based on Arc Mutex variables instead of passing the simvar context into the logic of the systems. This approach ensures that there is no direct sim presence in logic threads.
3. **Result Parsing:** Results from each system thread are parsed each tick. Subsequently, the Simvar Writer is called to write the group of simvars to the simulator.
4. **Profit:** Achieve desired functionality and enhance the simulation experience.
