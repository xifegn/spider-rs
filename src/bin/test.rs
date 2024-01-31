fn main() {
    let str = "\"https://p3-pc-sign.douyinpic.com/tos-cn-i-0813/oIEe6U37QIeJBwJBHJMALCPG4c2AAEJAHQeoAZ~tplv-dy-water-v2:5oqW6Z-z5Y-377yaWGlhb2JhaVlhMDAw:888:1920.webp?biz_tag=aweme_images&from=3213915784&s=PackSourceEnum_AWEME_DETAIL&sc=image&se=false&sig=_F8_ApbK3SaynhS5735-muF4sXQ%3D&x-expires=1709272800&x-signature=3gEj5Zbpg%2BY%2F3SFc6cC9GYM%2Bt7s%3D\"";
    println!("{:?}", str);
    let name = str.replace("\"", "");
    println!("{:?}",name);
    let mut filename = name.split('=').last().unwrap().to_string();
    filename.push_str(".png");
    println!("{}", filename);
}