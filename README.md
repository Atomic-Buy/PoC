# Proof of Content 

This repo provide some circom pritimives for an undergoing project. 

## Prove 
For any 15KB data `C`, which can be divided into `512 * 240 bits = 512 big number`.

### Prove One: prove of recursive poseidon hash
Commit any content with size less than 15 KB using recursive poseidon hash.  In the commitment process, we using 3 layer of poseidon hash to compose a merkle-like tree structure to commit the content array. each poseidon hash receive 8 numbers as input and ouput one number as the hash. 
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
 
$$ h = recursive_poseidon(C)$$

### Prove TWO 
Commit 512 numbers using [Ciminion](https://eprint.iacr.org/2021/267) Symmetric Encryption. The Ciminion Circom is implemented by
[Kudelski Security](https://github.com/kudelskisecurity/circom-ciminion). 
Ciminion encryption function take 4 inputs: 
- static IV
- a nonce
- two master key
- plaintext pair

Outputs: 
- A TAG: the MAC of the plaintext 
- ciphertext pair: the corresbonding ciphertext for plaintext 


In Prove 2, we prove the following claim: `h == Enc(public IV, nonce, masterkey[2], plaintext[N * 2]).TAG`
- This claim that **"I have some plaintext and its MAC is `h` "**. 

### Prove THREE 

In Prove 3, we prove the claim that: `MAC(public IV, nonce,  masterkey[2],public ciphertext[N*2] ) == h && Enc(public IV, nonce2, sk[4], masterkey[2] ).ciphertext == c2[2]`, where `sk[4] = [IV, nonce, masterkey[0], masterkey[1] ]`
- This claim that **"I provides the `ciphertext` that represened by `h` in Prove2 and I give you the private key `sk` which can decode the `ciphertext` to plaintext.  "**. 




## Dependencies 
- circom_tester: 
- circomlib 2.1: https://github.com/iden3/circomlib/tree/circom2.1
- circom ciminion: https://github.com/kudelskisecurity/circom-ciminion
