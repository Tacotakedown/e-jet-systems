# Systems
Systems for the E-170/175

### what is a system?
Any task or aspect of the aircraft that requires constant or regular manipulation of simvars.

### why do these not run in WASM
its wasm = its glorified js, think thats enough reason. I simulated the heat of passangers and food, aint no way that shit gonna run well in the sim

### How the systems work for the ejet:
1- Systems logic and system tasks (reading and writing) are handled by two seperate sides of the codebase. This means any system will happly run with or without simvar presence (the systems shouldnt know they are a part of msfs at all)
2- Spawn a tokio task for each system logic. Interthread communicaiton of data will be based on Arc Mutex variables rather than passing the simvar context into the logic of the systems, like i said, no sim presence in logic threads
3- results are parsed from each system thread each tick, then we will just call the simvarwriter to write the group of simvars to the sim
4- profit $$$

### Contributing:
if you can understand the codebase and think you can do better (shouldnt be hard) then submit a pr idfk
