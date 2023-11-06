circom: 
	bash prepare.bash	

key: 
	snarkjs groth16 setup prove2.r1cs p18_final.ptau prove2_0.zkey
	snarkjs zkey contribute prove2_0.zkey prove2_1.zkey --name="1st Contributor Name" -v -e="sdfasdfddfsfs"
	snarkjs zkey export verificationkey prove2_1.zkey verification_key.json
	
proof: 
	snarkjs groth16 prove prove2_1.zkey output.wtns proof.json public.json

verify: 
	snarkjs groth16 verify verification_key.json public.json proof.json

clean: 
	rm -rf *_cpp *_js *.zkey *.r1cs *.wasm *.wtns public.json verification_key.json proof.json output.json


