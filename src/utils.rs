
// valenta, mlejva, mTvare, gaetgu
const MODS:[u64; 4] = [752286662544982024, 213651890746032128, 591641526615146498, 553242897760256030];
const NON_ASSIGNABLE_ROLE:[u64; 8] = [
    840515997437788171,
    839400684746309652,
    839398113411858483,
    839203458531196998,
    836998296291639327,
    803270102564864101,
    787118958012661790,
    834533620927561759
];

pub fn is_user_mod(uid:&u64) -> bool{
        uid == &MODS[0] || uid == &MODS[1] || uid == &MODS[2] || uid == &MODS[3]
}

pub fn is_assignable_role(rid:&u64) -> bool{
            !(rid == &NON_ASSIGNABLE_ROLE[0] || rid == &NON_ASSIGNABLE_ROLE[1] || rid == &NON_ASSIGNABLE_ROLE[2] ||
            rid == &NON_ASSIGNABLE_ROLE[3] || rid == &NON_ASSIGNABLE_ROLE[4] || rid == &NON_ASSIGNABLE_ROLE[5] ||
            rid == &NON_ASSIGNABLE_ROLE[6] || rid == &NON_ASSIGNABLE_ROLE[7] )
}

pub fn tags(tag:&str) -> Option<&str>{
  return match tag {
    "roadmap" => Some("https://trello.com/b/NnOWzu29/devbook-roadmap"),
    "invite" => Some("https://discord.gg/ypuZfadw8H"),
    _ => None,
  };
}
