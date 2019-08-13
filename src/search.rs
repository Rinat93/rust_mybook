pub mod Trees {
    /// Принцип двоичного дерева
    #[derive(Debug)]
    enum BinaryTree<T> {
        Empty,
        NonEmpty(Box<TreeNode<T>>)
    }

    /// element - родитель, left and right дети
    #[derive(Debug)]
    struct TreeNode<T> {
        element: T,
        left: BinaryTree<T>,
        right: BinaryTree<T>
    }

    impl<T: Ord> BinaryTree<T> {
        fn add(&mut self, value: T) {

            /// Наименьший элемент уходить в лево, наибольший в право
            match *self {
                BinaryTree::Empty => 
                    *self = BinaryTree::NonEmpty(
                        Box::new(TreeNode {
                            element: value,
                            left: BinaryTree::Empty,
                            right: BinaryTree::Empty
                        })
                    ),
                BinaryTree::NonEmpty(ref mut node) =>
                    if value <= node.element {
                        node.left.add(value)
                    } else {
                        node.right.add(value)
                    }
            }
        }
    }

    pub fn bst_tree() {
        let mut tree = BinaryTree::Empty;
        tree.add("Mercury");
        println!("{:?}",&tree);
        tree.add("Venus");
        println!("{:?}",&tree);
        tree.add("Ser");
        println!("{:?}",&tree);
        tree.add("Arc");
        println!("{:?}",&tree);
        tree.add("Brc");
        println!("{:?}",&tree);
        tree.add("12");
        println!("{:?}",&tree);
        // for i in &tree.NonEmpty {
        //     println!("{}",i.element);
        // }
    }

}