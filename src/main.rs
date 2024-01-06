use syn::{parse_file, visit::Visit, ItemFn, Block, Stmt, ExprUnsafe};


struct RustCodeVisitor;

impl<'ast> Visit<'ast> for RustCodeVisitor {
    fn visit_item_fn(&mut self, i: &'ast ItemFn) {
        println!("Function found: {:?}", i.sig.ident);
        syn::visit::visit_item_fn(self, i);
    }

    fn visit_block(&mut self, block: &'ast Block) {
        for stmt in &block.stmts {
            if let Stmt::Expr(expr) | Stmt::Semi(expr, _) = stmt {
                if let syn::Expr::Unsafe(ExprUnsafe { block: _unsafe_block, .. }) = expr {
                    println!("Unsafe block found in the function.");
                }
            }
        }
        syn::visit::visit_block(self, block);
    }

    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        println!("Struct found: {:?}", i.ident);
    }

    fn visit_item_trait(&mut self, i: &'ast syn::ItemTrait) {
        println!("Trait found: {:?}", i.ident);
    }

    // TODO: more visit_
}

fn main() {
    println!("\n\n");
    let src = std::fs::read_to_string("src/lib.rs").expect("Could not read the file");
    let ast = parse_file(&src).expect("Unable to parse file");
    
    let mut visitor = RustCodeVisitor;
    visitor.visit_file(&ast);
}
