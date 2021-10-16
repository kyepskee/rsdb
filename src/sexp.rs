use quote::{ToTokens, quote};
use proc_macro2::TokenStream;

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    Num(i64),
    Str(String),
    Bool(bool),
    Sym(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    List(Box<Vec<Expr>>),
    Atom(Atom),
}

trait ToExpr {
    fn to_expr(&self) -> Expr;
}

trait FromExpr {
    fn from_expr(expr: Expr) -> Option<Self> where Self: Sized;
}

impl ToTokens for Atom {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let x = match self {
            Atom::Num(x) => quote!(Atom::Num(#x)),
            Atom::Str(s) => quote!(Atom::Str(#s.to_string())),
            Atom::Bool(b) => quote!(Atom::Bool(#b)),
            Atom::Sym(s) => quote!(Atom::Sym(#s.to_string()))
        };
        
        tokens.extend(TokenStream::from(x))
    }
}

impl ToTokens for Expr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let x = match self {
            Expr::Atom(a) => quote!(Expr::Atom(#a)),
            Expr::List(box v) => quote!{
                Expr::List(Box::new(vec![#(#v),*]))
            }
        };
        
        tokens.extend(TokenStream::from(x))
    }
}

impl ToExpr for Expr {
    fn to_expr(&self) -> Expr {
        (*self).clone()
    }
}

impl FromExpr for Expr {
    fn from_expr(expr: Expr) -> Option<Expr> {
        Some(expr)
    }
}

impl<T: ToExpr> ToExpr for Option<T>
{
    fn to_expr(&self) -> Expr {
        match self {
            Some(x) => Expr::List(box vec![Expr::Atom(Atom::Sym(String::from("Some"))), 
                ToExpr::to_expr(x)]),
            None => Expr::List(box vec![Expr::Atom(Atom::Sym(String::from("None")))])
        }
    }
}

impl<T: FromExpr> FromExpr for Option<T> {
    fn from_expr(expr: Expr) -> Option<Self> {
        let l = expr.list()?;
        if l[0] == Expr::Atom(Atom::Sym(String::from("Some"))) {
            let x: Option<Option<T>> = Some(FromExpr::from_expr(l[1].clone()));
            x
        } else {
            None
        }
    }
}

impl Expr {
    pub fn atom(&self) -> Option<Atom> {
        if let Expr::Atom(x) = self {
            Some((*x).clone())
        } else {
            None
        }
    }
    
    pub fn list(&self) -> Option<Box<Vec<Expr>>> {
        if let Expr::List(x) = self {
            Some((*x).clone())
        } else {
            None
        }
    }
}

impl Atom {
    pub fn str(&self) -> Option<String> {
        if let Atom::Str(x) = self {
            Some((*x).clone())
        } else {
            None
        }
    }
}
