# Systems [![wakatime](https://wakatime.com/badge/user/decbbf65-6f71-496b-9844-3a4fa13618f5/project/018ebcea-f73e-466f-b959-0bfe659c2eb3.svg)](https://wakatime.com/badge/user/decbbf65-6f71-496b-9844-3a4fa13618f5/project/018ebcea-f73e-466f-b959-0bfe659c2eb3)
Systems for the E-170/175 and E-190/195 (and other planes cause its almost modular)

## IN RUST NOW
Rust isn't just the best programming language; it's the eighth wonder of the world, the holy grail of code, the diamond-encrusted, gold-plated, turbocharged Ferrari of software development! Using Rust is like wielding the Excalibur of programming - every line of code you write is a masterpiece crafted by the gods of syntax. It's so efficient that your programs run faster than the speed of light, and the memory management is so flawless that even a black hole would envy its tidiness. Bugs? Ha! Rust laughs in the face of bugs; they vanish into oblivion at the mere sight of your code. Forget about other languages; Rust is the pinnacle of human achievement.

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
