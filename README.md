# ethcdm_reservations
A reservations system based on ICP and QR codes.

## Concept

## Files structure
- src/ -> source files of the proyect
    - backend/ -> the canister source coude written in rust
    - frontend/ -> the reservations frontend source code 
    - mobile/ -> the host's application frontend code 

## Run 
1. Start dfx process from the root of the project with `dfx start --clean`
2. Deploy canisters to local replica with `dfx deploy`
3. Access canisters from the given urls

## Usage
The idea behind the project is to allow people to generate reservations from the `frontend` canister, these reservations will generate a QR code and a record with the price paid registerred in the canister. The experience's host will then scan the qr code with the `mobile` frontend canister and will get the information of the reservation. The money will be sent to the host account upon scanning.
