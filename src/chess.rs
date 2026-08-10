use crate::{
    gamerules::{GameResult, GameRules},
    general::{bitboard::Bitboard, rank::Rank, side::Side, square::Square},
};
use std::fmt;

mod zobrist {
    use super::{ChessPiece, Side};

    const PIECE: [[[u64; 64]; 6]; 2] = [
        [
            [
                0x3B4B1390F54F8365,
                0xF55F61FFF36F2EE9,
                0x5AD20553217F76AF,
                0x79D4DDF25E8745CC,
                0x55A7C032C4643798,
                0xA00B1FE7E66F3FEF,
                0x34491F6907F8E731,
                0xC993CFBC23B11B47,
                0xA82C12E6FA190CF6,
                0x113D9839B6F31053,
                0x5A0FBC1FCE12B19A,
                0xF78E7214798AE78F,
                0xE17835DBA4DA05B8,
                0x891BF43D5235F221,
                0xA183C59893413947,
                0x7792FD23BF8DA7E3,
                0x403942090970B2E6,
                0x1E75D0DC630C2FB8,
                0xB1D7D0CE866661B8,
                0x20C88CCE31EAF5E3,
                0x68C089E125B75198,
                0x7A97E903925CE588,
                0x6A532BE44635D087,
                0xCFE23A955742B3B0,
                0x5B66E1B457FCD5CF,
                0x09E6753918D2081E,
                0xB104946ECB3677AD,
                0x502FDF7B02A35617,
                0xD57A0955984F6236,
                0x06CA35D693D53CED,
                0x3B17D02E417280D8,
                0x446B1DE5E450E64E,
                0xC50857683210E2C6,
                0xD669C56B6D6093B6,
                0xE7C8D487BB61218F,
                0xBAF56FD546C78B30,
                0x2665EC0B5503F6BE,
                0x0D8895AD999F82F7,
                0x5164DCF2E171AD5E,
                0x9A0B3DE3CD82D69E,
                0x6619504616C74A33,
                0xA1A46DE060610185,
                0x60F776AD5074A696,
                0xE371DB34FC0C3F22,
                0x781CD995304FAB1A,
                0xEF2AEC955B5FBD96,
                0x803B4981168F13AE,
                0x8F532E2DCE17757A,
                0x2DD0A71CBE61C4A3,
                0xB5CD20CD6711953F,
                0x1AC9FA5DDE52E75E,
                0xBEE55701FE8D4EFA,
                0x167D2CB2446A1935,
                0x92897D734C520BF0,
                0x9EAA0140B0DDDB3D,
                0xC9D3900D05895A93,
                0xC3F37BDF7ABFB521,
                0x5CF688FFA520B117,
                0xE3CFA15B889B166E,
                0x077F3E2556699037,
                0x16FD6951F8392D83,
                0xDCCDBAED53DE9410,
                0xBB86AAFF33779D4B,
                0x749C039DB372FF77,
            ],
            [
                0x1707B48E93440213,
                0xD235689DA6E7AA90,
                0x0CB667DC4DE7915A,
                0xCF41FD7A09DE3484,
                0xCC25A856296EEAC4,
                0xCA895A1D3CD3BABF,
                0x732D411CE615AD20,
                0xDCF558C285F159DF,
                0x951D73ED5BF2D01E,
                0x7B20D18CBDB7EB4A,
                0xABFF6631689D8E94,
                0x2239EB60512A9128,
                0x42FAB5A58494DDEE,
                0xC524AA0AB3575B29,
                0x8D3D54F070F790D1,
                0x77A00EE859D0F308,
                0x9DE593349FCA8E52,
                0x85EB62EB4B228DB5,
                0x4E67E68AC08C1263,
                0xED066D5C72C31826,
                0x1454BACC57DC76EE,
                0xF968982CD3EB98B0,
                0x53954BA5C93F0488,
                0xAB72CBD7982B69F5,
                0xE8269B0DA5562FAD,
                0xCBEDA1F7FAF341CA,
                0xF69355D354A3ACE8,
                0xF12289B4BB70B61E,
                0xA71324B8C3B966DB,
                0x1ADC6C02194B3071,
                0x2A9AC597C89E39D7,
                0x9B7B1305FE1C9AFA,
                0x5CEE589E7B304944,
                0x1C9A18CA287A042C,
                0x12795781E8F099E4,
                0xD9D403CC3BAAB968,
                0xF7F54BA1243F83B2,
                0x463367BA92D4A50A,
                0xA25A68BA752EC1C0,
                0x7AFDACADC960F2F0,
                0xD52974386757628C,
                0x294629413D1F9371,
                0x070388CF15118FBA,
                0x2C208863F9AFB3FB,
                0x2FCD09C4869C43AC,
                0x2BBC9274468C7137,
                0xCE6B2822BABDDD6B,
                0x5854C7A4A3C4937B,
                0x5C5AA2A54C714B96,
                0x69BCDDF3A5A4C345,
                0xC45ECA2B050AFB60,
                0x9FE98F83B8A52458,
                0x8C8EBE5A20925CF2,
                0x9FFD4EEB75E16F3D,
                0x88235C4604555652,
                0x07788052264DEB0D,
                0xE34A98041592FB0B,
                0xA6CCD904568C4CD6,
                0x9A3C5B21B0F23DF6,
                0xED38853FC962C7D3,
                0xCB2FECFF3E3DADFE,
                0xBD4EEE4EE1B655D4,
                0x94F2DA85AB20BA83,
                0x2C685222CF4D3FA6,
            ],
            [
                0xBA7D2903186E8739,
                0xC9B36B122B7DE14B,
                0x5AE075474ABAF47A,
                0xE4844F8CCB6BEDB8,
                0xBD6CFF35D610F6DF,
                0xAA2C65E5F72B66C8,
                0x39170CD5D7886FBD,
                0x77B59EB650ECBAF9,
                0xEE0AC3CCBB51A7E3,
                0xC4767C0023879AFD,
                0xBE3F72717E66B134,
                0xE5C3852BF21C24A7,
                0xF360665D6E07A619,
                0xA304BF6ADF1642D2,
                0x54E9E724F174C12B,
                0x90C46643D4C1458E,
                0x630069062DE3C6F0,
                0x320B45B6A15A31B9,
                0x8E6534E36A23F69C,
                0x073C459874C930E0,
                0x1431520EA29584BB,
                0x458EE9AA0A9CC7A2,
                0xF2576AE121EAFB59,
                0x5DCAB5012CA58195,
                0x3E4436A2BB9ED25A,
                0x215B30A980C694EF,
                0xC05431CBCE7A2A3B,
                0x95B79E41C9EDF5AA,
                0xCF81FFA1D4A41D25,
                0x342BA7F07677F300,
                0xF309646CA0478A99,
                0x24F871A4EFEFE400,
                0x78F7C8CD26073F09,
                0x41774E4BD39745F0,
                0x72E4078175237674,
                0xB391567269461CB3,
                0x97C25550FFF699E3,
                0x7D332C2092FE55CD,
                0xDFAB77A132D2BF30,
                0x349ACC0B0A8A023F,
                0x4B7B9190428579C3,
                0xF97FFB6BA6A90C85,
                0xEA949E47A1FDDE52,
                0x082BDC0719268136,
                0xD6C21690F9345AE7,
                0x7FB55752F27D5D11,
                0x8599D3F6CFD8FED3,
                0x1FCDCBE2AF927B46,
                0x1AA104141DABE2DD,
                0x301D6B437A291FEE,
                0x4C76B263F14A1A1F,
                0x45974B3D6F1322F1,
                0xCE37F9D1CA095570,
                0xBE66665F4E10300D,
                0x7B91DAF747E5B263,
                0xDA51CFAB43CD7293,
                0x2E84537E3C961000,
                0xE8B467C76D798289,
                0x97F9B20CA9396125,
                0x096DCE9B2E8DCC6B,
                0x6CC56F50331AFFD7,
                0x4A60B65129693FF4,
                0x0B033E6464141E80,
                0x2867BE6598D150FE,
            ],
            [
                0x6CFDC01010B1D4B2,
                0x1BAC57AFEC7F73FA,
                0x7328C13425B469EA,
                0x255609A7B9D856D1,
                0x0C281A0CEAD9E652,
                0xB2E6A5FCA6A493E3,
                0xA14C4EC9AEDCBCC8,
                0x9FA81E87CCDB75D6,
                0x36B598DDF8E0B5E5,
                0x4904DEC5F2291680,
                0x47EAA1CB170EAC9E,
                0x57AE84F7480E5D1B,
                0x70C9E3F5426F08AF,
                0x867BAD4FACAEBF4F,
                0x32AE96AF20304C71,
                0x79240BB5C13C80DC,
                0xDAF6C459EF2C41C2,
                0x98292CE739AECED2,
                0xF212F3D910CB3A18,
                0xCBB3873BA4B19224,
                0x9B7B196A59665F33,
                0xD01C0BC07E591495,
                0xA09009325047D5D3,
                0x0E8738658A15AC04,
                0xA058E2483F1BDA10,
                0xFE2C697DA06522CC,
                0x5ADED13F879ED49F,
                0xEDA8CBF840481888,
                0xE6DD2ABAACBAF728,
                0x0FD184D83E653F7F,
                0x3BBC14E1E203DE36,
                0x2B8805E042D6EBCE,
                0xAE4A407B432D79A2,
                0x942A7FC45B0FE40E,
                0xECA75EAB0A452ED2,
                0x9F7A4B38A5C8FAD7,
                0x6AEC0ADC04D5088F,
                0xA036EDB337FBC3E2,
                0x6E192C4913EED672,
                0x6E1EBE9E3A11A36E,
                0xEEDA066B04CC3271,
                0x3C0358A7D8D81A3B,
                0xE9F91A71C3E7312F,
                0xBFFFCDD1976A00D5,
                0xD5CA6AE80DC54689,
                0x2DEB8C8B2171DEFF,
                0x388551174CC9308C,
                0x2FCF8342D3F3BFE6,
                0x0EFFDE8C4869E351,
                0x6421F8F33A0B2712,
                0x665665CF259D1A96,
                0xA5C6E66D0816ADBD,
                0x23B3FE8F819DEB65,
                0xB23024D11A6C4E5C,
                0xB59FA30F8815D059,
                0x3221F7B7CE078B19,
                0x2859228E9440421E,
                0x96FE43FF4B1C4CF0,
                0x282BD1C848CF4DD0,
                0x334B958FA6956336,
                0xF05687E4401F52B6,
                0xE0D580160DD74E34,
                0xBE7AEDBCFC259AFA,
                0xA5E16763C0E208CB,
            ],
            [
                0xA949A68E8EA01A12,
                0x56EC9254F0367DEA,
                0x68AB1305DFB53C3D,
                0x49E2727808273BCB,
                0x297E6476B12CCC9B,
                0x17E40705DEDB3067,
                0x83AEED5371C9E5B9,
                0xC06E9F4A01B2A6A0,
                0x45DE2E4280A164DF,
                0x93CA7F8CC086D104,
                0x1B48A438D72DF35C,
                0x646696AB23D274CB,
                0x3157D4CC25B39ABA,
                0xCD16E577CA97A760,
                0x9A1F735A8C2FBE73,
                0x4125E68C32FA24B6,
                0x3052DC63B4E11BAE,
                0x84888868481BA394,
                0x33522318BC37CFA9,
                0x47A0FCC094E23968,
                0x48A83EEEBB6415A1,
                0x90083124F3BCE794,
                0xC9EE0686C1F2A532,
                0xFA784C52B94501F6,
                0xAB635D0B12301C3C,
                0xDA992728456B94C9,
                0x86751D8DF6FF83E6,
                0xD5D6783B59306F50,
                0x22C3F006270122BF,
                0x13B8A0B69D30F5CE,
                0xA6F96EFA8653B213,
                0x88116662018B7248,
                0x0994D68C7DE7709C,
                0x46CDA4DF20632B1C,
                0xAF341FF35F0B016F,
                0xF31C0AAF7593DC74,
                0x225CCE0BB7E15344,
                0xCCBDDB0C5DDF4C39,
                0x19ACEFE98D1F629E,
                0xBB93A469BF85D993,
                0x1EED85189154AB8B,
                0x67196A565C912958,
                0xC539B92D840C419E,
                0x95EE7EE427ED9BF8,
                0xC9A41EA679B5CAFB,
                0xD5D41E73904F6DFE,
                0x3D07E7B91D570021,
                0x7BB68CDFB208414B,
                0x6C3117C3CF646A2B,
                0x3BC2E59EECB517F4,
                0x8A27E84053A2B9D0,
                0x8655A812C29BC3E9,
                0x0977495CACB20ACF,
                0x4C8ED103D648D58E,
                0x8FE9ED0B998F04A1,
                0x466296BCBA7AD7EB,
                0x01D64C7681FDD5B3,
                0x9EAF7A5440C6831F,
                0x24BC5C05E34D4653,
                0x6DA0F3767B3D69DF,
                0xB817C16DB16AC4A5,
                0xAF62CE9186FD38A5,
                0x57B3B9CF7A516C67,
                0x25D4C1422F566C5C,
            ],
            [
                0x91BA99433BD33FCF,
                0x511C6582599C2E65,
                0xF5D286849158851F,
                0x4A8F8A6D13EC3788,
                0x98ADC2E4115EADB8,
                0xCEBBDF1D426AFAC2,
                0x53B07135DFFBCC82,
                0xFF08788103770ADD,
                0xDCDBAC855CC74490,
                0x3C9C5332905B6936,
                0xF80E9A2DB8FA7805,
                0x7D0E5B8220FF144D,
                0x39C1EFE4589E6780,
                0x0559E92EB4F211FF,
                0x39D84EEB95C708AE,
                0x31B828A6F39901BD,
                0xCCA1BFC338093A34,
                0xB479232BC5DC4A66,
                0x1739238308DFE36E,
                0x6B3BE6A43B808BB2,
                0x396355B380CEC5D4,
                0xC9E6A69C84079D65,
                0xA7037C14BAB92E77,
                0x783AD19BDE071CEB,
                0x885882CDB6DC5B2B,
                0x5E904312F586A6D3,
                0x6F906085BDA0859F,
                0x62E0F24CCC9AC4C5,
                0xD35BD20522EF3D98,
                0xF548B985D12B4044,
                0x11EB601DC7628DA8,
                0x93CDC3CEDC050B20,
                0x60F292C0D67F2EB3,
                0x359A74F896FFEC3E,
                0x00934D3D7E952AD2,
                0x781A8E3A094F1B65,
                0x807BF51BDB1E0BF2,
                0x0BF245716ABF1EA7,
                0x151C03FD261184FC,
                0x375E62429AFC82C5,
                0x19A7C9859974FADB,
                0x8C888AECBF0AC252,
                0x9D1BC5C50D96DFB1,
                0x95639EF8ABCEA651,
                0x5120141E640884FB,
                0xE8AF292808E6D022,
                0xE4F2C4280B4755F1,
                0x1DAAE2E20D8A0A36,
                0xEE19817C19090726,
                0x804028CCAD115452,
                0xCE09B47943E128F7,
                0xB7208F3965865AD3,
                0x81E78D6B24F99F81,
                0x390DAE32251FFEFF,
                0x9BC6E7F0F02DBBF0,
                0xBFC65AC3CC80408F,
                0x62141DAFE21CD9E0,
                0x14568F3E940240AC,
                0x37D7E32E9ED93418,
                0x206E3A9F52ABA9A4,
                0x05F360754F19DB35,
                0x30ECA08459647C4B,
                0x05F7B76610EF1B1F,
                0x0C36BADE05E76A51,
            ],
        ],
        [
            [
                0x095F1D07CF13BC8E,
                0x197717F7B9C7EE6A,
                0x291AF9C95779E52D,
                0xA42CA1EC7FECE53B,
                0x00496FF003180950,
                0x6D619782259B603D,
                0xDC7F759AFF4F93C8,
                0x0CEB5CD438F960C0,
                0x59D2FE4B892E1558,
                0x602187B2C016FD02,
                0x273CE3805DB27A6C,
                0x901FCDEC04EFC0D0,
                0x760762333646AFD1,
                0x291CED133B3020B6,
                0xC0B0D250C30702EC,
                0x65BD0EB2DBC8A1BE,
                0xDE15C4B33EC7DD49,
                0x5FDA9F6CE550B8BB,
                0x6C48461E73193765,
                0xD3A35D1079186CCD,
                0x4EC05124DA77519E,
                0x097DFBC58F6DE725,
                0xEF871A323D65D150,
                0xF1087152280A7EE0,
                0x3BFD7559B73464C2,
                0x4E3CD12FFE7796B0,
                0xB8BB384E2F856E19,
                0xAA88F1641547C631,
                0xE0754587CB693F55,
                0xF102968AA3C6B0ED,
                0xEE0C555C5FD7D048,
                0x5CF60623A08EE51D,
                0xBCD67D376F6EF741,
                0x3B2D027D4DD7F6C4,
                0x541F760715359A2E,
                0x2E942DCB98E81698,
                0x697338673C7CF487,
                0xDA509E94E99537D7,
                0xD80539D9F42CA198,
                0xF021FFB15582ED43,
                0x0565FF07075829B5,
                0x104A458FB925C6E6,
                0x74C901CFF432CFCB,
                0x4B0ECBD4B7A538DE,
                0x00A4B6FC2245952A,
                0xBBA67813E74F39EF,
                0x67B8CB5101A8045F,
                0xD30CF74D700E36AB,
                0x0B71A95C9906ECDE,
                0x02B903858D47CEC4,
                0xD63431D89B894CF2,
                0x9D3903D23AAF69C7,
                0x795D54E6B7B2D006,
                0xCC635F037974839A,
                0x326DFCE9DC67B0D1,
                0x56B44F5A4BDD39E6,
                0x4546D8C13D9619B0,
                0xF81D61DE9B32E660,
                0x84B1A5D633750C71,
                0x1385BF9DE4CDCAC5,
                0xB6F1962FA6A44E9B,
                0x3B8C754623AFCA7A,
                0xDED06C882DEA7B8B,
                0x583A064C7923D076,
            ],
            [
                0x4FCCE7D530516B14,
                0xE0E6ED81FE36F91B,
                0x4B046A5925D68F30,
                0xD85CDF14B9C771BC,
                0x6522F36C0F290DE7,
                0x8C8700945F68EA1A,
                0x7269877B34ECE64A,
                0xB7B72EC365C458FE,
                0x15718DF24A4D01C2,
                0x3C594BD84CD71FCD,
                0x328E8FF76BD47030,
                0x232609F0C5BD254D,
                0xC911050AA0F0AF78,
                0x005FE033C5196FA7,
                0x5DD12E4883186D39,
                0xF965BAE895024827,
                0x89700BD272F29C30,
                0x7D0B9A1A8D22C7ED,
                0x2D455CDBEC251180,
                0x05B61239696F8A93,
                0x784B1D3C796705F7,
                0x31CCF5B8639E6A87,
                0xA8EE0A550952EB69,
                0x4DC247C3B408DE47,
                0x44B5155ED7ACD6D4,
                0x58FCFD0A0D50686B,
                0x638F2A440BBB0A57,
                0x3E3537AEA3BDEC8A,
                0x008BAA036379CC59,
                0x90AA7E0CDE9C0CBE,
                0xD2FD332650B4FB68,
                0x0C5D670055D6C970,
                0xA550A87DE5944EC1,
                0xF1C090959659259D,
                0x81668A8DC19F157A,
                0x60869AA800245A55,
                0x1E1FD28270A8C181,
                0xD5921564E92277F0,
                0x00C05202C35F696F,
                0x5657F52DDA0DB720,
                0xC597BB43E74B1B86,
                0x53E50A5FA608FFF6,
                0x89B72CFE3C3BA250,
                0x42016A4869BF8CF8,
                0xE8484198C407580E,
                0xE10B811C10115078,
                0x5695DE5E113E14DB,
                0x1D50FD2629017B62,
                0x5CB6F71792B34298,
                0x268CD38E9DED694D,
                0x0ED4E606D387DA52,
                0xD317B7215CECDD4E,
                0x35AB44A0ABCFD616,
                0x9718039C1148BDAD,
                0x0B7D3EAAB08D0AB2,
                0x188DCFBE986A0006,
                0x68AE70191C59C955,
                0xB885262B04239167,
                0x3656F89F21808FA4,
                0xAC98E5B8CD15B793,
                0x09326DA065ED2B9C,
                0x354D5D10FCD92F8D,
                0x31EC0AC37660AD68,
                0xDA74612BF36BDDE9,
            ],
            [
                0x8F66F9B5F93EE7D8,
                0x17ACB0371AFE498D,
                0x52E4749C241E45A8,
                0x46895304984D4AE4,
                0x927438D37B242232,
                0xC24CD956A11CDC4D,
                0xC439F4AE111E9AF4,
                0x845A086E79A9FDA2,
                0x1BD62C7B9171BDD6,
                0xA936184050954731,
                0x4E43CFBB9711CB7C,
                0xDC72DF0F3992A639,
                0xEF8CF19C663F3A6C,
                0xF109F6E9C387F92B,
                0x4312363077FCEF4D,
                0xF50D06560BCB5158,
                0xF1B02B7B13B062DB,
                0x62166832D185605C,
                0x0A1F1811CC50F824,
                0x5D44D3C0CFD0091A,
                0xF35662ABEF2D9848,
                0x2310235C69CD9488,
                0x1F6D2AF7F8ACD2A9,
                0x369E1360FC92762B,
                0x65F9029FF4BABC79,
                0x9C9684D926687D66,
                0xA1FAB5E037300E3F,
                0x9CB893CD83D4B23A,
                0x9765114E10DBD0C8,
                0x24A721DE49DF08FC,
                0x13E35FFDEE9DE383,
                0xDD66AB519FE61392,
                0xE0A3BAE4A99F5D79,
                0xD687796F942009CB,
                0x1AFBEA0F6DCC99CD,
                0x117B5BCFD6FC753B,
                0x22ECA8C97660C94D,
                0x65B444058E13E36B,
                0x70E7E9FF14997B2F,
                0x3D865D434068EE06,
                0x1028AFB5ADE5F706,
                0xE6089018146832AD,
                0x1D8D0FDFB8B801AE,
                0x3AAEA18AECC9017C,
                0x8BAA3CF161DB70D4,
                0x3451779336274244,
                0x1F5E343E9A1D6129,
                0xB850416D82314AEC,
                0x448D300677C4FF89,
                0x00932AC186DE965E,
                0xE3BDFEF31FD5C72F,
                0xE2E4811DF7A7AB7D,
                0xBB80B8A2D17C21C1,
                0x97F3929706A966CE,
                0x5F5B26E86EB9EFB1,
                0xCCE5FC8175091D9C,
                0x21720491DAD77F39,
                0x8A7CFF40EA059437,
                0xFC964C23F80DC381,
                0x6C4B5FD55E7B3F81,
                0xA42ABADE7A94B4F0,
                0x6985299EF262DB40,
                0x13B3CA1FA294DF76,
                0xA9B7CA00E8332487,
            ],
            [
                0x8EE1DCAF3E54D0CB,
                0xA9C7002555C1AD60,
                0xF83D81C4BF4521D5,
                0xA5B9FCE1E6C53A8E,
                0x70E6CD8C304A0B70,
                0xE0478EA55C9F08C9,
                0xD794DC7CE739863B,
                0x1627A172D3D3A61F,
                0xD0569458AFA28C5F,
                0x400CF0C8F3AB4642,
                0x291C39A088F198A0,
                0x6DBF354344BD0DC6,
                0xC2B678262E4CCAE2,
                0x9A5EF02F6D11E151,
                0x8E4B458B73AC8B06,
                0x8F1AC8BD1B4C0282,
                0xC14E12340209F825,
                0x6EE8A486731EB1D6,
                0x3D7E829D962AADED,
                0x8A1D48DCB7ACE182,
                0x7FE08FE1D9192843,
                0x9FE481F4E0321881,
                0xED7B15F267C14F51,
                0xDEC7E909775A6947,
                0xCD05DCF442B8CFBC,
                0x7AC7BBFBA8BB53BE,
                0xFC21858F6EDC24F9,
                0xAD197288FEEA57E4,
                0x05131557F27C2162,
                0x5DF54C0D9110E4BC,
                0x91B9089C1AE9BDBB,
                0x05C274879AFF87A5,
                0x6AEAC9E1C9E4C72F,
                0xF4BAF207E6079351,
                0x6552B97A9B2BB058,
                0x730051B3E0BA8F27,
                0x21571B769C9F58F1,
                0x597B4F28E4C7F605,
                0xC652DDE885B22175,
                0xDC431FA47F09F9B8,
                0x16258532067C74CD,
                0xFC7BD58BF247B30C,
                0x970E96986AA03D56,
                0x25EF72CAD0922C86,
                0xF0C9A0D6B68F91DC,
                0xDA5FD2700F12FEC5,
                0x5B283883491ACFF9,
                0xA754946D6455D7FB,
                0x2DD77C7FEC97CCD6,
                0x2078F5F6620F6143,
                0xD2A5E994ECD708DC,
                0xE78A298D4CEF7BEA,
                0x33CFDD73EC31A967,
                0xBE054C6B57F6968A,
                0x295E0DF903472E85,
                0xAE69F45B4C701441,
                0x14AFA67FE8F4674E,
                0x332B10910F615236,
                0x19E6502FDBBCB409,
                0xDF93264E423A1826,
                0x2DE218D4144E35B8,
                0x5BB95980537A66EC,
                0xB8688FD1C87F196F,
                0x97D3B5A0A1DCCE11,
            ],
            [
                0x36F810FA7DC63C29,
                0x9FB88E5180B6B67A,
                0x0FD015E64508D26A,
                0xF705F64E9DC63FA3,
                0x9A75776A210A928B,
                0x1E5FE6A5F840622B,
                0x5799D37A134D3D18,
                0xD553080D490A0DEB,
                0xE350D94ECD473247,
                0x77E6E7698D552440,
                0x2AEB976D3D563B44,
                0x08799F44116AA53C,
                0x02B770547909D478,
                0xF3036F5DFE58A9B2,
                0x33B0DBD8D8A303A5,
                0x17D90A344DDFABBC,
                0x28285A0243B63FB0,
                0x29A15051CA01477E,
                0x7D8464CAE1598213,
                0x7665716D7A109252,
                0xFEB93BA77FDD8463,
                0x9BD72300BEBAB9D5,
                0xE5252BD7E8FB6109,
                0x75840506B71D47FE,
                0xCFDF7BA1CA8C841D,
                0x0752FDF57F3516F3,
                0x94285B84ED5F82E3,
                0x0DDF67451F036009,
                0x3C9ECF0FB3BD2487,
                0x85B9E15C9FE7FDFA,
                0x2719AEE614B28FF9,
                0x4ABA1C4CCC2A8063,
                0x454E5782DD5BDE89,
                0x7C644E5196EA5C8D,
                0xE9737EDFE49A618D,
                0xCA5698D77DB28334,
                0xCFDD86B92A12618C,
                0xDE794ED2169DF712,
                0xE5394C5A85CAA697,
                0x8D737AE023F97389,
                0xEA3B2CD1E442EA7D,
                0x4C6F22DF04A10BB8,
                0xC90F2F56CAB39054,
                0x375997604E3D5DDB,
                0x28713F454F0A94F8,
                0x04DFCB5F333AA223,
                0x966E23DDDD8709AF,
                0xB099B6F105390BB4,
                0x0BF9FC92D78757D6,
                0xEC342AA738D1E0DC,
                0x34E02E567FA4889B,
                0x0251C6A8144E6DBF,
                0x323627379F2F67FA,
                0x1A9E2920CF27C5F4,
                0x169FF73844A9F928,
                0xE8DAA34D2C9A93E7,
                0xBFE43A98FB7D70DD,
                0xD7C5B0DB03BE5803,
                0x9F9B88E41668E8D9,
                0x2679BD32E60B3875,
                0x5D4BC0F8BED0E056,
                0x6A5080F2796E6F31,
                0x65434C3C3167A684,
                0x292D993C90E402A6,
            ],
            [
                0x4B51420164C6F6E1,
                0xEF2D82918EF6B348,
                0xE8D74A34E622D0A5,
                0x762239871214662F,
                0x730CB868E1EFFE48,
                0x0BEAE065298DAC2F,
                0xC82C06E21F6FAA78,
                0x56F01ADFCDDAE5FE,
                0x31EDE66C95EDD935,
                0x6C4494C11268F0AB,
                0x806A15E2CECA987D,
                0xA157A09C63597984,
                0x1621A3A6D9A8C182,
                0x598DF86148D6068D,
                0xA163DAFFBEF451C2,
                0xB8279305D6EC2515,
                0xB75EF7EB1DD57B4E,
                0x5E064A536C8097C3,
                0x9A5568D60BEEA044,
                0x21DF2319A5A8C341,
                0xA930D1D3E6440654,
                0xD26C04531AD7BAA1,
                0xC4B5503881047670,
                0x213A6266FD05E797,
                0x612BBB8444858D7F,
                0x87827A11A262AF55,
                0x449450343FE9D17A,
                0x00FF0334F64D18E5,
                0xE1F6F7F3116ACDBC,
                0x25E987A45C17AC03,
                0x4BCCA7C33CCE9C30,
                0x30E64B55F6F6FE1F,
                0x5A457904056D9B30,
                0xC22A18AC4058B35E,
                0x3CCFF011C7DE8781,
                0xFC07B353EFFD146E,
                0x5106AB0DB3AEB8DC,
                0x23B5A59C8C0AC332,
                0xF009A1A19610A9BE,
                0x76F0188F84CCA906,
                0xBE000D22A2F8CC10,
                0xE6D8F3D29AFBDC9D,
                0x50CA0B3CB111258A,
                0x5CE92FA9AFBC3943,
                0xB10047354EAF687D,
                0xB0F1D2C173973149,
                0x2E3C3E06997C968F,
                0xC86F2646BFA11A4E,
                0x4C0C78AB801023B9,
                0x34F524009C32872C,
                0xF8A46F41C8ECE8D8,
                0xD75C2CBB464C3DCD,
                0x45DB3C80D72B9B17,
                0x56A1F76C86499D39,
                0x264D5275F3924E85,
                0x7090657C5E94EC88,
                0x94F4D4D06F45B89C,
                0x3ACC7D43FB1A549A,
                0x85D3AA87DA6A77DC,
                0x6CF6ABE0C9D34F5C,
                0x438F87931FA138D2,
                0xE8828D54AB116A3D,
                0x61243DA67C33D357,
                0x64712615783C10F0,
            ],
        ],
    ];

