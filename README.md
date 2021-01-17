# I2T Streams BLE Gateway

## IMPORTANT
This software was so far only tested on Linux and Linux-based Operating systems, it has not yet been tested on Windows and MacOS. If you encounter issues with these systems please open an Issue on this repository.

## Preparation
Install rust if you don't have it already, find the instructions here https://www.rust-lang.org/tools/install

Make sure you also have the build dependencies installed, if not run:  
`sudo apt install build-essential`  
`sudo apt install pkg-config`  
`sudo apt install libssl-dev`  
`sudo apt-get install libdbus-glib-1-dev`  
`sudo apt update`  

## Installing the streams-ble-gateway

Download the Repository:  

`git clone https://github.com/iot2tangle/streams-ble-gateway.git`  
  
Make sure that the Bluetooth is ON and your BLE Devices are in reach and have a name, than run:  
`cargo run --release --bin scan`  
  
This will list all BLE devices and their MAC address in reach of the Gateway, copy the MAC addresses of the devices you want to connect to into the *config.json* file  
  
Configure the streams-gateway:  
`nano config.json`  
 
Set the *device_ids* to include all the device IDs specified in their respective configuration files.  
Set the *reading_interval* to define how often the Gateway will read data from the devices (note: this is not precise and also depends on how long the IOTA-Proof-Of-Work takes).  
Change *node, mwm, local_pow* if needed.  



## Runnig the Gateway:  
  
Run the streams-gateway:  

`cargo run --release --bin ble-gateway`  

This starts the server which will forward messages from the devices to the Tangle  
  
The Output will be something like this:  

`>> Starting.... `  
`>> Channel root: "47d504e1a825e142dd899dda81ff787c7cfad3b83977feec3545eaef4315c8a50000000000000000:fd93e57d937910f429cdd211"`  
  
`>> To read the messages copy the channel root into https://explorer.iot2tangle.io/`  
 

The gateway will now connect to each device in the list sequentially anf publish the sensor data into the Channel.
