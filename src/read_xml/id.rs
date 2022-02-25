pub fn exec(list: &Vec<xml::attribute::OwnedAttribute>) -> Option<String>
{
    for idx in 0..(list.len())
    {
        if list[idx].name.local_name == "identifier"
        {
            let obj: String = list[idx].value.clone();
            return Some(obj);
        }
    }

    None
}
