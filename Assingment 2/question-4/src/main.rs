#[derive(Debug)]
struct Children {
    name: String,
}

pub trait education {
    fn Is_educated(&self) -> bool;
}

pub trait bilingual {
    fn can_speak_two_lang(&self) -> bool;
}

impl education for Children {
    fn Is_educated(&self) -> bool {
        true
    }
}

impl bilingual for Children {
    fn can_speak_two_lang(&self) -> bool {
        true
    }
}
fn adopt<T: education + bilingual>(item: T) {
    println!(
        "child can speak 2 language {:?} and have primary education {:#?} \n",
        item.can_speak_two_lang(),
        item.Is_educated()
    );
    println!("ok Mr.Allah Bux i am adopting this child");
}
fn main() {
    println!("\n \n Hey I am Asim , \n I want to Adopt 2 childs who have primary education \n and are not bilingual \n");

    println!("Hello Mr.Asim , \n I am Alllah Bux \n Owner of this orphan   \n let me show you this 2 childs they have the qualities you want... \n");

    let child1 = Children {
        name: String::from("child 1"),
    };

    adopt(child1);
}
