// int
// *int
// [char]
// int, int -> int
//
//
//

use std::{collections::HashMap, sync::LazyLock};

use chs_util::{chs_error, CHSError};

use crate::nodes::{Expression, Literal, Var};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CHSTypeId(usize);
impl CHSTypeId {
    pub fn next_id(&mut self) -> CHSTypeId {
        self.0 += 1;
        *self
    }

    pub fn reset_id(&mut self) {
        self.0 = 0;
    }
}

pub type CHSTypeLevel = isize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CHSType {
    Const(String),
    App(Box<CHSType>, Vec<CHSType>),
    Arrow(Vec<CHSType>, Box<CHSType>),
    Var(CHSTypeVar),
}

pub static CHSINT: LazyLock<CHSType>  = LazyLock::new(|| CHSType::Const("int".to_string()));
pub static CHSBOOL: LazyLock<CHSType> = LazyLock::new(|| CHSType::Const("bool".to_string()));
pub static CHSCHAR: LazyLock<CHSType> = LazyLock::new(|| CHSType::Const("char".to_string()));
pub static CHSVOID: LazyLock<CHSType> = LazyLock::new(|| CHSType::Const("void".to_string()));
pub static CHSSTRING: LazyLock<CHSType> = LazyLock::new(|| CHSType::Const("string".to_string()));

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CHSTypeVar {
    Unbound(CHSTypeId, CHSTypeLevel),
    Link(Box<CHSType>),
    Generic(CHSTypeId),
}

impl CHSType {
    pub fn new_var(id: &mut CHSTypeId, level: CHSTypeLevel) -> CHSType {
        let id = id.next_id();
        CHSType::Var(CHSTypeVar::Unbound(id, level))
    }
    pub fn new_gen_var(id: &mut CHSTypeId) -> CHSType {
        let id = id.next_id();
        CHSType::Var(CHSTypeVar::Generic(id))
    }
    pub fn is_var_unbound(&self) -> bool {
        if let Self::Var(CHSTypeVar::Unbound(_, _)) = self {
            true
        } else {
            false
        }
    }
    pub fn is_void(&self) -> bool {
        if let Self::Const(name) = self {
            name == "void"
        } else {
            false
        }
    }
}

fn generalize(ty: CHSType, level: CHSTypeLevel) -> CHSType {
    match ty {
        CHSType::Var(CHSTypeVar::Unbound(mut id, other_level)) if other_level > level => {
            CHSType::new_gen_var(&mut id)
        }
        CHSType::App(chstype, chstype_list) => CHSType::App(
            generalize(*chstype, level).into(),
            chstype_list
                .into_iter()
                .map(|c| generalize(c, level))
                .collect(),
        )
        .into(),
        CHSType::Arrow(chstype_list, chstype) => CHSType::Arrow(
            chstype_list
                .into_iter()
                .map(|c| generalize(c, level))
                .collect(),
            generalize(*chstype, level).into(),
        ),
        CHSType::Var(CHSTypeVar::Link(chstype)) => generalize(*chstype, level),
        CHSType::Var(CHSTypeVar::Generic(_))
        | CHSType::Var(CHSTypeVar::Unbound(_, _))
        | CHSType::Const(_) => ty,
    }
}

/*
let rec generalize level = function
    | TVar {contents = Unbound(id, other_level)} when other_level > level ->
            TVar (ref (Generic id))
    | TApp(ty, ty_arg_list) ->
            TApp(generalize level ty, List.map (generalize level) ty_arg_list)
    | TArrow(param_ty_list, return_ty) ->
            TArrow(List.map (generalize level) param_ty_list, generalize level return_ty)
    | TVar {contents = Link ty} -> generalize level ty
    | TVar {contents = Generic _} | TVar {contents = Unbound _} | TConst _ as ty -> ty
*/

