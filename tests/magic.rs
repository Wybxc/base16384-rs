use base16384::Base16384;

#[test]
fn magic_1() {
    const MAGIC: &str = "一帠娐匆係亐瘬娍冃缁剈愔卅潱湤栛唇忡誀漢囉偒暜瘩墊胂芸細婌焳廔萷導憣竰謾巐刔圍剅徑芄猩奌慓狵佅恓挕捥歡杚擗叕蝽湡暘葆掙畨桚璶羵籯樜攧寑荶毞喗短詽涟蘈吊冄潡癸瀦墋焣曨豂徒狥坙桞暙璦蟉葺涠癨砺悖璧砪梪粲箮秬夛壎芵箭見瓪覼絯秼儇僃缱橬洣埊胳嫜褿廑芴譍敛旘葶箽腷泟蘸氮嶓珦蟺岞禯竭覻贏嗋致譽絿燧裻贿淯言㴄";
    let data = MAGIC.encode_utf16().collect::<Vec<_>>();
    let decoded = Base16384::decode(&data).unwrap();
    assert_eq!(decoded.len(), 256);
    assert_eq!(decoded, (0..=255).collect::<Vec<_>>());
}
