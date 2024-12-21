// int
// *int
// [char]
// int, int -> int
//
//
//

use std::collections::HashMap;

use chs_util::{chs_error, CHSError};

use crate::nodes::{Expression, Literal, Module, Var};

type CHSTypeId = usize;
type CHSTypeLevel = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CHSType {
    Const(String),
    App(Box<CHSType>, Vec<CHSType>),
    Arrow(Vec<CHSType>, Box<CHSType>),
    Var(Box<CHSTypeVar>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CHSTypeVar {
    Unbound(CHSTypeId, CHSTypeLevel),
    Link(CHSType),
    Generic(CHSTypeId),
}

impl CHSType {
    pub fn new_var(id: CHSTypeId, level: CHSTypeLevel) -> CHSType {
        let id = next_id(id);
        CHSType::Var(Box::new(CHSTypeVar::Unbound(id, level)))
    }
    pub fn new_gen_var(id: CHSTypeId) -> CHSType {
        let id = next_id(id);
        CHSType::Var(Box::new(CHSTypeVar::Generic(id)))
    }
}

pub fn next_id(id: CHSTypeId) -> CHSTypeId {
    id + 1
}

pub const fn reset_id() -> CHSTypeId {
    0
}

pub fn generalize(ty: CHSType, level: CHSTypeLevel) -> CHSType {
    match ty {
        CHSType::Var(ref b) => match **b {
            CHSTypeVar::Unbound(id, other_level) if other_level > level => CHSType::new_gen_var(id),

            CHSTypeVar::Link(ref chstype) => generalize(chstype.clone(), level),
            _ => ty,
        },
        CHSType::App(chstype, chstype_list) => CHSType::App(
            generalize(*chstype, level).into(),
            chstype_list
                .into_iter()
                .map(|c| generalize(c, level))
                .collect(),
        ),
        CHSType::Arrow(chstype_list, chstype) => CHSType::Arrow(
            chstype_list
                .into_iter()
                .map(|c| generalize(c, level))
                .collect(),
            generalize(*chstype, level).into(),
        ),
        CHSType::Const(_) => ty,
    }
}

pub fn instantiate(ty: CHSType, level: CHSTypeLevel) -> CHSType {
    use CHSType::*;
    let mut id_var: HashMap<usize, CHSType> = HashMap::new();
    match ty {
        Const(_) => ty,
        Var(ref b) => match **b {
            CHSTypeVar::Link(ref chstype) => instantiate(chstype.clone(), level),
            CHSTypeVar::Unbound(_, _) => ty,
            CHSTypeVar::Generic(id) => {
                if let Some(var) = id_var.get(&id) {
                    var.clone()
                } else {
                    let var = CHSType::new_var(id, level);
                    id_var.insert(id, var.clone());
                    var
                }
            },
        },
        App(ty, ty_arg_list) =>
            App(instantiate(*ty, level).into(), ty_arg_list.into_iter().map(|t| instantiate(t, level)).collect()),
        Arrow(param_ty_list, return_ty) =>
            Arrow(param_ty_list.into_iter().map(|t| instantiate(t, level)).collect(), instantiate(*return_ty, level).into())
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

pub fn infer(
    m: &mut Module,
    expr: &Expression,
    level: CHSTypeLevel,
) -> Result<CHSType, CHSError> {
    match expr {
        Expression::Literal(literal) => match literal {
            Literal::IntegerLiteral { .. } => return Ok(CHSType::Const("int".into())),
            Literal::BooleanLiteral { .. } => return Ok(CHSType::Const("bool".into())),
        },
        Expression::VarDecl(v) => {
            if let Some(v) = unsafe { v.as_ref() } {
                let var_ty = infer(m, &v.value, level + 1)?;
                let generalized_ty = generalize(var_ty, level);
                let k = v.name.clone();
                m.env.insert(k, generalized_ty);
                return Ok(CHSType::Const("()".into()));
            } else {
                chs_error!("infer VarDecl")
            }
        }
        Expression::Var(Var { name, loc: _ }) => {
            if let Some(ty) = m.env.get(name) {
                Ok(instantiate(ty.clone(), level))
            } else {
                chs_error!("variable {} not found", name)
            }
        }
    }
}

/*
    | Var name -> begin
            try
                instantiate level (Env.lookup env name)
            with Not_found -> error ("variable " ^ name ^ " not found")
        end
    | Fun(param_list, body_expr) ->
            let param_ty_list = List.map (fun _ -> new_var level) param_list in
            let fn_env = List.fold_left2
                (fun env param_name param_ty -> Env.extend env param_name param_ty)
                env param_list param_ty_list
            in
            let return_ty = infer fn_env level body_expr in
            TArrow(param_ty_list, return_ty)
    | Let(var_name, value_expr, body_expr) ->
            let var_ty = infer env (level + 1) value_expr in
            let generalized_ty = generalize level var_ty in
            infer (Env.extend env var_name generalized_ty) level body_expr
    | Call(fn_expr, arg_list) ->
            let param_ty_list, return_ty =
                match_fun_ty (List.length arg_list) (infer env level fn_expr)
            in
            List.iter2
                (fun param_ty arg_expr -> unify param_ty (infer env level arg_expr))
                param_ty_list arg_list
            ;
            return_ty
*/


#[cfg(test)]
mod tests {
    use chs_util::Loc;

    use crate::nodes::VarDecl;

    use super::*;

    #[test]
    fn test_name() {
        let mut m = Module::default();
        m.push_var_decl(VarDecl {
            name: "x".into(),
            value: Expression::Literal(Literal::IntegerLiteral {
                loc: Loc::default(),
                value: 10,
            }),
            loc: Loc::default(),
            ttype: CHSType::Const("int".into())
        });
        let res = infer(
            &mut m,
            &Expression::Literal(Literal::IntegerLiteral {
                loc: Loc::default(),
                value: 10,
            }),
            1,
        );
        assert!(res.is_ok())
    }
}
