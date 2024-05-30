# bitceptron retriever gui

This is part of the bitceptron product suite. A gui app to scan the utxo set for bitcoins locked in scripts built by custom derivation paths.

## NOTICE

***THIS IS A PROOF-OF-CONCEPTish THING. NOT PRODUCTION READY YET.***

## About

Various wallets using different BIP32 derivation paths can be a problem for bitcoiners. In some cases, this might lead to confusion or a perception of loss of bitcoins, when incompatible wallets are used for receiving bitcoins and retrieving them in a later time. Bitceptron retriever is to alleviate that problem to some extent.

Bitceptron retriever uses the txout dump of bitcoincore to scan the utxo set for various descriptors derivable from your mnemonic and passphrase. We use miniscript to create the following single key descriptors:

1. P2PK.
2. P2PKH.
3. P2SHWPKH.
4. P2WPKH.
5. P2TR (Single key path spending without a script tree).

## Usage

To use the bitceptron-retriever-gui, you must follow these steps:

1. Make sure you have a bitcoind instance running with an rpc port open to requests.
2. Build the `bitceptron-retriever-gui` from source (`cargo build --release`) or download pertinent executable.
3. Run `RUST_LOG=trace ./bitceptron-retriever-gui` from where you put your release build, which defaults to `target/release` or run `RUST_LOG=trace cargo run --release` from the root of the repository.
4. Following screen opens up:
   <img width="1136" alt="Screenshot 1403-03-10 at 11 53 39" src="https://github.com/bitceptron/bitceptron-retriever-gui/assets/139527025/1113be14-d21c-4d6d-b595-4089a46c3269">
5. Enter bitcoincore rpc data. If all inputs are valid, those lights turn green and the `Fix Setting` button activates:
   <img width="1136" alt="Screenshot 1403-03-10 at 11 56 57" src="https://github.com/bitceptron/bitceptron-retriever-gui/assets/139527025/7590bf15-669d-4e79-b086-37e573a9ba32">
6. Enter exploration settings.
   <img width="1136" alt="Screenshot 1403-03-10 at 12 01 35" src="https://github.com/bitceptron/bitceptron-retriever-gui/assets/139527025/99bd6998-a6ac-41dc-a4a4-8ce4f524fedf">
7. About base derivation paths:
  This is a vector of base derivation paths. These are the fixed parts of the derivation path, after which the exploration
  starts. These base paths should comply with these formatting rules:
  
- Must start with "m"
- Each child should be separated by a "/"
- Children may be normal or hardened. Normal children are just numbers and hardened children are numbers followed by either of "h" or " ' " characters.
  
  Some valid examples:
  
  - "m/84'/0/0"
  - "m/40/0h/0h"
  - "m/0/1/2'/4h/8"
  
  If use presets is selected, it will use the built-in list of all known base paths for bitcoin wallets which is based on the data provided by <https://walletsrecovery.org>
  
8.About exploration path:
   This is the exploration path in which the program searches. Exploration path consists of steps separated by a "/". Step semantics are as follows:

   - For any A, a member of u32: A means the specific child number A of the parent.
   - For any A and B, members of u32 with A <= B: A..B means all children number A (inclusive) to number B (inclusive) of the parent.
   - For and A, a member of u32: ..A means all the children from number 0 (inclusive) to number B (inclusive) of the parents.
   - " * " means all children from (inclusive) 0 to exploration_depth (inclusive).
   - suffixes " ' " and " h " mean all hardened children. Not using these suffixes makes all children in that step normal.
   - Suffix " a " means exploring both hardened and normal children at that step.
   
   Some valid examples(lose the spaces):
   
   - " ..100' / 50..75a / * / *"
   - " 42a / 83..120a / 68h / *a / 54h"
   - " *' / *h / *a "
9.  Choose descriptors you want to be included in search and enter path of a temp directory of your choosing. Dump file will be created or searched for in this directory.
   <img width="1136" alt="Screenshot 1403-03-10 at 12 05 41" src="https://github.com/bitceptron/bitceptron-retriever-gui/assets/139527025/c04ef3ce-615c-4202-9ff9-ee27c4c39e59">
10. Now fix settings.
  <img width="1136" alt="Screenshot 1403-03-10 at 12 06 41" src="https://github.com/bitceptron/bitceptron-retriever-gui/assets/139527025/76815de4-51ed-4fb9-96fc-6a21035d6a90">
11. You see two buttons activated now. If you want tp fetch a new utxo dump file from your bitcoincore, press `new dump file`. If you want to use a dump file already existing in your temp folder, or if nothing exists, create a new one, press `use/create dump file`. If you are on the main net, dumping the utxo set will take a while. The file is about 12GB as of block 845,771.
12. After sorting out the dump file, you see `populate database` activated. It will create an in-memory database of all ScriptPubkeys in the utxo set. Takes about 15 mins as of block 845,771. You can stop populating whenever you want.
    <img width="1136" alt="Screenshot 1403-03-10 at 12 14 16" src="https://github.com/bitceptron/bitceptron-retriever-gui/assets/139527025/f4cedc59-0bbe-46dd-afe0-cc33de0f63e1">
13. After that, `new search` activates and you can perform any search you want. Just make sure the exploration settings are fixed. You can stop search whenever you want.
    <img width="1136" alt="Screenshot 1403-03-10 at 12 16 37" src="https://github.com/bitceptron/bitceptron-retriever-gui/assets/139527025/95aa47af-d835-42e4-a4b8-1ff2551b7e11">
14. By pressing `new search` if anything is found, you'll be informed by the `Results` window.
    <img width="1136" alt="Screenshot 1403-03-10 at 12 19 17" src="https://github.com/bitceptron/bitceptron-retriever-gui/assets/139527025/756d4502-10ac-4c77-93e2-5cc3620988b7">
15. To get the details, we need to connect to the bitcoincore. So press `get details` for more. After a while you'll see more details in the `Results` window.
    <img width="1136" alt="Screenshot 1403-03-10 at 12 22 31" src="https://github.com/bitceptron/bitceptron-retriever-gui/assets/139527025/53b00769-cf21-47ad-b5be-09d63406923a">

## Improvements
This is a POC thing as of now. Can be improved in error handeling, logging and architecture fields.

## Epilogue

Happy rusting plebs.
