# Proof of Content 

For any 15KB data `C`, which can be divided into `512 * 240 bits = 512 big number`. `hash()` is 3-layer recursive 8-poseidon hash. We first prove: 

$$ h = hash(C)$$

For any 512 number  `C` and 512 number `K`, a symmetric encryption `Enc`(we use Ciminion),  we prove: 

$$ h=hash(C) \\ X=Enc_K(C) $$ 


## Dependencies 

- circomlib 2.1: https://github.com/iden3/circomlib/tree/circom2.1
- circom ciminion: https://github.com/kudelskisecurity/circom-ciminion