    const CASTLING: [u64; 4] = [
        0x4807DDCCADE0A0C8,
        0x7D113ADA24F1A982,
        0x286CFEF1DC95FC71,
        0x84E9663358CE3A15,
    ];

    pub const EP_FILE: [u64; 8] = [
        0x4C686F44ED923232,
        0x81AC7790B3E8B3F3,
        0xA14EF33562FD83DC,
        0x44115F80BDE9EA92,
        0x8192A3760D4A335A,
        0x5F3D124F973D9631,
        0xE86931D5D3E533DA,
        0x92A564662816E59E,
    ];

    pub const SIDE_TO_MOVE: u64 = 0x2A074F3991939B18;

    pub const US_KSC: u64 = CASTLING[0];
    pub const US_QSC: u64 = CASTLING[1];
    pub const THEM_KSC: u64 = CASTLING[2];
    pub const THEM_QSC: u64 = CASTLING[3];

    #[must_use]
    pub const fn piece_key(side: Side, piece: ChessPiece, square_index: usize) -> u64 {
        PIECE[side as usize][piece as usize][square_index]
    }
}

#[derive(Clone)]
struct Snapshot {
    board: Board,
    halfmoves: i32,
    fullmoves: i32,
    us_ksc: bool,
    us_qsc: bool,
    them_ksc: bool,
    them_qsc: bool,
    hash: u64,
}