fn instantiate(
    m: &mut InferEnv,
    id_var: &mut HashMap<usize, CHSType>,
    ty: CHSType,
    level: CHSTypeLevel,
) -> CHSType {
    use CHSType::*;
    match ty {
        Const(_) => return ty,
        Var(CHSTypeVar::Link(ty)) => instantiate(m, id_var, *ty, level),
        Var(CHSTypeVar::Generic(id)) => {
            if let Some(var) = id_var.get(&id.0) {
                var.clone()
            } else {
                let var = CHSType::new_var(&mut m.id, level);
                id_var.insert(m.id.0, var.clone());
                var
            }
        }
        Var(CHSTypeVar::Unbound(_, _)) => ty,
        App(ty, ty_arg_list) => App(
            instantiate(m, id_var, *ty, level).into(),
            ty_arg_list
                .into_iter()
                .map(|t| instantiate(m, id_var, t, level))
                .collect(),
        ),
        Arrow(param_ty_list, return_ty) => Arrow(
            param_ty_list
                .into_iter()
                .map(|t| instantiate(m, id_var, t, level))
                .collect(),
            instantiate(m, id_var, *return_ty, level).into(),
        ),
    }
}

/*
let instantiate level ty =
    let id_var_map = Hashtbl.create 10 in
    let rec f ty = match ty with
        | TConst _ -> ty
        | TVar {contents = Link ty} -> f ty
        | TVar {contents = Generic id} -> begin
                try
                    Hashtbl.find id_var_map id
                with Not_found ->
                    let var = new_var level in
                    Hashtbl.add id_var_map id var ;
                    var
            end
        | TVar {contents = Unbound _} -> ty
        | TApp(ty, ty_arg_list) ->
                TApp(f ty, List.map f ty_arg_list)
        | TArrow(param_ty_list, return_ty) ->
                TArrow(List.map f param_ty_list, f return_ty)
    in
    f ty
*/

fn match_fun_ty(
    m: &mut InferEnv,
    num_params: usize,
    ty: &mut CHSType,
) -> Result<(Vec<CHSType>, CHSType), CHSError> {
    use CHSType::*;
    match ty {
        Arrow(param_ty_list, return_ty) => {
            if param_ty_list.len() != num_params {
                chs_error!("unexpected number of arguments");
            } else {
                Ok((param_ty_list.to_vec(), *return_ty.clone()))
            }
        }
        Var(CHSTypeVar::Link(ty)) => match_fun_ty(m, num_params, ty),
        Var(CHSTypeVar::Unbound(_, level)) => {
            let param_ty_list: Vec<CHSType> = (0..num_params)
                .map(|_| CHSType::new_var(&mut m.id, *level))
                .collect();
            let return_ty = CHSType::new_var(&mut m.id, *level);
            *ty = CHSType::Var(CHSTypeVar::Link(
                Arrow(param_ty_list.clone(), return_ty.clone().into()).into(),
            ));
            Ok((param_ty_list, return_ty.into()))
        }
        _ => chs_error!("Expect a function"),
    }
}

/*
let rec match_fun_ty num_params = function
    | TArrow(param_ty_list, return_ty) ->
            if List.length param_ty_list <> num_params then
                error "unexpected number of arguments"
            else
                param_ty_list, return_ty
    | TVar {contents = Link ty} -> match_fun_ty num_params ty
    | TVar ({contents = Unbound(id, level)} as tvar) ->
            let param_ty_list =
                let rec f = function
                    | 0 -> []
                    | n -> new_var level :: f (n - 1)
                in
                f num_params
            in
            let return_ty = new_var level in
            tvar := Link (TArrow(param_ty_list, return_ty)) ;
            param_ty_list, return_ty
    | _ -> error "expected a function"
*/

