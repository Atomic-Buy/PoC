circom: 
	bash prepare.bash	

key: 
	snarkjs groth16 setup prove1.r1cs p15_final.ptau prove1_0.zkey
	snarkjs zkey contribute prove1_0.zkey prove1_1.zkey --name="1st Contributor Name" -v -e="sdfasdfds"
	snarkjs zkey export verificationkey prove1_1.zkey verification_key.json
	
proof: 
	snarkjs groth16 prove prove1_1.zkey output.wtns proof.json public.json

verify: 
	snarkjs groth16 verify verification_key.json public.json proof.json

clean: 
	rm -rf *_cpp *_js *.zkey *.r1cs *.wasm *.wtns public.json verification_key.json proof.json output.json