#[derive(Debug, PartialEq, Default, Clone)]
pub struct Board {
    colour: [Bitboard<8, 8>; 2],
    pieces: [Bitboard<8, 8>; 6],
    ep: Option<Square<8, 8>>,
    pub flipped: bool,
}

#[derive(Default, Clone)]
pub struct ChessPosition {
    pub board: Board,
    pub halfmoves: i32,
    pub fullmoves: i32,
    pub us_ksc: bool,
    pub us_qsc: bool,
    pub them_ksc: bool,
    pub them_qsc: bool,
    stack: Vec<Snapshot>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChessMove {
    pub from: Square<8, 8>,
    pub to: Square<8, 8>,
    pub promo: Option<ChessPiece>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ChessPiece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl ChessPiece {
    pub const ALL: [ChessPiece; 6] = [
        ChessPiece::Pawn,
        ChessPiece::Knight,
        ChessPiece::Bishop,
        ChessPiece::Rook,
        ChessPiece::Queen,
        ChessPiece::King,
    ];

    #[must_use]
    fn to_char(self, side: Side) -> char {
        let c = match self {
            ChessPiece::Pawn => 'p',
            ChessPiece::Knight => 'n',
            ChessPiece::Bishop => 'b',
            ChessPiece::Rook => 'r',
            ChessPiece::Queen => 'q',
            ChessPiece::King => 'k',
        };
        match side {
            Side::Player1 => c.to_ascii_uppercase(),
            Side::Player2 => c,
        }
    }
}

pub enum Castling {
    WKS,
    WQS,
    BKS,
    BQS,
}

fn try_pawn_move(
    from: Square<8, 8>,
    to: Square<8, 8>,
    func: &mut impl FnMut(ChessMove) -> bool,
) -> bool {
    if to.get_rank() == Rank(7) {
        func(ChessMove {
            from,
            to,
            promo: Some(ChessPiece::Queen),
        }) || func(ChessMove {
            from,
            to,
            promo: Some(ChessPiece::Rook),
        }) || func(ChessMove {
            from,
            to,
            promo: Some(ChessPiece::Bishop),
        }) || func(ChessMove {
            from,
            to,
            promo: Some(ChessPiece::Knight),
        })
    } else {
        func(ChessMove {
            from,
            to,
            promo: None,
        })
    }
}

impl ChessMove {
    #[must_use]
    pub fn flip(&self) -> Self {
        Self {
            from: self.from.flipped(),
            to: self.to.flipped(),
            promo: self.promo,
        }
    }
}

impl Board {
    #[must_use]
    pub fn get_us(&self) -> Bitboard<8, 8> {
        self.colour[0]
    }

    #[must_use]
    pub fn get_them(&self) -> Bitboard<8, 8> {
        self.colour[1]
    }

    #[must_use]
    pub fn get_occupied(&self) -> Bitboard<8, 8> {
        self.colour[0] | self.colour[1]
    }

    #[must_use]
    pub fn get_empty(&self) -> Bitboard<8, 8> {
        !self.get_occupied()
    }

    #[must_use]
    pub fn get_pawns(&self) -> Bitboard<8, 8> {
        self.pieces[ChessPiece::Pawn as usize]
    }

    #[must_use]
    pub fn get_knights(&self) -> Bitboard<8, 8> {
        self.pieces[ChessPiece::Knight as usize]
    }

    #[must_use]
    pub fn get_bishops(&self) -> Bitboard<8, 8> {
        self.pieces[ChessPiece::Bishop as usize]
    }

    #[must_use]
    pub fn get_rooks(&self) -> Bitboard<8, 8> {
        self.pieces[ChessPiece::Rook as usize]
    }

    #[must_use]
    pub fn get_queens(&self) -> Bitboard<8, 8> {
        self.pieces[ChessPiece::Queen as usize]
    }

    #[must_use]
    pub fn get_kings(&self) -> Bitboard<8, 8> {
        self.pieces[ChessPiece::King as usize]
    }

    #[must_use]
    pub fn get_bq(&self) -> Bitboard<8, 8> {
        self.get_bishops() | self.get_queens()
    }

    #[must_use]
    pub fn get_rq(&self) -> Bitboard<8, 8> {
        self.get_rooks() | self.get_queens()
    }

    #[must_use]
    pub fn piece_on(&self, x: i32, y: i32) -> Option<ChessPiece> {
        let sq = Square::<8, 8>::from_coords(x, y);
        self.piece_on_sq(sq)
    }

    #[must_use]
    pub fn piece_on_sq(&self, sq: Square<8, 8>) -> Option<ChessPiece> {
        ChessPiece::ALL
            .into_iter()
            .find(|&piece| self.pieces[piece as usize].is_square_set(sq))
    }

    #[must_use]
    pub fn side_on(&self, x: i32, y: i32) -> Option<Side> {
        let sq = Square::<8, 8>::from_coords(x, y);
        if self.colour[Side::Player1].is_square_set(sq) {
            Some(Side::Player1)
        } else if self.colour[Side::Player2].is_square_set(sq) {
            Some(Side::Player2)
        } else {
            None
        }
    }

    #[must_use]
    pub fn piece_hash(&self) -> u64 {
        let mut hash = 0;
        for side in [Side::Player1, Side::Player2] {
            for piece in ChessPiece::ALL {
                for sq in self.colour[side] & self.pieces[piece as usize] {
                    hash ^= zobrist::piece_key(side, piece, sq.get_index());
                }
            }
        }
        hash
    }

    pub fn toggle(&mut self, sq: Square<8, 8>, piece: ChessPiece, side: Side) {
        self.colour[side] ^= Bitboard::from_square(sq);
        self.pieces[piece as usize] ^= Bitboard::from_square(sq);
    }

    pub fn set(&mut self, sq: Square<8, 8>, piece: ChessPiece, side: Side) {
        self.colour[side] |= Bitboard::from_square(sq);
        self.pieces[piece as usize] |= Bitboard::from_square(sq);
    }

    pub fn remove(&mut self, sq: Square<8, 8>, piece: ChessPiece, side: Side) {
        self.colour[side] &= !Bitboard::from_square(sq);
        self.pieces[piece as usize] &= !Bitboard::from_square(sq);
    }

    #[must_use]
    fn is_empty(&self, sq: Square<8, 8>) -> bool {
        !self.get_occupied().is_square_set(sq)
    }

    #[must_use]
    fn is_attacked(&self, sq: Square<8, 8>) -> bool {
        let bb = Bitboard::from_square(sq);

        let pawn_attacks = bb.ne() | bb.nw();
        let knight_attacks = bb.north().north().east()
            | bb.north().north().west()
            | bb.south().south().east()
            | bb.south().south().west()
            | bb.east().east().north()
            | bb.east().east().south()
            | bb.west().west().north()
            | bb.west().west().south();
        let bishop_attacks = bb.ray_ne(self.get_occupied())
            | bb.ray_nw(self.get_occupied())
            | bb.ray_se(self.get_occupied())
            | bb.ray_sw(self.get_occupied());
        let rook_attacks = bb.ray_north(self.get_occupied())
            | bb.ray_south(self.get_occupied())
            | bb.ray_east(self.get_occupied())
            | bb.ray_west(self.get_occupied());
        let king_attacks = bb.adjacent();

        (pawn_attacks & self.get_pawns() & self.get_them()).is_occupied()
            | (knight_attacks & self.get_knights() & self.get_them()).is_occupied()
            | (bishop_attacks & (self.get_bishops() | self.get_queens()) & self.get_them())
                .is_occupied()
            | (rook_attacks & (self.get_rooks() | self.get_queens()) & self.get_them())
                .is_occupied()
            | (king_attacks & self.get_kings() & self.get_them()).is_occupied()
    }

    #[must_use]
    fn get_attackers(&self, sq: Square<8, 8>) -> Bitboard<8, 8> {
        let bb = Bitboard::from_square(sq);
        let blockers = self.get_occupied();
        let mut attackers = Bitboard::default();

        // Pawns
        attackers |= (bb.ne() | bb.nw()) & self.get_pawns();

        // Knights
        attackers |= bb.knights() & self.get_knights();

        // Bishops & Queens
        attackers |= bb.rays_diagonal(blockers) & (self.get_bishops() | self.get_queens());

        // Rooks & Queens
        attackers |= bb.rays_hor_ver(blockers) & (self.get_rooks() | self.get_queens());

        // King
        attackers |= bb.adjacent() & self.get_kings();

        attackers & self.get_them()
    }

    pub fn swap(&mut self) {
        // Swap colours
        self.colour[0] = self.colour[0].swap();
        self.colour[1] = self.colour[1].swap();
        (self.colour[0], self.colour[1]) = (self.colour[1], self.colour[0]);

        // Swap pieces
        self.pieces[ChessPiece::Pawn as usize] = self.pieces[ChessPiece::Pawn as usize].swap();
        self.pieces[ChessPiece::Knight as usize] = self.pieces[ChessPiece::Knight as usize].swap();
        self.pieces[ChessPiece::Bishop as usize] = self.pieces[ChessPiece::Bishop as usize].swap();
        self.pieces[ChessPiece::Rook as usize] = self.pieces[ChessPiece::Rook as usize].swap();
        self.pieces[ChessPiece::Queen as usize] = self.pieces[ChessPiece::Queen as usize].swap();
        self.pieces[ChessPiece::King as usize] = self.pieces[ChessPiece::King as usize].swap();

        // Swap ep
        if let Some(sq) = self.ep {
            self.ep = Some(sq.flipped());
        }

        self.flipped = !self.flipped;
    }
}

impl ChessPosition {
    #[must_use]
    pub fn can_castle(&self, perm: Castling) -> bool {
        match perm {
            Castling::WKS => self.us_ksc,
            Castling::WQS => self.us_qsc,
            Castling::BKS => self.them_ksc,
            Castling::BQS => self.them_qsc,
        }
    }

    #[must_use]
    pub fn in_check(&self) -> bool {
        let ksq = (self.board.get_kings() & self.board.get_us()).lsb();
        self.board.is_attacked(ksq)
    }

    #[must_use]
    pub fn hash(&self) -> u64 {
        let mut hash = self.board.piece_hash();

        if self.us_ksc {
            hash ^= zobrist::US_KSC;
        }
        if self.us_qsc {
            hash ^= zobrist::US_QSC;
        }
        if self.them_ksc {
            hash ^= zobrist::THEM_KSC;
        }
        if self.them_qsc {
            hash ^= zobrist::THEM_QSC;
        }
        if let Some(ep) = self.board.ep {
            hash ^= zobrist::EP_FILE[ep.get_file().get_index()];
        }
        if self.board.flipped {
            hash ^= zobrist::SIDE_TO_MOVE;
        }

        hash
    }

    #[must_use]
    fn can_move(&self) -> bool {
        let mut found = false;
        self.move_generator(|_| {
            found = true;
            true
        });
        found
    }

    pub fn parse_movestr(&self, word: &str) -> Result<ChessMove, &'static str> {
        if word.len() < 4 || word.len() > 5 {
            Err("movestr wrong length")
        } else {
            let mut from = Square::<8, 8>::from_string(&word[0..2]).unwrap();
            let mut to = Square::<8, 8>::from_string(&word[2..4]).unwrap();
            let promo = match word.chars().nth(4) {
                Some('q') => Some(ChessPiece::Queen),
                Some('r') => Some(ChessPiece::Rook),
                Some('b') => Some(ChessPiece::Bishop),
                Some('n') => Some(ChessPiece::Knight),
                Some(_) => return Err("invalid promotion character"),
                _ => None,
            };

            if self.board.flipped {
                from = from.flipped();
                to = to.flipped();
            }

            Ok(ChessMove { from, to, promo })
        }
    }

    #[must_use]
    pub fn move_to_string(&self, mv: &ChessMove) -> String {
        format!(
            "{}{}{}",
            if self.board.flipped {
                mv.from.flipped()
            } else {
                mv.from
            },
            if self.board.flipped {
                mv.to.flipped()
            } else {
                mv.to
            },
            match mv.promo {
                Some(ChessPiece::Queen) => "q",
                Some(ChessPiece::Rook) => "r",
                Some(ChessPiece::Bishop) => "b",
                Some(ChessPiece::Knight) => "n",
                Some(_) => panic!("Invalid promotion piece"),
                None => "",
            }
        )
    }

    fn flip(&mut self) {
        self.board.swap();
        (self.us_ksc, self.them_ksc) = (self.them_ksc, self.us_ksc);
        (self.us_qsc, self.them_qsc) = (self.them_qsc, self.us_qsc);
    }
}

impl GameRules for ChessPosition {
    type MoveType = ChessMove;
    const WIDTH: i32 = 8;
    const HEIGHT: i32 = 8;

