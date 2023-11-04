# Proof of Content 

For any 15KB data `C`, which can be divided into `512 * 240 bits = 512 big number`. `hash()` is 3-layer recursive 8-poseidon hash. We first prove: 

$$ h = hash(C)$$

For any 512 number  `C` and 512 number `K`, a symmetric encryption `Enc`(we use Ciminion),  we prove: 

$$ h=hash(C) \\ X=Enc_K(C) $$ 

## Usage 

## Implementaion 

### Prove One 

Commit any content with size less than 15 KB using circom 

- In `data_prep` dir, set your content path in `main.rs` run `cargo run`, which will generate a input.json for content commitment. This step will convert any legit content to 512 big numbers on BN254. 
- In the commitment process, we using 3 layer of poseidon hash to compose a merkle-like tree structure to commit the content array. each poseidon hash receive 8 numbers as input and ouput one number as the hash. 

```
512 numbers 
|
└───LayerN(3)
|   └───Poseidon[0]...Poseidon[63]
|
64 hashes
└───LayerN(2)
|   └───Poseidon[0]...Poseidon[7]
|
8 hashes 
└───LayerN(1)
|   └───Poseidon[0]
|
1 root hash 
```
 
$$ h = hash(C)$$


## Dependencies 

- circomlib 2.1: https://github.com/iden3/circomlib/tree/circom2.1
- circom ciminion: https://github.com/kudelskisecurity/circom-ciminion