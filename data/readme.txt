src.jpg 

ciminion_enc_input.json: we convert the src.jpg for readable format for circom Ciminion. This also include the keys we used in encryption. 

sk_seller.json: the corresbonding key part used in ciminion_enc_input.json. 

mac_ct_iv.json: the encrypted mac, ciphertext and iv after encryption. 

mac_iv.json: only the mac and iv using encryption. 

buyer_sk.json: the buyer's keys used in prove3. 

prove3_input.json: combine mac_ct_iv.json, sk_seller.json and buyer_sk.json

prove3_output.json: 
    ciphertext of seller's key, which can be decrpyted by buyer's key 
    IV of buyer's key
    MAC of the ciphertext. 




