
use rand::Rng;
use std::io::prelude::*;
use num_bigint::BigUint;

use crate::prove1::{Num, Data15K};

pub struct DecInput512{
    buyer_MK_0: BigUint,
    buyer_MK_1: BigUint,
    buyer_nonce: BigUint,
    buyer_IV: BigUint,
    seller_MK_0: BigUint,
    seller_MK_1: BigUint,
    seller_nonce: BigUint,
    seller_IV: BigUint,
}

impl DecInput512{

    /// new a Decinput512 from buyer's sk file path  
    pub fn new(buyer_path: &str) -> Result<DecInput512, String> {
        // read the sk from buyer_path
        let mut buyer_sk_file = std::fs::File::open(buyer_path).unwrap();
        let mut buyer_sk = String::new();
        buyer_sk_file.read_to_string(&mut buyer_sk).unwrap();
        // parse the sk to json
        let buyer_sk_json: serde_json::Value = serde_json::from_str(&buyer_sk).unwrap();
        // parse the sk_json to biguint
        let buyer_mk0 = BigUint::parse_bytes(buyer_sk_json["MK_0"].as_str().unwrap().as_bytes(), 10).unwrap();
        let buyer_mk1 = BigUint::parse_bytes(buyer_sk_json["MK_1"].as_str().unwrap().as_bytes(), 10).unwrap();
        let buyer_nonce = BigUint::parse_bytes(buyer_sk_json["nonce"].as_str().unwrap().as_bytes(), 10).unwrap();
        let buyer_iv = BigUint::parse_bytes(buyer_sk_json["IV"].as_str().unwrap().as_bytes(), 10).unwrap();
        // build decinput512 from buyer's sk
        Ok(DecInput512{
            buyer_MK_0: buyer_mk0,
            buyer_MK_1: buyer_mk1,
            buyer_nonce: buyer_nonce,
            buyer_IV: buyer_iv,
            seller_MK_0: BigUint::from(0u32),
            seller_MK_1: BigUint::from(0u32),
            seller_nonce: BigUint::from(0u32),
            seller_IV: BigUint::from(0u32),
        })
    }

    pub fn add_seller_key(&mut self, seller_path: &str) -> Result<(), String>{
        // read the sk from seller_path
        let mut seller_sk_file = std::fs::File::open(seller_path).unwrap();
        let mut seller_sk = String::new();
        seller_sk_file.read_to_string(&mut seller_sk).unwrap();
        // parse the sk to json
        let seller_sk_json: serde_json::Value = serde_json::from_str(&seller_sk).unwrap();
        // parse the sk_json to biguint
        let seller_mk0 = BigUint::parse_bytes(seller_sk_json["MK_0"].as_str().unwrap().as_bytes(), 10).unwrap();
        let seller_mk1 = BigUint::parse_bytes(seller_sk_json["MK_1"].as_str().unwrap().as_bytes(), 10).unwrap();
        let seller_nonce = BigUint::parse_bytes(seller_sk_json["nonce"].as_str().unwrap().as_bytes(), 10).unwrap();
        let seller_iv = BigUint::parse_bytes(seller_sk_json["IV"].as_str().unwrap().as_bytes(), 10).unwrap();
        // add seller's sk to self
        self.seller_MK_0 = seller_mk0;
        self.seller_MK_1 = seller_mk1;
        self.seller_nonce = seller_nonce;
        self.seller_IV = seller_iv;
        Ok(())
    }

