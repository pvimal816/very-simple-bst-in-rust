struct BST{
    val: i32,
    left: Option<Box<BST>>,
    right: Option<Box<BST>>
}

impl BST{
    fn new(key: i32) -> Box<BST>{
        let x = BST{val: key, left: None, right:None};
        return Box::new(x);
    }

    fn insert(&mut self, key: i32){
        if self.val>key{
            match &mut self.left{
                None => {
                    self.left = Some(BST::new(key));
                },
                Some(x) => x.insert(key),
            }
        }else{
            match &mut self.right{
                None => {
                    self.right = Some(BST::new(key));
                },
                Some(x) => x.insert(key),
            }
        }
    }

    fn print_inorder(&self){
        match &self.left{
            Some(x) => x.print_inorder(),
            _ => (),
        }
        print!("{}, ", self.val);
        match &self.right{
            Some(x) => x.print_inorder(),
            _ => (),
        }
    }
}

fn main() {
    let mut t = BST::new(0);
    t.insert(1);
    t.insert(2);
    t.insert(3);
    t.insert(-1);
    t.print_inorder();
}