    fn startpos() -> Self {
        Self {
            board: Board {
                colour: [
                    Bitboard::new(0x000000000000FFFF),
                    Bitboard::new(0xFFFF000000000000),
                ],
                pieces: [
                    Bitboard::new(0x00FF00000000FF00),
                    Bitboard::new(0x4200000000000042),
                    Bitboard::new(0x2400000000000024),
                    Bitboard::new(0x8100000000000081),
                    Bitboard::new(0x0800000000000008),
                    Bitboard::new(0x1000000000000010),
                ],
                ep: None,
                flipped: false,
            },
            us_ksc: true,
            us_qsc: true,
            them_ksc: true,
            them_qsc: true,
            halfmoves: 0,
            fullmoves: 1,
            stack: vec![],
        }
    }

    fn set_piece(&mut self, x: i32, y: i32, c: char, _yee: usize) -> bool {
        let sq = Square::from_coords(x, y);
        match c {
            'P' => self.board.set(sq, ChessPiece::Pawn, Side::Player1),
            'N' => self.board.set(sq, ChessPiece::Knight, Side::Player1),
            'B' => self.board.set(sq, ChessPiece::Bishop, Side::Player1),
            'R' => self.board.set(sq, ChessPiece::Rook, Side::Player1),
            'Q' => self.board.set(sq, ChessPiece::Queen, Side::Player1),
            'K' => self.board.set(sq, ChessPiece::King, Side::Player1),
            'p' => self.board.set(sq, ChessPiece::Pawn, Side::Player2),
            'n' => self.board.set(sq, ChessPiece::Knight, Side::Player2),
            'b' => self.board.set(sq, ChessPiece::Bishop, Side::Player2),
            'r' => self.board.set(sq, ChessPiece::Rook, Side::Player2),
            'q' => self.board.set(sq, ChessPiece::Queen, Side::Player2),
            'k' => self.board.set(sq, ChessPiece::King, Side::Player2),
            _ => panic!("Unrecognised piece char {}", c),
        }
        true
    }