    pub fn gen_prove3_input(&self, mac_ct_iv_path: &str, output_path: &str) -> Result<(), String>{
        // read the mac_ct_iv from mac_ct_iv_path, parse it as bigUint vec 
        let mut mac_ct_iv_file = std::fs::File::open(mac_ct_iv_path).unwrap();
        let mut mac_ct_iv_str = String::new();
        mac_ct_iv_file.read_to_string(&mut mac_ct_iv_str).unwrap();
        let mac_ct_iv_json: serde_json::Value = serde_json::from_str(&mac_ct_iv_str).unwrap();
        let content_mac = BigUint::parse_bytes(mac_ct_iv_json[0].as_str().unwrap().as_bytes(), 10).unwrap();
        let mut content_ct: Vec<BigUint> = Vec::new(); 
        for i in 1..513{
            content_ct.push(BigUint::parse_bytes(mac_ct_iv_json[i].as_str().unwrap().as_bytes(), 10).unwrap());
        }
        let seller_iv = BigUint::parse_bytes(mac_ct_iv_json[513].as_str().unwrap().as_bytes(), 10).unwrap();

        // check seller iv 
        if seller_iv != self.seller_IV{
            return Err("seller iv not match".to_string());
        }
        // convert the content_ct to a vector of string
        let mut content_ct_str: Vec<String> = Vec::new();
        for i in 0..512{
            content_ct_str.push(content_ct[i].to_str_radix(10));
        }
        // compose the output json
        // format: {MK_0_buyer: 1122, MK_1_buyer: 1122, nonce_buyer: 1122, IV_buyer: 1122, MK_0_seller: 1122, MK_1_seller: 1122, nonce_seller: 1122, IV_seller: 1122, content_mac: 1122, content_ct: [1122, 1122, ...]}
        let output_json = serde_json::json!({
            "MK_0_buyer": self.buyer_MK_0.to_str_radix(10),
            "MK_1_buyer": self.buyer_MK_1.to_str_radix(10),
            "nonce_buyer": self.buyer_nonce.to_str_radix(10),
            "IV_buyer": self.buyer_IV.to_str_radix(10),
            "MK_0_seller": self.seller_MK_0.to_str_radix(10),
            "MK_1_seller": self.seller_MK_1.to_str_radix(10),
            "nonce_seller": self.seller_nonce.to_str_radix(10),
            "IV_seller": self.seller_IV.to_str_radix(10),
            "CT1": content_ct_str,
            "MAC": content_mac.to_str_radix(10)
        });
        // write the output json to output_path
        let mut output_file = std::fs::File::create(output_path).unwrap();
        output_file.write_all(output_json.to_string().as_bytes()).unwrap();
        Ok(())
    }
    pub fn gen_dec_input(&self, mac_ct_iv_path: &str, output_path: &str) -> Result<(), String>{
        // read the mac_ct_iv from mac_ct_iv_path, parse it as bigUint vec 
        let mut mac_ct_iv_file = std::fs::File::open(mac_ct_iv_path).unwrap();
        let mut mac_ct_iv_str = String::new();
        mac_ct_iv_file.read_to_string(&mut mac_ct_iv_str).unwrap();
        let mac_ct_iv_json: serde_json::Value = serde_json::from_str(&mac_ct_iv_str).unwrap();
        let content_mac = BigUint::parse_bytes(mac_ct_iv_json[0].as_str().unwrap().as_bytes(), 10).unwrap();
        let mut content_ct: Vec<BigUint> = Vec::new(); 
        for i in 1..513{
            content_ct.push(BigUint::parse_bytes(mac_ct_iv_json[i].as_str().unwrap().as_bytes(), 10).unwrap());
        }
        let seller_iv = BigUint::parse_bytes(mac_ct_iv_json[513].as_str().unwrap().as_bytes(), 10).unwrap();

        // check seller iv 
        if seller_iv != self.seller_IV{
            return Err("seller iv not match".to_string());
        }
        // convert the content_ct to a vector of string
        let mut content_ct_str: Vec<String> = Vec::new();
        for i in 0..512{
            content_ct_str.push(content_ct[i].to_str_radix(10));
        }
        // compose the output json
        // format: {MK_0_buyer: 1122, MK_1_buyer: 1122, nonce_buyer: 1122, IV_buyer: 1122, MK_0_seller: 1122, MK_1_seller: 1122, nonce_seller: 1122, IV_seller: 1122, content_mac: 1122, content_ct: [1122, 1122, ...]}
        let output_json = serde_json::json!({
            "MK_0_buyer": self.buyer_MK_0.to_str_radix(10),
            "MK_1_buyer": self.buyer_MK_1.to_str_radix(10),
            "nonce_buyer": self.buyer_nonce.to_str_radix(10),
            "IV_buyer": self.buyer_IV.to_str_radix(10),
            "MK_0_seller": self.seller_MK_0.to_str_radix(10),
            "MK_1_seller": self.seller_MK_1.to_str_radix(10),
            "nonce_seller": self.seller_nonce.to_str_radix(10),
            "IV_seller": self.seller_IV.to_str_radix(10),
            "CT1": content_ct_str,
            "MAC": content_mac.to_str_radix(10)
        });
        // write the output json to output_path
        let mut output_file = std::fs::File::create(output_path).unwrap();
        output_file.write_all(output_json.to_string().as_bytes()).unwrap();
        Ok(())
    }

}

#[cfg(test)]
mod tests{
    #[test]
    fn test_prove3(){
        // new a decinput512 from buyer's sk file path
        let mut decinput512 = super::DecInput512::new("./buyer_sk.json").unwrap();
        // add seller's sk to decinput512
        decinput512.add_seller_key("./seller_sk.json").unwrap();
        // gen prove3 input
        decinput512.gen_prove3_input("./mac_ct_iv.json", "./prove3_input.json").unwrap();
    }
}