use chrono::Local;
use lab3_rust::{check_hastard_attack, check_meet_in_the_middle, get_string_hex_array, perform_hastad_broadcast_attack, perform_meet_in_the_middle_attack};
use num_bigint::BigUint;
use num_traits::Num;

const MITM_VAR_PATH: &str = "./tmp/MitM_vars/MitM_RSA_2048_20_regular/04.txt";
const SE_VARS_PATH: &str = "./tmp/SE_vars/SE_RSA_1024_5_hard/04.txt";

fn main() {
    for is_hard in 0..=1{
        let (mut c_i, mut n_i, mut l) = (Vec::new(), Vec::new(), 0);
        if is_hard == 1 {
            c_i = vec![
                BigUint::from_str_radix("1c8e662244ed68aec1436dafb3f84f74f2d1b047a54c6c685ff73404f6c5e515cabb17840bfd4de1df0e4f3334f882c60e9f05b783d7c55be35d0b02f9f3e56dac22abe13b7cd22d78eb9f709f827ab7b273b7096430df099b94276d38cadb184a945d534ad01de00acdc4dd88aa0bf527f3bf54bbd7b70d25c4fc4c92437c7c", 16).unwrap(),
                BigUint::from_str_radix("54fedb2618069fb56de6d04d0c3202ad236f026cf2efa640ba77f18d780ed87a861ff9a68c164452b47e6d2aa4189a0e3414c78dc1a702e82e4a6c976b9eb2395dfddf8471e84ca5a75f8d725cfa2379c35b1f8dbc21479adff70b4959ffef5156eab789ff58f5b4e8205e34dec6104923a0576337bd54e347392f69ee8c8ad2", 16).unwrap(),
                BigUint::from_str_radix("2672369fa3ba2eb14644587d7b1c1faea6e5c5441d07102387df3ec5f49ed45da5860053057f5bd27081adcfd424ef0c3ec41480fad66b37fe9d5f179e4072deda644a3069a1d21c37837b6837c2a493068be76ff54b6c9b0c9672942157c7735d3856e538321916cb7cee60afa81fb9218e3591c82c780ef1f5cc837dee76c2", 16).unwrap(),
                BigUint::from_str_radix("befb533504c6d59ae53220703f26832d70bff1f821ce0682a8b8d2585e0a20226737b9cba68b684a8a4460229fca98d5ce6f12d735cc4e00f0ed547c151b07b366e019fb7b4e6e2d17be07e9fb8ce08bea07aed7eff82d41db88d09391eec8fc5b3ea7be4be97c661166b90e9ed12ecab8f06ce114516ea51119a66ca2053d39", 16).unwrap(),
                BigUint::from_str_radix("08b49b6929557c76040abe1d9ad891219b16858fbf99db1d00df92f0b6236148db9f0d71fabc2b2c7a235d20211786d09c4d9c902454602e46a5ff304083b4f7e9cac16bca12270c66498323460469d2bedb6216b794db5dc6276313a610f7a65142df3f5dc10f69bf7a97ae76a452c1333d41c1c4f3d7f42dbb9c72a71e6ccb", 16).unwrap(),
            ];
            n_i = vec![
                BigUint::from_str_radix("C6E249C705C2BA092F925819623253EF8F23DA3E2CB70106AFCE07E8FFF4024BFA994A7BC33C99EA8A8320F0B9E0368403ABEAB78F8DC6AB7D44117022A2DE96FBE48E2DABA525888924B0172DD11197173C6D7DCA6D745CCD9027B90ADA59156C106AD28653D528845DB109C7F1C6921F4FE55AE00A68B5C673BFCA3BA70651", 16).unwrap(),
                BigUint::from_str_radix("C578EEC934A8FA5A3F436F6342D4B4A661A176FDFD4283969E06EE51E0C9D5A897321C5425C2DFC0B66BB830CE9896CAD459651BDE2EAEB8286610601905F35FAF6EAB4D29F0D0F1DF788EAFD6416A142ABA2640803B0C88760FA271A7F7575AE6441D701A3E11EF89D448FA7C95F2B22641871C7C8F3831320F9AEB9DC433F3", 16).unwrap(),
                BigUint::from_str_radix("BCEF8437014F92C2E947B74E8D6F78E1EC540FA88756E0E2270CA31AFDE3A495EF5D488489B7B9B6203F1924470C4588CD420A8E935EB26E1F93CABAC4F6BEE1DEB87D587422AA0EE3057BA874DC66447F9E1D452EF18598E9B14F3F89FB40AFC9768DBD452CB9D853E1E4783B7E2C04B324F386CCC80F2A24E3356F4CEBCF73", 16).unwrap(),
                BigUint::from_str_radix("E55057E99DC6F5978D0BC1A1478800785E789329A87CA934460768E5DEB4CB197C8F1C4B074F6E6DE938D6D0F1F60B5A30D8EE62F197B97905978D9C57693B9985CA986C59ABB9613627EF252867F24D545A79A01BBD501B9A904521DA90B1EA7FFD8A21152FF77734728E8CEFFE7F9B9C8EA4A6DA4F836C3FFA5CEF2582556B", 16).unwrap(),
                BigUint::from_str_radix("C35E7C67D8487ECD88AA7DB0093039F5A9646660D16BC56ADD906225855419047E59A8602E6875E02C52188B98B04C643C7AC1BB27F2DEA9D6B5A4BFE4AC99CF0FAD26446C6180D0F576BCD4B8792CDD4BC45FD077AB012722B57AE4E43CC5A6079C2B8F55E2A8D8B9179C5C3F7D0EF3C43B6A4B90AE186B52438490D939F1F3", 16).unwrap(),
            ];
        } else {
            c_i = vec![
                BigUint::from_str_radix(
                    "1b11452e4f15cc1fdf65f2d0c09773a56295b8447ada0d02f095ace99c857391",
                    16,
                )
                    .unwrap(),
                BigUint::from_str_radix(
                    "425d16b88571f6f9e68e654c9578654933dd3cd5b20c94cd2094d97aafa4a6f5",
                    16,
                )
                    .unwrap(),
                BigUint::from_str_radix(
                    "8c7f8d4b03879db2c7942ecee64aae63aa195cf1b2ee9b1037f96fa21f83c6d7",
                    16,
                )
                    .unwrap(),
            ];
            n_i = vec![
                BigUint::from_str_radix(
                    "993F26C703E0FC07A0B64325CDBE56B26630092DF5852F5A161C3AF11D075409",
                    16,
                )
                    .unwrap(),
                BigUint::from_str_radix(
                    "A1ABFDEBA3E44475314CDB9157C29767A2DD9297E2802D00A92D249D7FCCD509",
                    16,
                )
                    .unwrap(),
                BigUint::from_str_radix(
                    "D3D72C10F8F3E4CFF260F5C7C042719468C653437C3C1926E8BBCBEDEE7B7D41",
                    16,
                )
                    .unwrap(),
            ];
        }

        let before = Local::now();
        let m = perform_hastad_broadcast_attack(&c_i, &n_i);
        let after = (Local::now() - before);
        println!(
            "is_hard: {is_hard}, [Hastard broadcast attack] m: {}, time_spent: {}, check: {:?}",
            get_string_hex_array(&m.to_radix_be(16)), after, check_hastard_attack(&c_i, &n_i,&m)
        );

        if is_hard == 1 {
            c_i = vec![
                BigUint::from_str_radix("74de573c7ccc83076aa3744ced51f5e058ac7a3e41db240fe4719b381817c424e3c6be62578a3d823d009e0343feeca2eec2b94e163ad7b522f1c0174f8f47f5cc4c7b07c9ff0ddc4f5116369477cca0a9cfba0f2973eb889baf5f9f4310e154650e4b00c0e437b95293074eabaa539bf7ff6f648ab02a7b8b6aff764f60cc77d43a0331899820e6762f2c8ceb709635fc0c2c2c6682863a3a6659a1d2cac64f064543b5eccdb4bb2d685a9bc46f49b003506da3727b6ae95fe1b62fdd8329cd6a387290fa117a824b740c0a2809ff26cf2e704cb62c300660303e2403e8b7c455b00b9436fc330ad0585736889c22061d80b90cbfd313b1ba92f60e5cd16dc5", 16).unwrap(),
            ];
            n_i = vec![
                BigUint::from_str_radix("ABF2848CD165F354E9C0ABC7BE0E18DD357B96C109EB06936DFCB37E77BA44BD15EA849B03E3971F91573D61D3932C3D25BF9DB0FC25910F7BC854F111FF61ABEA968F90ECB8BD126440B8A9326438C6812E4D552981FF2A02DCF6E7F6AC782CC7636A4265BDACAEB2BD60825DC670AEDE593F45FCEA3653870B8FADA0829E5B1FB1D5B63BE1035CF897AA436563A13B07DCAB16012D3F8DCB7A0497EEDCBE21D05C20732D9A31CD917CE5F060BA4D0F92BEBAE885E9832F20BD177F92613E8AD2A42AE7CEF35E769C03741AA70FAC5EE842C89E45520712DCE08CF2C1CC167B559AD0E17125072A2A1C041382BB0CB299B8E647BE046FAD04462C641ACB4C9F", 16).unwrap(),
            ];
            l = 20;
        } else {
            c_i = vec![
                BigUint::from_str_radix(
                    "1bede6fd06724fa47581ef8dd5c7fe537f389ce72582951c901bcfcb48ea911cc2882d5d10025ad5db6e533db506a0d52d4482a4f790209117052a44ec1ad1e1",
                    16,
                )
                    .unwrap(),
            ];
            n_i = vec![
                BigUint::from_str_radix(
                    "BE9876D922C5BF0A33E2A96FEBCC9AAF7C2BF64071363B8B5C0DB75F5705F46E421D5E365A0C6220BA36E712D62706D369FED0B306DCDD7B4ACFC567AEB67A85",
                    16,
                )
                    .unwrap(),
            ];
            l = 20;
        }

        let before = Local::now();
        let m = perform_meet_in_the_middle_attack(l, &c_i[0], &n_i[0]).unwrap();
        let after =  Local::now() - before;
        println!(
            "is hard: {is_hard}, [Meet in the middle] m: {:02X?}, time_spent: {}, check: {:?}",
            get_string_hex_array(&m.to_radix_be(16)),after.num_minutes(), check_meet_in_the_middle(&c_i[0], &n_i[0], &m)
        );

        // println!("Checking meet_in_the_middle is_hard: {is_hard}, status: {:?}",check_meet_in_the_middle(&c_i[0], &n_i[0], &(BigUint::from(715293_u64) % &n_i[0])));
    }
}