fn unify(ty1: CHSType, ty2: CHSType) -> Result<CHSType, CHSError> {
    use CHSType::*;
    if ty1 == ty2 {
        return Ok(ty1);
    }
    match (ty1, ty2) {
        (Const(n1), Const(n2)) if n1 == n2 => Ok(CHSType::Const(n1)),
        (App(ty1, ty_arg_list1), App(ty2, ty_arg_list2)) => {
            let ty1 = unify(*ty1, *ty2)?;
            let mut ty_arg_list = vec![];
            for (ty1, ty2) in ty_arg_list1.into_iter().zip(ty_arg_list2.into_iter()) {
                ty_arg_list.push(unify(ty1, ty2)?);
            }
            Ok(App(ty1.into(), ty_arg_list))
        }
        (Arrow(param_list1, ret_ty1), Arrow(param_list2, ret_ty2)) => {
            let mut param_list = vec![];
            for (ty1, ty2) in param_list1.into_iter().zip(param_list2.into_iter()) {
                param_list.push(unify(ty1, ty2)?);
            }
            let ty1 = unify(*ret_ty1, *ret_ty2)?;
            Ok(Arrow(param_list, ty1.into()))
        }
        (Var(CHSTypeVar::Link(ty1)), ty2) => unify(*ty1, ty2),
        (ty1, Var(CHSTypeVar::Link(ty2))) => unify(ty1, *ty2),
        (Var(CHSTypeVar::Unbound(id1, _)), Var(CHSTypeVar::Unbound(id2, _))) if id1 == id2 => {
            chs_error!("There is only a single instance of a particular type variable.")
        }
        (tvar, mut ty) if tvar.is_var_unbound() => {
            if let Var(CHSTypeVar::Unbound(mut id, mut level)) = tvar {
                occurs_check_adjust_levels(&mut id, &mut level, &mut ty)?;
            }
            Ok(Var(CHSTypeVar::Link(ty.clone().into())))
        }
        (mut ty, tvar) if tvar.is_var_unbound() => {
            if let Var(CHSTypeVar::Unbound(mut id, mut level)) = tvar {
                occurs_check_adjust_levels(&mut id, &mut level, &mut ty)?;
            }
            Ok(Var(CHSTypeVar::Link(ty.clone().into())))
        }
        (ty1, ty2) => chs_error!("cannot unify types {:?} and {:?}", ty1, ty2),
    }
}

/*
let rec unify ty1 ty2 =
    if ty1 == ty2 then () else
    match (ty1, ty2) with
        | TConst name1, TConst name2 when name1 = name2 -> ()
        | TApp(ty1, ty_arg_list1), TApp(ty2, ty_arg_list2) ->
                unify ty1 ty2 ;
                List.iter2 unify ty_arg_list1 ty_arg_list2
        | TArrow(param_ty_list1, return_ty1), TArrow(param_ty_list2, return_ty2) ->
                List.iter2 unify param_ty_list1 param_ty_list2 ;
                unify return_ty1 return_ty2
        | TVar {contents = Link ty1}, ty2 | ty1, TVar {contents = Link ty2} -> unify ty1 ty2
        | TVar {contents = Unbound(id1, _)}, TVar {contents = Unbound(id2, _)} when id1 = id2 ->
                assert false (* There is only a single instance of a particular type variable. *)
        | TVar ({contents = Unbound(id, level)} as tvar), ty
        | ty, TVar ({contents = Unbound(id, level)} as tvar) ->
                occurs_check_adjust_levels id level ty ;
                tvar := Link ty
        | _, _ -> error ("cannot unify types " ^ string_of_ty ty1 ^ " and " ^ string_of_ty ty2)
*/

fn occurs_check_adjust_levels(
    id: &mut CHSTypeId,
    level: &mut CHSTypeLevel,
    ty: &mut CHSType,
) -> Result<(), CHSError> {
    match ty {
        CHSType::Var(CHSTypeVar::Link(ty)) => occurs_check_adjust_levels(id, level, ty),
        CHSType::Var(CHSTypeVar::Generic(_)) => chs_error!("TODO: occurs_check_adjust_levels"),
        CHSType::Var(CHSTypeVar::Unbound(other_id, other_level)) => {
            if other_id == id {
                chs_error!("recursive types")
            } else if other_level > level {
                *ty = CHSType::Var(CHSTypeVar::Unbound(*other_id, *level));
            }
            Ok(())
        }
        CHSType::App(ty, ty_arg_list) => {
            occurs_check_adjust_levels(id, level, ty)?;
            for ty_arg in ty_arg_list.iter_mut() {
                occurs_check_adjust_levels(id, level, ty_arg)?;
            }
            Ok(())
        }
        CHSType::Arrow(param_ty_list, return_ty) => {
            for param_ty in param_ty_list.iter_mut() {
                occurs_check_adjust_levels(id, level, param_ty)?;
            }
            occurs_check_adjust_levels(id, level, return_ty)?;
            Ok(())
        }
        CHSType::Const(_) => Ok(()),
    }
}

