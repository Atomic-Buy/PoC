/* Decryption test for Ciminion circuit */

const chai = require("chai");
const path = require("path");
const F1Field = require("ffjavascript").F1Field;
const Scalar = require("ffjavascript").Scalar;
exports.p = Scalar.fromString(
  "21888242871839275222246405745257275088548364400416034343698204186575808495617"
);
const Fr = new F1Field(exports.p);

const wasm_tester = require("circom_tester").wasm;

const assert = chai.assert;

describe("Ciminion - encryption operation", function () {
  this.timeout(100000);

  it("Decryption check", async () => {
    const ciminion_input_dec = {
      MK_0: "1493553771752236207272610030712060271193289939876553297276263782386575059",
      MK_1: "1292038353963878406282758258603470598724349918981955977917818779094141974",
      nonce: "1605505514096142962680974126667713297155356298812625332291840092077046148",
      IV: "123456789",
      CT: [
        "11971137762055684285114671818099979675735907173530824426685425114132272231291",
        "15606949932650811862926188075810612227187103596999854474311995572308274881825",
        "11249294338808233840027162635349570412956191750288639902375570954189347020692",
        "3366133725721170842993113440054283774035249332476454410557162187340510341641",
      ],
    };

    const circuit_dec = await wasm_tester(
      path.join(__dirname, "../dec", "dec_sk.circom")
    );
    let witness_dec;
    witness_dec = await circuit_dec.calculateWitness(ciminion_input_dec, true);
    let t = await circuit_dec.getDecoratedOutput(witness_dec);
    // save in file as plaintext 
    const fs = require("fs");
    fs.writeFileSync("plaintext.json", JSON.stringify(t));
    

    await circuit_dec.assertOut(witness_dec, {
      PT: [
        "199997040245289573843469830738771149996678918678672760377350279807098037",
        "1331165210433931980483654342964946251762496413189840213140035047532383283",
        "838345152875747577432169841715608031120461514379080819692441178540799571",
        "123456789",
      ],
    });
  });
});
