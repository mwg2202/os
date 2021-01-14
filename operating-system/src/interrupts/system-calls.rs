// A system call does not generally require a context switch to another process; 
// instead it is processed in the context of whichever process invoked it. 
// The hardware sees the world in terms of the execution mode according to the 
// processor status register

// Process Control
//  Create Process
//  Terminate Process
//  Load, Execute
//  get/set process attributes
//  wait for time, wait event, signal event
//  allocate and free memory

// File Management
//  create file, delete file
//  open, close
//  read, write, reposition
//  get/set file attributes

// Device Management
//  request device, release device
//  read, write, reposition
//  get/set device attributes
//  logically attach or detach devices

// Information Maintenance
//  get/set time or date
//  get/set system data
//  get/set process, file, or device attributes

// Communication
//  create, delete communication connection
//  send, recieve messages
//  transfer status information
//  attach or detach remote devices

// Protection
//  get/set file permissions