/*
let occurs_check_adjust_levels tvar_id tvar_level ty =
    let rec f = function
        | TVar {contents = Link ty} -> f ty
        | TVar {contents = Generic _} -> assert false
        | TVar ({contents = Unbound(other_id, other_level)} as other_tvar) ->
                if other_id = tvar_id then
                    error "recursive types"
                else
                    if other_level > tvar_level then
                        other_tvar := Unbound(other_id, tvar_level)
                    else
                        ()
        | TApp(ty, ty_arg_list) ->
                f ty ;
                List.iter f ty_arg_list
        | TArrow(param_ty_list, return_ty) ->
                List.iter f param_ty_list ;
                f return_ty
        | TConst _ -> ()
    in
    f ty
*/

#[derive(Debug, Default, Clone)]
pub struct InferEnv {
    pub env: HashMap<String, CHSType>,
    pub id: CHSTypeId,
}

pub fn infer(m: &mut InferEnv, expr: &Expression, level: CHSTypeLevel) -> Result<CHSType, CHSError> {
    match expr {
        Expression::Literal(literal) => match literal {
            Literal::IntegerLiteral { .. }  => return Ok(CHSINT.clone()),
            Literal::BooleanLiteral { .. }  => return Ok(CHSBOOL.clone()),
            Literal::StringLiteral  { .. }  => return Ok(CHSType::App(
                CHSType::Const("pointer".to_string()).into(),
                vec![CHSCHAR.clone()],
            )),
        },
        Expression::VarDecl(v) => {
            let var_ty = infer(m, &v.value, level + 1)?;
            let generalized_ty = generalize(var_ty, level);
            m.env.insert(v.name.clone(), generalized_ty);
            return Ok(CHSVOID.clone());
        }
        Expression::FnDecl(fd) => {
            let mut fn_env = m.clone();
            fn_env.env.extend(fd.args.clone());
            fn_env.env.insert(fd.name.clone(), CHSType::Arrow(
                fd.args.clone().into_iter().map(|(_, t)| t).collect(),
                fd.ret_type.clone().into(),
            ));
            unify(fd.ret_type.clone(), infer(&mut fn_env, &fd.body, level)?)?;
            m.env.insert(fd.name.clone(), fn_env.env.get(&fd.name).cloned().unwrap());
            return Ok(CHSVOID.clone());
        }
        Expression::Var(Var { name, loc: _ }) => {
            if let Some(ty) = m.env.get(name) {
                let mut id_var = HashMap::new();
                Ok(instantiate(m, &mut id_var, ty.clone(), level))
            } else {
                chs_error!("variable {} not found", name)
            }
        }
        Expression::Call(c) => {
            let mut ty = infer(m, &c.caller, level)?;
            let (mut param_ty_list, return_ty) = match_fun_ty(m, c.args.len(), &mut ty)?;
            for (param_ty, arg_expr) in param_ty_list.iter_mut().zip(c.args.iter()) {
                let tvar = infer(m, arg_expr, level)?;
                *param_ty = unify(param_ty.clone(), tvar)?;
            }
            Ok(return_ty)
        }
        Expression::Ref(expr) => {
            let ty = infer(m, &expr, level)?;
            Ok(CHSType::App(
                CHSType::Const("pointer".to_string()).into(),
                vec![ty],
            ))
        }
        Expression::Deref(expr) => {
            let ty = infer(m, &expr, level)?;
            if let CHSType::App(p, e) = ty {
                if *p == CHSType::Const("pointer".to_string()) && e.len() == 1 {
                    return Ok(e[0].clone());
                }
            }
            chs_error!("Expect pointer")
        }
        Expression::Print(expr) => {
            let ty = infer(m, &expr, level)?;
            if *CHSSTRING != ty {
                chs_error!("Expect string")
            }
            Ok(CHSType::Const("()".to_string()))
        }
    }
}