    fn get_square_string(&self, x: i32, y: i32) -> Option<String> {
        let ny = if self.board.flipped { 7 - y } else { y };

        let flip_side = if self.board.flipped {
            Side::Player2
        } else {
            Side::Player1
        };

        let side = if self.board.side_on(x, ny) == Some(flip_side) {
            Side::Player1
        } else {
            Side::Player2
        };

        self.board
            .piece_on(x, ny)
            .map(|piece| piece.to_char(side).to_string())
    }

    fn move_generator(&self, mut func: impl FnMut(Self::MoveType) -> bool) {
        let ksq = (self.board.get_kings() & self.board.get_us()).lsb();
        let kbb = Bitboard::from_square(ksq);
        let us = self.board.get_us();
        let attackers = self.board.get_attackers(ksq);
        let in_check = attackers.is_occupied();
        let (ray_n, xray_n) = kbb.xray_north(self.board.get_occupied());
        let (ray_e, xray_e) = kbb.xray_east(self.board.get_occupied());
        let (ray_s, xray_s) = kbb.xray_south(self.board.get_occupied());
        let (ray_w, xray_w) = kbb.xray_west(self.board.get_occupied());
        let (ray_ne, xray_ne) = kbb.xray_ne(self.board.get_occupied());
        let (ray_nw, xray_nw) = kbb.xray_nw(self.board.get_occupied());
        let (ray_se, xray_se) = kbb.xray_se(self.board.get_occupied());
        let (ray_sw, xray_sw) = kbb.xray_sw(self.board.get_occupied());

        let pinned = |xray: Bitboard<8, 8>, ray: Bitboard<8, 8>, sliders: Bitboard<8, 8>| {
            if (xray & self.board.get_them() & sliders).is_occupied() {
                ray & self.board.get_us()
            } else {
                Bitboard::empty()
            }
        };
        let pinned_north = pinned(xray_n, ray_n, self.board.get_rq());
        let pinned_south = pinned(xray_s, ray_s, self.board.get_rq());
        let pinned_east = pinned(xray_e, ray_e, self.board.get_rq());
        let pinned_west = pinned(xray_w, ray_w, self.board.get_rq());
        let pinned_ne = pinned(xray_ne, ray_ne, self.board.get_bq());
        let pinned_nw = pinned(xray_nw, ray_nw, self.board.get_bq());
        let pinned_se = pinned(xray_se, ray_se, self.board.get_bq());
        let pinned_sw = pinned(xray_sw, ray_sw, self.board.get_bq());
        let pinned_ne_sw = pinned_ne | pinned_sw;
        let pinned_nw_se = pinned_nw | pinned_se;
        let pinned_diagonal = pinned_ne_sw | pinned_nw_se;
        let pinned_horizontal = pinned_east | pinned_west;
        let pinned_vertical = pinned_north | pinned_south;
        let pinned = pinned_diagonal | pinned_horizontal | pinned_vertical;

        let allowed = match attackers.count() {
            0 => Bitboard::all(),
            1 => {
                let mut mask = Bitboard::empty();

                mask |= kbb.knights() & self.board.get_knights() & self.board.get_them();

                for ray in [ray_n, ray_s, ray_e, ray_w, ray_nw, ray_ne, ray_sw, ray_se] {
                    if (ray & attackers).is_occupied() {
                        mask |= ray;
                    }
                }

                mask
            }
            _ => Bitboard::empty(),
        };

        // Single pawn moves
        for to in (self.board.get_pawns() & us & !pinned_diagonal & !pinned_horizontal).north()
            & self.board.get_empty()
            & allowed
        {
            let from = Square::from_coords(to.get_file().0 as i32, to.get_rank().0 as i32 - 1);

            if try_pawn_move(from, to, &mut func) {
                return;
            }
        }

        // Pawn captures NE
        for from in self.board.get_pawns()
            & !pinned_horizontal
            & !pinned_vertical
            & !pinned_nw_se
            & self.board.get_us()
            & self.board.get_them().south().west()
            & allowed.south().west()
        {
            let to =
                Square::from_coords(from.get_file().0 as i32 + 1, from.get_rank().0 as i32 + 1);

            if try_pawn_move(from, to, &mut func) {
                return;
            }
        }

        // Pawn captures NW
        for from in self.board.get_pawns()
            & !pinned_horizontal
            & !pinned_vertical
            & !pinned_ne_sw
            & self.board.get_us()
            & self.board.get_them().south().east()
            & allowed.south().east()
        {
            let to =
                Square::from_coords(from.get_file().0 as i32 - 1, from.get_rank().0 as i32 + 1);

            if try_pawn_move(from, to, &mut func) {
                return;
            }
        }

        // Double pawn moves
        for from in self.board.get_pawns()
            & self.board.get_us()
            & self.board.get_empty().south()
            & self.board.get_empty().south().south()
            & Bitboard::from_rank(Rank(1))
            & !pinned_diagonal
            & !pinned_horizontal
            & allowed.south().south()
        {
            let to = Square::from_coords(from.get_file().0 as i32, from.get_rank().0 as i32 + 2);
            if func(Self::MoveType {
                from,
                to,
                promo: None,
            }) {
                return;
            }
        }

        if let Some(ep) = self.board.ep {
            let bb = Bitboard::from_square(ep);

            if attackers.is_empty() || (attackers == bb.south()) || (allowed & bb).is_occupied() {
                // En passant NW
                if (bb.se()
                    & self.board.get_us()
                    & self.board.get_pawns()
                    & !pinned_horizontal
                    & !pinned_vertical
                    & !pinned_ne_sw)
                    .is_occupied()
                {
                    let occupied_after = self.board.get_occupied() ^ bb ^ bb.south() ^ bb.se();
                    let bad = self.board.get_rq() & self.board.get_them();
                    let is_pinned = ksq.get_rank() == Rank(4)
                        && ((kbb.ray_east(occupied_after) & bad).is_occupied()
                            || (kbb.ray_west(occupied_after) & bad).is_occupied());

                    if !is_pinned
                        && func(Self::MoveType {
                            from: bb.se().lsb(),
                            to: ep,
                            promo: None,
                        })
                    {
                        return;
                    }
                }

                // En passant NE
                if (bb.sw()
                    & self.board.get_us()
                    & self.board.get_pawns()
                    & !pinned_horizontal
                    & !pinned_vertical
                    & !pinned_nw_se)
                    .is_occupied()
                {
                    let occupied_after = self.board.get_occupied() ^ bb ^ bb.south() ^ bb.sw();
                    let bad = self.board.get_rq() & self.board.get_them();
                    let is_pinned = ksq.get_rank() == Rank(4)
                        && ((kbb.ray_east(occupied_after) & bad).is_occupied()
                            || (kbb.ray_west(occupied_after) & bad).is_occupied());

                    if !is_pinned
                        && func(Self::MoveType {
                            from: bb.sw().lsb(),
                            to: ep,
                            promo: None,
                        })
                    {
                        return;
                    }
                }
            }
        }

        // Knight moves
        for from in self.board.get_knights()
            & self.board.get_us()
            & !pinned_diagonal
            & !pinned_horizontal
            & !pinned_vertical
        {
            let bb = Bitboard::from_square(from);
            let moves = bb.knights();
            for to in moves & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }

        // Bishop moves
        for from in self.board.get_bishops() & self.board.get_us() & !pinned {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_diagonal(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }
        for from in self.board.get_bishops() & self.board.get_us() & pinned_ne_sw {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_ne_sw(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }
        for from in self.board.get_bishops() & self.board.get_us() & pinned_nw_se {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_nw_se(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }

        // Rook moves
        for from in self.board.get_rooks() & self.board.get_us() & !pinned {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_hor_ver(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }
        for from in self.board.get_rooks() & self.board.get_us() & pinned_vertical {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_ver(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }
        for from in self.board.get_rooks() & self.board.get_us() & pinned_horizontal {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_hor(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }

        // Queen moves - bishop
        for from in self.board.get_queens() & self.board.get_us() & !pinned {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_diagonal(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }
        for from in self.board.get_queens() & self.board.get_us() & pinned_ne_sw {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_ne_sw(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }
        for from in self.board.get_queens() & self.board.get_us() & pinned_nw_se {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_nw_se(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }

        // Queen moves - rook
        for from in self.board.get_queens() & self.board.get_us() & !pinned {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_hor_ver(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }
        for from in self.board.get_queens() & self.board.get_us() & pinned_vertical {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_ver(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }
        for from in self.board.get_queens() & self.board.get_us() & pinned_horizontal {
            let bb = Bitboard::from_square(from);
            let rays = bb.rays_hor(self.board.get_occupied());

            for to in rays & !self.board.get_us() & allowed {
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }

        // King moves
        for from in self.board.get_kings() & self.board.get_us() {
            let bb_from = Bitboard::from_square(from);
            let bb_other = self.board.get_occupied() ^ bb_from;

            for to in bb_from.adjacent() & !self.board.get_us() {
                let bb_to = Bitboard::from_square(to);
                let pawn_rays = bb_to.ne() | bb_to.nw();
                let knight_rays = bb_to.knights();
                let bishop_rays = bb_to.rays_diagonal(bb_other);
                let rook_rays = bb_to.rays_hor_ver(bb_other);
                let king_rays = bb_to.adjacent();
                let gives_check = (pawn_rays & self.board.get_them() & self.board.get_pawns())
                    .is_occupied()
                    | (knight_rays & self.board.get_them() & self.board.get_knights())
                        .is_occupied()
                    | (bishop_rays & self.board.get_them() & self.board.get_bq()).is_occupied()
                    | (rook_rays & self.board.get_them() & self.board.get_rq()).is_occupied()
                    | (king_rays & self.board.get_them() & self.board.get_kings()).is_occupied();

                if gives_check {
                    continue;
                }
                if func(Self::MoveType {
                    from,
                    to,
                    promo: None,
                }) {
                    return;
                }
            }
        }

        // Castling - King side
        if self.can_castle(Castling::WKS)
            && self.board.is_empty(Square::from_coords(5, 0))
            && self.board.is_empty(Square::from_coords(6, 0))
            && !in_check
            && !self.board.is_attacked(Square::from_coords(5, 0))
            && !self.board.is_attacked(Square::from_coords(6, 0))
            && func(Self::MoveType {
                from: Square::from_coords(4, 0),
                to: Square::from_coords(6, 0),
                promo: None,
            })
        {
            return;
        }

        // Castling - Queen side
        #[allow(clippy::needless_return)]
        if self.can_castle(Castling::WQS)
            && self.board.is_empty(Square::from_coords(1, 0))
            && self.board.is_empty(Square::from_coords(2, 0))
            && self.board.is_empty(Square::from_coords(3, 0))
            && !in_check
            && !self.board.is_attacked(Square::from_coords(3, 0))
            && !self.board.is_attacked(Square::from_coords(2, 0))
            && func(Self::MoveType {
                from: Square::from_coords(4, 0),
                to: Square::from_coords(2, 0),
                promo: None,
            })
        {
            return;
        }
    }

    fn get_turn(&self) -> Side {
        match self.board.flipped {
            true => Side::Player2,
            false => Side::Player1,
        }
    }

    fn makemove(&mut self, mv: &Self::MoveType) {
        debug_assert!(mv.from != mv.to);
        debug_assert!(mv.promo != Some(ChessPiece::Pawn));
        debug_assert!(mv.promo != Some(ChessPiece::King));

        let piece = self.board.piece_on_sq(mv.from);
        let captured = self.board.piece_on_sq(mv.to);
        debug_assert!(piece.is_some());
        debug_assert!(captured != Some(ChessPiece::King));

        self.stack.push(Snapshot {
            board: self.board.clone(),
            halfmoves: self.halfmoves,
            fullmoves: self.fullmoves,
            us_ksc: self.us_ksc,
            us_qsc: self.us_qsc,
            them_ksc: self.them_ksc,
            them_qsc: self.them_qsc,
            hash: self.hash(),
        });

        self.board.ep = None;
        self.halfmoves += 1;

        // Remove piece
        if let Some(p) = piece {
            self.board.remove(mv.from, p, Side::Player1);

            if p == ChessPiece::Pawn {
                self.halfmoves = 0;
            }
        }

        // Remove captured
        if let Some(p) = captured {
            self.board.remove(mv.to, p, Side::Player2);
            self.halfmoves = 0;
        }

        // Add piece
        if let Some(p) = piece {
            self.board.set(mv.to, p, Side::Player1);
        }

        debug_assert_eq!(self.board.piece_on_sq(mv.from), None);
        debug_assert_ne!(self.board.piece_on_sq(mv.to), None);

        // En passant
        if piece == Some(ChessPiece::Pawn)
            && mv.from.get_file() != mv.to.get_file()
            && captured.is_none()
        {
            let sq = Square::from_coords(mv.to.get_file().0 as i32, 4);
            debug_assert_eq!(self.board.piece_on_sq(sq), Some(ChessPiece::Pawn));
            self.board.remove(sq, ChessPiece::Pawn, Side::Player2);
            debug_assert_eq!(self.board.piece_on_sq(sq), None);
        }

        // Double pawn move
        if piece == Some(ChessPiece::Pawn)
            && mv.from.get_rank() == Rank(1)
            && mv.to.get_rank() == Rank(3)
        {
            self.board.ep = Some(Square::<8, 8>::from_coords(mv.from.get_file().0 as i32, 2));
        }

        // Promotions
        if let Some(p) = mv.promo {
            // Remove pawn
            self.board.remove(mv.to, ChessPiece::Pawn, Side::Player1);

            // Add piece
            self.board.set(mv.to, p, Side::Player1);
        }

        // Castling - kingside
        if piece == Some(ChessPiece::King)
            && mv.from == Square::from_coords(4, 0)
            && mv.to == Square::from_coords(6, 0)
        {
            self.board
                .remove(Square::from_coords(7, 0), ChessPiece::Rook, Side::Player1);
            self.board
                .set(Square::from_coords(5, 0), ChessPiece::Rook, Side::Player1);
        }

        // Castling - queenside
        if piece == Some(ChessPiece::King)
            && mv.from == Square::from_coords(4, 0)
            && mv.to == Square::from_coords(2, 0)
        {
            self.board
                .remove(Square::from_coords(0, 0), ChessPiece::Rook, Side::Player1);
            self.board
                .set(Square::from_coords(3, 0), ChessPiece::Rook, Side::Player1);
        }

        // Castling permissions
        if mv.from == Square::from_coords(4, 0) {
            self.us_ksc = false;
            self.us_qsc = false;
        }
        if mv.from == Square::from_coords(0, 0) {
            self.us_qsc = false;
        }
        if mv.from == Square::from_coords(7, 0) {
            self.us_ksc = false;
        }
        if mv.to == Square::from_coords(0, 7) {
            self.them_qsc = false;
        }
        if mv.to == Square::from_coords(7, 7) {
            self.them_ksc = false;
        }

        self.flip();
        self.fullmoves += (!self.board.flipped) as i32;
    }

    fn undomove(&mut self, _mv: &Self::MoveType) {
        debug_assert!(!self.stack.is_empty());

        let snapshot = self.stack.pop().unwrap();
        self.board = snapshot.board;
        self.halfmoves = snapshot.halfmoves;
        self.fullmoves = snapshot.fullmoves;
        self.us_ksc = snapshot.us_ksc;
        self.us_qsc = snapshot.us_qsc;
        self.them_ksc = snapshot.them_ksc;
        self.them_qsc = snapshot.them_qsc;
    }

    fn makenull(&mut self) {
        debug_assert!(!self.in_check(), "null move from a position in check");

        self.stack.push(Snapshot {
            board: self.board.clone(),
            halfmoves: self.halfmoves,
            fullmoves: self.fullmoves,
            us_ksc: self.us_ksc,
            us_qsc: self.us_qsc,
            them_ksc: self.them_ksc,
            them_qsc: self.them_qsc,
            hash: self.hash(),
        });

        self.board.ep = None;
        self.halfmoves = 0;

        self.flip();
        self.fullmoves += (!self.board.flipped) as i32;
    }

    fn undonull(&mut self) {
        debug_assert!(!self.stack.is_empty());

        let snapshot = self.stack.pop().unwrap();
        self.board = snapshot.board;
        self.halfmoves = snapshot.halfmoves;
        self.fullmoves = snapshot.fullmoves;
        self.us_ksc = snapshot.us_ksc;
        self.us_qsc = snapshot.us_qsc;
        self.them_ksc = snapshot.them_ksc;
        self.them_qsc = snapshot.them_qsc;
    }

    fn is_valid(&self) -> Result<(), &'static str> {
        let board = &self.board;

        if (board.get_us() & board.get_them()).is_occupied() {
            return Err("a square is occupied by both sides");
        }

        let all_pieces = [
            board.get_pawns(),
            board.get_knights(),
            board.get_bishops(),
            board.get_rooks(),
            board.get_queens(),
            board.get_kings(),
        ];
        let mut union = Bitboard::empty();
        for (i, bb) in all_pieces.iter().enumerate() {
            if (union & *bb).is_occupied() {
                return Err("two piece types on one square");
            }
            union |= *bb;
            debug_assert!(i < 6);
        }
        if union != board.get_occupied() {
            return Err("piece and colour bitboards disagree");
        }

        if (board.get_kings() & board.get_us()).count() != 1 {
            return Err("side to move does not have exactly one king");
        }
        if (board.get_kings() & board.get_them()).count() != 1 {
            return Err("waiting side does not have exactly one king");
        }

        let back_ranks = Bitboard::from_rank(Rank(0)) | Bitboard::from_rank(Rank(7));
        if (board.get_pawns() & back_ranks).is_occupied() {
            return Err("pawn on the first or last rank");
        }

        let mut mirrored = self.clone();
        mirrored.flip();
        if mirrored.in_check() {
            return Err("the side not to move is in check");
        }

        if let Some(ep) = board.ep {
            if ep.get_rank() != Rank(5) {
                return Err("en passant square is not on the sixth rank");
            }
            if board.get_occupied().is_square_set(ep) {
                return Err("en passant square is occupied");
            }
            let pushed = Square::from_coords(ep.get_file().0 as i32, 4);
            if !(board.get_pawns() & board.get_them()).is_square_set(pushed) {
                return Err("no pawn to capture en passant");
            }
        }

        let us_king_home = board.get_kings().is_square_set(Square::from_coords(4, 0));
        let them_king_home = board.get_kings().is_square_set(Square::from_coords(4, 7));
        let rooks = board.get_rooks();
        if (self.us_ksc || self.us_qsc) && !us_king_home {
            return Err("castling rights without the king at home");
        }
        if (self.them_ksc || self.them_qsc) && !them_king_home {
            return Err("castling rights without the king at home");
        }
        if self.us_ksc && !rooks.is_square_set(Square::from_coords(7, 0)) {
            return Err("kingside castling right without the rook");
        }
        if self.us_qsc && !rooks.is_square_set(Square::from_coords(0, 0)) {
            return Err("queenside castling right without the rook");
        }
        if self.them_ksc && !rooks.is_square_set(Square::from_coords(7, 7)) {
            return Err("kingside castling right without the rook");
        }
        if self.them_qsc && !rooks.is_square_set(Square::from_coords(0, 7)) {
            return Err("queenside castling right without the rook");
        }

        if self.halfmoves < 0 {
            return Err("negative halfmove clock");
        }
        if self.fullmoves < 1 {
            return Err("fullmove counter below one");
        }

        Ok(())
    }

    fn get_result(&self) -> Option<GameResult> {
        let in_check = self.in_check();
        let them = !self.get_turn();

        if !self.can_move() {
            // Checkmate
            if in_check {
                return Some(GameResult::Win(them));
            }

            // Stalemate
            return Some(GameResult::Draw);
        }

        // 50 move rule
        if self.halfmoves >= 100 {
            return Some(GameResult::Draw);
        }

        // Threefold
        let hash = self.hash();
        let window = (self.halfmoves as usize).min(self.stack.len());
        if window >= 8
            && self
                .stack
                .iter()
                .rev()
                .take(window)
                .skip(1)
                .step_by(2)
                .filter(|s| s.hash == hash)
                .count()
                >= 2
        {
            return Some(GameResult::Draw);
        }

        None
    }

    fn get_fen(&self) -> String {
        let wksc = if self.board.flipped {
            self.them_ksc
        } else {
            self.us_ksc
        };
        let wqsc = if self.board.flipped {
            self.them_qsc
        } else {
            self.us_qsc
        };
        let bksc = if self.board.flipped {
            self.us_ksc
        } else {
            self.them_ksc
        };
        let bqsc = if self.board.flipped {
            self.us_qsc
        } else {
            self.them_qsc
        };

        format!(
            "{} {} {}{}{}{}{} {} {} {}",
            self.get_board_fen(),
            match self.get_turn() {
                Side::Player1 => "w",
                Side::Player2 => "b",
            },
            if wksc { "K" } else { "" },
            if wqsc { "Q" } else { "" },
            if bksc { "k" } else { "" },
            if bqsc { "q" } else { "" },
            if !(self.us_ksc | self.us_qsc | self.them_ksc | self.them_qsc) {
                "-"
            } else {
                ""
            },
            if let Some(sq) = self.board.ep {
                sq.to_string()
            } else {
                "-".to_string()
            },
            self.halfmoves,
            self.fullmoves
        )
    }

    fn parse_fen_part(&mut self, idx: usize, part: &str) {
        match idx {
            0 => {}
            1 => match part {
                "w" => self.board.flipped = false,
                "b" => self.board.flipped = true,
                _ => panic!("Uh oh {}", part),
            },
            // castling
            2 => {
                self.us_ksc = part.contains("K");
                self.us_qsc = part.contains("Q");
                self.them_ksc = part.contains("k");
                self.them_qsc = part.contains("q");
            }
            // en passant
            3 => self.board.ep = Square::from_string(part).ok(),
            // Halfmoves
            4 => self.halfmoves = part.parse::<i32>().unwrap(),
            // Fullmoves
            5 => {
                self.fullmoves = part.parse::<i32>().unwrap();
                if self.board.flipped {
                    self.flip();
                    self.board.flipped = true;
                }
            }
            _ => panic!("Invalid fen part index"),
        }
    }

    fn parse_move(&self, movestr: &str) -> Result<Self::MoveType, &'static str> {
        self.parse_movestr(movestr)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nboard = if self.flipped {
            let mut nboard = self.clone();
            nboard.swap();
            nboard
        } else {
            self.clone()
        };

        for y in (0..8).rev() {
            for x in 0..8 {
                match (nboard.side_on(x, y), nboard.piece_on(x, y)) {
                    (Some(side), Some(piece)) => write!(f, "{}", piece.to_char(side))?,
                    (_, _) => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl fmt::Display for ChessPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.board)?;

        match self.get_turn() {
            Side::Player1 => writeln!(f, "Turn: w")?,
            Side::Player2 => writeln!(f, "Turn: b")?,
        }

        write!(f, "Castling: ")?;
        if self.us_ksc {
            write!(f, "K")?;
        }
        if self.us_qsc {
            write!(f, "Q")?;
        }
        if self.them_ksc {
            write!(f, "k")?;
        }
        if self.them_qsc {
            write!(f, "q")?;
        }
        writeln!(f)?;

        writeln!(f, "Halfmoves: {}", self.halfmoves)?;
        writeln!(f, "Fullmoves: {}", self.fullmoves)?;

        Ok(())
    }
}
