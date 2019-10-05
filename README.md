# Secure Ballot
This is a confidential voting dapp running on a blockchain employing an enhanced voting system Democracy 2.1 (D21).

## Technology
Oasis Labs platform - The smart concract is written in Rust and executed in Oasis Labs Network via WASM. The transactions aren't called via Web3 as a thin client but via Oasis Client which is encrypted.
D21 - First proposed by Czech mathematican and filantrop Karel Janecek, this voting system enables upto 2 positive votes and 1 negative to each voter to use, thus resulting in a statistical advantage of "sophisticated voters" and restrinction of populistic candidates. Because each voter has more votes, more people can agree on a common thing.

## Project structure
The smart contract "main.rs" is located in folder "service/src".
Back and Front-End is in folder "app".
Project presentation "Hacktothemoon.odp" is in the main folder.

## Links
Oasis Labs: https://www.oasislabs.com/
D21 (whitepaper): https://drive.google.com/file/d/1om-flxepX8spQU_dxgu0Y57o0ZsJi-_h/view
Hacktothemoon: https://hacktothemoon.com/



Created at blockchain hackatlon held in Prague 28.-29. Sept. 2019 (we won!)