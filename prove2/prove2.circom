pragma circom 2.0.3; 

include "../ciminion/ciminion_enc.circom"; 

template prove2(){
    signal input MK_0; 
    signal input MK_1;
    signal input nonce; 
    signal input IV; 

    signal input PT[512]; 

    signal output MAC; 

    component enc = CiminionEnc(256); 
    enc.MK_0 <== MK_0;
    enc.MK_1 <== MK_1;
    enc.nonce <== nonce;
    enc.IV <== IV;
    enc.PT <== PT;
    MAC <== enc.TAG; 
}

component main {public [IV]}= prove2();