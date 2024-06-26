#![allow(dead_code)]
#![allow(unused_variables)]

use hex::FromHex;
use rustc_serialize::hex::ToHex;
use std::str;

pub fn xor() {
    let s0 = "15481555260c011c535055565d671501024255554b00350d131d0a0a024c0014000a024501061b0105490c464312160b4f190b550e05490d03191b5746001c4c0b0f0b4b034f";
    let s1 = "064f0610000b011c4e041645135f114f4a1f0a015241024511061d1b074e08464f030a45171c174f060c024410411c0e00164f16090a54130d00030e000c01021d1c0a4c1c041d4d0b0b00044f06045342094918004e3a41141a060011584c1b5512521f18520a4e351220411c110d571b1d4b124e031d03005267191c000d0319150006010d4e334f0648010a100a48540e00040c4e09010f0d080247491a1b110006551b1e0140";
    let s2 = "034904161d0c0259490345020048151b0d1116451300100a001e490008000b0f470c18040f541f00000c1a0c4303061c001e1b064c174313051c1b1e4e084e000800025511061c4d0c1d54074f1b4f5707044b530301060005070b45081d021d0009144f1e451d070a55034113100f411a0c45054e0e0418091b43161d1d0701065454084f0b0b474201011e1b55080600154f1647";
    let s3 = "2f001f0716001e59541f450d135b114f4c50010011451811001205061d450b46440418040115010a42491a4f16411d0d45134f01034448001a094f04450c1b1e001a1c0e5028174d0a1c101652541b4f420d4105004e0745071c000c170d40595909074f03450a0a45541f411a150f4554064e020b01000113175359";
    let s4 = "005350061d06051c54094507174e1b02480345081d521345131d0d4f034f1d03000603081318171742490048060007014e104f0205084c4105024f1a4101174c1e0f1c5350031c0e0a03115350060047100053000c18114c1d491704101d090b00071c0b4d450e1d0c450241061b59441b4f410f0a4f1c09171645054900074f051b4c0e0c0c4e0852540d040a1b471d4e0545141a1a00000747";
    let s5 = "0800121015040259541f4517174c18065e15450b175756151d001a060c49030f540c0916431b020a00000d474314034842121b0209014e4118040a5746060b000d1d454f1641302e314e151d445408410f0000070d0b1b521d4552040d104c0d48035206034519071141120d17540a4f1706410d4e0c1c090b1545571d1b48181d1d430f4f1d060e53541f1d1a1903484c04410247";
    let s6 = "02520905060a0f0c5202000b1144111c0d1109091d571301521d060143431a15540a080c0218520a160a0b410d06164400000601040b55154c191c12521c4e0408180c4e17410d02451d1d144e541a50420a5253061c1141100c52040017030c4e120141";
    let s7 = "0f4f04550b0a190b001b001c010154214204451c1d550445111c00011d0e";

    let decoded0 = Vec::from_hex(s0).expect("Decoding failed");
    let decoded1 = Vec::from_hex(s1).expect("Decoding failed");
    let decoded2 = Vec::from_hex(s2).expect("Decoding failed");
    let decoded3 = Vec::from_hex(s3).expect("Decoding failed");
    let decoded4 = Vec::from_hex(s4).expect("Decoding failed");
    let decoded5 = Vec::from_hex(s5).expect("Decoding failed");
    let decoded6 = Vec::from_hex(s6).expect("Decoding failed");
    let decoded7 = Vec::from_hex(s7).expect("Decoding failed");

    let decoded = vec![
        &decoded0, &decoded1, &decoded2, &decoded3, &decoded4, &decoded5, &decoded6, &decoded7,
    ];

    println!("{:0x?}", decoded0);
    println!("{:0x?}", decoded1);

    //initial guess:
    let p0 = "The".as_bytes().to_hex();
    let p0 = Vec::from_hex(p0).expect("Decoding failed");

    //playground for working out the relation between cipher texts and guessed word, also key candidate
    for d in decoded.clone() {
        //change decoded1 to decoded0/1/2/:
        let xored = decoded0
            .iter()
            .zip(d)
            .map(|(x, y)| x ^ y)
            .collect::<Vec<_>>();
        let xored = xored
            .iter()
            .skip(0)
            .zip(&p0)
            .map(|(x, y)| x ^ y)
            .collect::<Vec<_>>();
        let other_plaintext_v = xored.clone();
        let other_plaintext = str::from_utf8(&xored);

        println!("other:{other_plaintext:?}");

        let xored = d
            .iter()
            .zip(other_plaintext_v)
            .map(|(x, y)| x ^ y)
            .collect::<Vec<_>>();
        //key is actual candidate if p0 is corresponding to decodedN
        let key = str::from_utf8(&xored);
        println!("key candidate:{key:?}");
    }

    println!("-----------------");
    println!("after all:");
    let key = "A purely peer-to-peer version of electronic cash would allow online payments to be sent directly from one party to another without going through a financial institution. ";
    let key = key.as_bytes().to_hex();

    let key = Vec::from_hex(key).expect("Decoding failed");
    for d in decoded {
        let xored = d.iter().zip(&key).map(|(x, y)| x ^ y).collect::<Vec<_>>();
        let out = str::from_utf8(&xored);
        println!("out:{out:?}");
    }
}
