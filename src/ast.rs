use swc_common::AstNode;
use swc_ecma_ast::{Expr, JSXElement, JSXElementChild, ModuleItem, Stmt};
// use swc_ecma_visit::{
//     noop_visit_mut_type, noop_visit_type, Visit, VisitMut, VisitMutWith, VisitWith, Fold, FoldWith, Spanned
// };

// trait CommentProcessor {
//     fn process_comment(&self, jsx: JSXElement) { /* process comment, doesn't matter */
//     }
// }

// impl<T> Fold<T> for dyn CommentProcessor
// where
//     T: FoldWith<Self> + Spanned,
// {
//     fn fold(&mut self, n: T) -> T {
//         let span = n.span();
//         n.fold_children(self)
//     }
// }
