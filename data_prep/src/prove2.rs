
use rand::Rng;
use std::io::prelude::*;
use num_bigint::BigUint;

use crate::prove1::{Num, Data15K};

/* 
// ark circom 
use ark_circom::{CircomBuilder, CircomConfig};
use ark_std::rand::thread_rng;

use ark_bn254::Bn254;
use ark_crypto_primitives::snark::SNARK;
use ark_groth16::Groth16;

type GrothBn = Groth16<Bn254>;
*/
const IV: &str = "123456789";


#[derive(Clone)]
pub struct EncInput512{
    pub master_key_0: BigUint,
    pub master_key_1: BigUint,
    pub nonce: BigUint, 
    pub iv: BigUint,
    data: Data15K
    
} 

impl EncInput512{
    /// new a encryption input and keys from src file path, save the private key in pk_path
    pub fn new(src_path: &str, sk_path: &str) -> EncInput512{
        // gen 3 bigutint with 240 bytes 
        let mut rng = rand::thread_rng();
        let t1:Vec<u8> = (0..30).map(|_| rng.gen()).collect();
        let mk0 = BigUint::from_bytes_be(&t1);
        //println!("mk0: {}", mk0.to_str_radix(10)); 
        let t2:Vec<u8> = (0..30).map(|_| rng.gen()).collect();
        let mk1 = BigUint::from_bytes_be(&t2);
        let t3:Vec<u8> = (0..30).map(|_| rng.gen()).collect();
        let nonce = BigUint::from_bytes_be(&t3);
        // parse IV from const IV 
        let iv = BigUint::parse_bytes(IV.as_bytes(), 10).unwrap();
        // build a data15k from src path 
        let data = Data15K::new(src_path).unwrap();
        // save the pk (MK_0, MK_1, nonce, IV) to pk_path in json format 
        let mut sk_file = std::fs::File::create(sk_path).unwrap();
        let sk = serde_json::json!({
            "MK_0": mk0.to_str_radix(10),
            "MK_1": mk1.to_str_radix(10),
            "nonce": nonce.to_str_radix(10),
            "IV": IV
        });
        println!("sk: {}", sk.to_string());
        sk_file.write_all(sk.to_string().as_bytes()).unwrap();
        // return a EncInput512
        EncInput512{
            master_key_0: mk0,
            master_key_1: mk1,
            nonce: nonce,
            iv: iv,
            data: data
        }
    }

    pub fn restore(src_path: &str, sk_path: &str) -> EncInput512{
        // read the sk from sk_path
        let mut sk_file = std::fs::File::open(sk_path).unwrap();
        let mut sk_str = String::new();
        sk_file.read_to_string(&mut sk_str).unwrap();
        let sk_json: serde_json::Value = serde_json::from_str(&sk_str).unwrap();
        // parse the sk_json to biguint
        let mk0 = BigUint::parse_bytes(sk_json["MK_0"].as_str().unwrap().as_bytes(), 10).unwrap();
        let mk1 = BigUint::parse_bytes(sk_json["MK_1"].as_str().unwrap().as_bytes(), 10).unwrap();
        let nonce = BigUint::parse_bytes(sk_json["nonce"].as_str().unwrap().as_bytes(), 10).unwrap();
        let iv = BigUint::parse_bytes(sk_json["IV"].as_str().unwrap().as_bytes(), 10).unwrap();
        // build a data15k from src path 
        let data = Data15K::new(src_path).unwrap();
        // return a EncInput512
        EncInput512{
            master_key_0: mk0,
            master_key_1: mk1,
            nonce: nonce,
            iv: iv,
            data: data
        }
    }
    //// generate the circom input json file in format: 
    /// {MK_0: "12", MK_1: "12", nonce: "12", IV: "12", PT: ["12", "12", ...]}
    pub fn gen_circom_json(&self, circom_json_path: &str){
        // gen a circom json from self 
        let mut circom_json_file = std::fs::File::create(circom_json_path).unwrap();
        // gen pt string vec frist 
        let mut pt_str_vec: Vec<String> = Vec::new();
        for i in 0..512{
            pt_str_vec.push(self.data.data[i].to_decimal());
        }
        let circom_json = serde_json::json!({
            "MK_0": self.master_key_0.to_str_radix(10),
            "MK_1": self.master_key_1.to_str_radix(10),
            "nonce": self.nonce.to_str_radix(10),
            "IV": IV,
            "PT": pt_str_vec
        });
        circom_json_file.write_all(circom_json.to_string().as_bytes()).unwrap();

    }
    
}

#[cfg(test)]
mod tests{
    use super::*; 
    #[test]
    fn test_prove2_key(){
        // create a file that is 15K 
        let mut file = std::fs::File::create("test1.txt").unwrap();
        let mut data = Vec::new();
        for i in 0..15*1024{
            data.push(0);
        }
        file.write_all(&data).unwrap();
        // new a encinput512
        let encinput512 = EncInput512::new("test1.txt", "test1_sk.json");
        // check if the sk is saved in test_sk.json by restore it
        let encinput512_restore = EncInput512::restore("test1.txt", "test1_sk.json");
        
        // check if the keys in encinput512_restore are the same as encinput512
        assert_eq!(encinput512.master_key_0, encinput512_restore.master_key_0);
        assert_eq!(encinput512.master_key_1, encinput512_restore.master_key_1);
        assert_eq!(encinput512.nonce, encinput512_restore.nonce);
        assert_eq!(encinput512.iv, encinput512_restore.iv);
        // remove all test files
        std::fs::remove_file("test1.txt").unwrap();
        std::fs::remove_file("test1_sk.json").unwrap();

    }
    #[test]
    fn test_prove2_circom_json(){
        // create a file that is 15K 
        let mut file = std::fs::File::create("test2.txt").unwrap();
        let mut data = Vec::new();
        for i in 0..15*1024{
            data.push(0);
        }
        file.write_all(&data).unwrap();
        // new a encinput512
        let encinput512 = EncInput512::new("test2.txt", "test2_sk.json");

        // gen a circom json from encinput512
        encinput512.gen_circom_json("test2_circom.json");
        // rm all test files 
        std::fs::remove_file("test2.txt").unwrap();
        std::fs::remove_file("test2_sk.json").unwrap();
        std::fs::remove_file("test2_circom.json").unwrap();

    }

}

