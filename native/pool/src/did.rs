
use rustler::{Env, Term, NifResult, Encoder};
use utils::atoms;

pub fn create_did<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {

    Ok((atoms::ok(), "").encode(env))
}

